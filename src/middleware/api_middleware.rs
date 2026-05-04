use std::{net::SocketAddr, sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use axum::{extract::{ConnectInfo, Request}, middleware::Next, response::Response};
use base64::{Engine};
use reqwest::Method;
use tokio::spawn;

use crate::{r#enum::api::error::auth::AuthError, middleware::api_log_middleware::db_create_api_log, services::{encrypt::encrypt::encrypt, postgres::postgres::get_pool}, structs::{api::api::ApiErrorResult, api_key::{api_key::ParseApiKeyHeader, api_key_data::ApiKeyData, rate_limiter_key_api::{RateLimiterDataApi, RateLimiterKeyApi}}, cache::{api_cache::{API_CACHE, ApiCache}, env::ENV_CACHE}}};


pub async fn protected_api_middleware(addr: ConnectInfo<SocketAddr>, mut request: Request, next: Next) -> Result<Response, ApiErrorResult> {
    let headers = request.headers().clone();
    let method = request.method().clone();
    let uri = request.uri().clone().to_string();
    let api_cache = API_CACHE.get().unwrap();
    let client_ip = if let Some(forwarded) = headers
    .get("x-forwarded-for")
    .and_then(|v| v.to_str().ok())
    .and_then(|s| s.split(',').next())
    {
        forwarded.trim().to_string()
    } else {
        addr.ip().to_string()
    };
    let api_key = headers.get("X-API-Key")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            spawn(db_create_api_log(method.clone(),uri.clone(),false, client_ip.clone()));
                AuthError::MissingApiKey.to_response()
        })?;

    request = validate_request_limiter(request, &method, &api_key, &uri, &api_cache, &client_ip).await?;
    request = validate_api_key(request, &method, &uri, &api_key, &api_cache, client_ip.clone()).await?;
    
    Ok(next.run(request).await)
}
    
pub async fn public_api_middleware(addr: ConnectInfo<SocketAddr>, mut request: Request, next: Next) -> Result<Response, ApiErrorResult> {
    let headers = request.headers().clone();
    let method = request.method().clone();
    let uri = request.uri().clone().to_string();
    let api_cache = API_CACHE.get().unwrap();
    let client_ip = if let Some(forwarded) = headers
    .get("x-forwarded-for")
    .and_then(|v| v.to_str().ok())
    .and_then(|s| s.split(',').next())
    {
        forwarded.trim().to_string()
    } else {
        addr.ip().to_string()
    };

    request = validate_request_limiter(request, &method, "", &uri, &api_cache, &client_ip).await?;
    
    Ok(next.run(request).await)
}


pub async fn validate_request_limiter(request: Request, method: &Method, api_key: &str, uri: &String, api_cache: &Arc<ApiCache>, client_ip: &String) -> Result<Request, ApiErrorResult> {
    let request_size = request.headers().get("content-length").and_then(|v| v.to_str().ok()).and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);

    if request_size > 5 * 1024 {
        spawn(db_create_api_log(method.clone(),uri.clone(),false, client_ip.clone()));
        return Err(AuthError::PayloadTooLarge.to_response());
    }

    // Rate limiting op IP address (api_key niet altijd beschikbaar in public endpoints)
    let key = RateLimiterKeyApi {
        api_key: client_ip.to_string(),
        uri: uri.to_string(),
    };

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let request_per_second = 1; 
    let max_requests = 5; // 5 requests per seconde

    let mut allow = true;

    let mut rate_limiter = api_cache.rate_limiters.entry(key).or_insert(RateLimiterDataApi {
        count: 0,
        window_start: now + request_per_second,
    });

    if now > rate_limiter.window_start {
        rate_limiter.count = 1;
        rate_limiter.window_start = now + request_per_second;
    } else if rate_limiter.count < max_requests {
        rate_limiter.count += 1;
    } else {
        allow = false;
    }

    let current_count = rate_limiter.count as i32;

    drop(rate_limiter);

    if !allow {
        let db = get_pool();
        let _ = sqlx::query!(
                r#"
                INSERT INTO rate_limiter_logs (hits, client_ip, api_key, uri, success)
                VALUES ($1, $2, $3, $4, $5)
                "#,
                current_count,
                client_ip,
                api_key,
                uri.to_string(),
                false
            )
            .execute(db)
            .await
            .unwrap();


        return Err(AuthError::RateLimitExceeded.to_response());
    }

    Ok(request)
}

async fn validate_api_key(mut request: Request, method: &Method, uri: &String, api_key: &str, api_cache: &ApiCache, ip_address: String) -> Result<Request, ApiErrorResult> {
    let env = ENV_CACHE.get().unwrap();
    let encrypted_api_key = encrypt(&api_key, env.api_encrypt_key.clone(), env.api_encrypt_iv.clone());
    let encrypted_api_key_str = base64::engine::general_purpose::STANDARD.encode(&encrypted_api_key);

    if !api_cache.api_key_data.contains_key(&encrypted_api_key_str) {
        let db = get_pool();
        let db_record = sqlx::query_as!(
            ApiKeyData,
            r#"
            SELECT id, api_key, user_id, name, nano_address, is_active, credits FROM api_keys WHERE api_key = $1 AND is_active = true
            "#,
            encrypted_api_key_str
        )
        .fetch_optional(db)
        .await
        .unwrap();

        if db_record.is_none() {
            spawn(db_create_api_log(method.clone(),uri.clone(),false, ip_address));

            return Err(AuthError::InvalidApiKey.to_response());
        }

        api_cache.api_key_data.insert(encrypted_api_key_str.clone(), db_record.unwrap());
    }
    
    request.extensions_mut().insert(ParseApiKeyHeader {
        api_key: api_key.to_string(),
        encrypted_api_key: encrypted_api_key_str,
    });

    Ok(request)
}