use uuid::Uuid;

use crate::{r#enum::api::error::{database::DatabaseError, transaction::TransactionError}, services::postgres::postgres::get_pool, structs::{api::api::{ApiErrorResult}, cache::{api_cache::API_CACHE, transaction_cache_data::TransactionCacheData}}};

pub async fn create_transaction_id() -> String {
    let api_cache = API_CACHE.get().unwrap();
    let mut transaction_id = Uuid::new_v4().to_string();
    loop {
        if !api_cache.transactions_to_process.contains_key(&transaction_id) {
            let db_record_exists = match sqlx::query!(
                "SELECT transaction_id FROM paid_transactions WHERE transaction_id = $1",
                transaction_id
            )
            .fetch_optional(get_pool())
            .await {
                Ok(result) => result.is_some(),
                Err(e) => {
                    eprintln!("Database query failed: {}", e);
                    false
                }
            };
    
            if !db_record_exists {
                break;
            }
        }
        transaction_id = Uuid::new_v4().to_string();
    }
    transaction_id
}

pub async fn db_create_paid_transaction(cached_transaction: &TransactionCacheData, transaction_id: String, api_key_id: i64) -> Result<(), ApiErrorResult> {
    let db = get_pool();

    let _ = sqlx::query!(
        "INSERT INTO paid_transactions (api_key_id, transaction_id, receive_address, amount, redirect_url, created_at) VALUES ($1, $2, $3, $4, $5, NOW())",
        api_key_id,
        transaction_id,
        cached_transaction.receive_address,
        cached_transaction.amount,
        cached_transaction.redirect_url,
    ).execute(db).await.map_err(|_| {
        DatabaseError::StoreError.to_response()
    })?;

    Ok(())
}

pub fn transaction_to_process_exists(transaction_id: &str) -> Result<(), ApiErrorResult> {
    let api_cache = API_CACHE.get().unwrap();
    
    if !api_cache.transactions_to_process.contains_key(transaction_id) {
        return Err(TransactionError::InvalidTransactionId.to_response());
    }

    Ok(())
}



