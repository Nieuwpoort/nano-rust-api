use axum::{Json, extract::Path, extract::Extension};
use chrono::{Utc};

use crate::{r#enum::api::error::{auth::AuthError, transaction::TransactionError}, helpers::{api_helper::api_success, helper::{nano_to_raw, raw_to_nano}}, services::{rpc::rpc::account_exists_rpc, 
    transaction::transaction_service::create_transaction_id, 
    websocket::nano_node_websocket_service::add_address_to_nano_node_websocket_subscriptions}, structs::{api::api::{ApiErrorResult, ApiSuccessResult}, api_key::api_key::ParseApiKeyHeader, cache::{api_cache::API_CACHE, 
        transaction_cache_data::{TransactionCacheData, TransactionDashMapKey}}, transaction::{create_transaction_request_api::CreateTransactionRequestApi, create_transaction_response_api::CreateTransactionResponseApi, 
        status_transaction_api::{StatusTransactionApi, StatusTransactionWeb}}}};

pub async fn create_transaction_api(
    Extension(api_key_header): Extension<ParseApiKeyHeader>,
    Json(payload): Json<CreateTransactionRequestApi>
) -> Result<ApiSuccessResult<CreateTransactionResponseApi>, ApiErrorResult> {
    let transaction_id = create_transaction_id().await;
    let api_cache = API_CACHE.get().unwrap();

    if !api_cache.api_key_data.contains_key(&api_key_header.encrypted_api_key) {
        return Err(AuthError::InvalidApiKey.to_response());
    }

    account_exists_rpc(&payload.receive_address).await?;

    let base_raw: u128 = nano_to_raw(&payload.amount)
        .map_err(|_| TransactionError::InvalidNumberFormat.to_response())?
        .parse()
        .map_err(|_| TransactionError::InvalidNumberFormat.to_response())?;

    let uuid_parsed = uuid::Uuid::parse_str(&transaction_id).unwrap();
    let uuid_bytes = uuid_parsed.as_bytes();
    let offset = u32::from_be_bytes([uuid_bytes[12], uuid_bytes[13], uuid_bytes[14], uuid_bytes[15]]) as u128;

    let unique_raw = base_raw + offset;
    let unique_amount = raw_to_nano(&unique_raw.to_string());
    let now = Utc::now().timestamp();

    api_cache.transactions_to_process.insert(transaction_id.clone(),TransactionCacheData {
        encrypted_api_key: api_key_header.encrypted_api_key.clone(),
        receive_address: payload.receive_address.clone(),
        amount: unique_amount.clone(),
        created_at: now,
        redirect_url: payload.redirect_url.clone(),
    });
    api_cache.transactions_to_process_index.insert(TransactionDashMapKey {
        receive_address: payload.receive_address.clone(),
        amount: nano_to_raw(&unique_amount).unwrap_or_default(),
    }, transaction_id.clone().to_string());
    add_address_to_nano_node_websocket_subscriptions(&payload.receive_address).await?;

    Ok(api_success(CreateTransactionResponseApi {
        receive_address: payload.receive_address,
        amount: unique_amount,
        transaction_id: transaction_id.to_string(),
    })) 
}


pub async fn get_transaction_status_api(Path(transaction_id): Path<String>) -> Result<ApiSuccessResult<StatusTransactionApi>, ApiErrorResult> {
    let api_cache = API_CACHE.get().unwrap();

    if api_cache.payed_transactions.contains_key(&transaction_id) {
        return Ok(api_success(StatusTransactionApi {
            status: "PAID".to_string(),
            transaction_id: transaction_id,
        }));
    }

    if let Some(transaction_data) = api_cache.transactions_to_process.get(&transaction_id) {
        drop(transaction_data);
        return Ok(api_success(StatusTransactionApi {
            status: "PENDING".to_string(),
            transaction_id: transaction_id,
        }));
    } else {
        return Err(TransactionError::InvalidTransactionId.to_response());
    }
}

pub async fn get_direct_transaction_status_api(Path(transaction_id): Path<String>) -> Result<ApiSuccessResult<StatusTransactionWeb>, ApiErrorResult> {
    let api_cache = API_CACHE.get().unwrap();

    if api_cache.payed_transactions.contains_key(&transaction_id) {
        return Ok(api_success(StatusTransactionWeb {
            transaction_id: transaction_id,
            is_paid: true,
            amount: None,
            receive_address: None,
            redirect_url: None,
        }));

    } else if api_cache.transactions_to_process.contains_key(&transaction_id) {
        let transaction_data = api_cache.transactions_to_process.get(&transaction_id).unwrap();
        return Ok(api_success(StatusTransactionWeb {
            transaction_id: transaction_id,
            is_paid: false,
            amount: Some(transaction_data.amount.clone()),
            receive_address: Some(transaction_data.receive_address.clone()),
            redirect_url: transaction_data.redirect_url.clone(),
        }));
    } else {
        Err(TransactionError::InvalidTransactionId.to_response())
    }
}