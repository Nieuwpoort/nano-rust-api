use axum::{Json};
use base64::Engine;
use rand::{RngCore, rngs::OsRng};

use crate::{r#enum::api::error::auth::AuthError, services::{encrypt::encrypt::{decrypt, encrypt}, postgres::postgres::get_pool}, structs::{api::api::{ApiErrorResult, ApiResponse}, cache::{counter_cache::COUNTER_CACHE, env::ENV_CACHE}}};


pub fn api_success<T>(data: T) -> Json<ApiResponse<T>> {
    Json(ApiResponse {
        success: true,
        data: Some(data),
        error: None,
    })
}

pub fn api_success_no_data() -> Json<ApiResponse<()>> {
    Json(ApiResponse {
        success: true,
        data: None,
        error: None,
    })
}

pub fn generate_api_key() -> (String, String) {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);

    let api_key_str = base64::engine::general_purpose::STANDARD.encode(bytes);
    let env = ENV_CACHE.get().unwrap();
    let encrypted = encrypt(&api_key_str, env.api_encrypt_key.clone(), env.api_encrypt_iv.clone());

    (api_key_str, (base64::engine::general_purpose::STANDARD.encode(encrypted)))
}


pub fn encrypt_api_key(api_key: &str) -> String {
    let env = ENV_CACHE.get().unwrap();
    let encrypted_api_key = encrypt(api_key, env.api_encrypt_key.clone(), env.api_encrypt_iv.clone());
    base64::engine::general_purpose::STANDARD.encode(encrypted_api_key)
}

pub fn decrypt_api_key(encrypted_api_key: &str) -> String {
    let env = ENV_CACHE.get().unwrap();
    let decrypted_api_key = decrypt(&encrypted_api_key.to_string(), env.api_encrypt_key.clone(), env.api_encrypt_iv.clone());
    
    decrypted_api_key.unwrap()
}

pub fn add_to_transaction_counters() {
    let counter_cache = COUNTER_CACHE.get().unwrap();

    counter_cache.counters.get_mut("tph").map(|mut v| *v += 1);
    counter_cache.counters.get_mut("tps").map(|mut v| *v += 1);
    counter_cache.counters.get_mut("total_transactions").map(|mut v| *v += 1);
    counter_cache.peak_counters.get_mut("tps").map(|mut v| *v += 1);
}


pub async fn encrypted_api_key_exists(encrypted_api_key: &str) -> Result<bool, sqlx::Error> {
    let db = get_pool();
    let record: Option<(i64,)> = sqlx::query_as(
        r#"
        SELECT id
        FROM api_keys
        WHERE api_key = $1
        "#,
    )
    .bind(encrypted_api_key)
    .fetch_optional(db)
    .await?;

    Ok(record.is_some())
}

pub fn is_valid_api_key(api_key: &str) -> Result<(), ApiErrorResult> {
    if api_key.trim().is_empty() {
        return Err(AuthError::MissingApiKey.to_response());
    }
    if api_key.len() != 44 {
        return Err(AuthError::InvalidApiKey.to_response());
    }
    Ok(())
}