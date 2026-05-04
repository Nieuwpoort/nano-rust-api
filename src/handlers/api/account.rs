use axum::{Json, extract::{Path}};
use serde_json::json;

use crate::{r#enum::api::{error::{account::AccountError, database::DatabaseError, rpc::RpcError}}, helpers::{api_helper::{api_success, api_success_no_data, encrypted_api_key_exists, generate_api_key}, helper::raw_to_nano}, services::{postgres::postgres::get_pool, 
rpc::rpc::{account_balance_rpc, account_exists_rpc, account_info_rpc, active_difficulty_rpc, block_info_rpc, create_wallet_rpc, 
    get_rpc_response, receivable_rpc, wallet_private_key_index_to_nano_address_rpc}}, structs::{account::{balance::balance_response_api::BalanceResponseApi, info::account_info_response_rpc::AccountInfoResponseApi, 
wallet::wallet_data::WalletData}, api::api::{ApiErrorResult, ApiSuccessResult}, api_key::api_key_data::ApiKeyData, block::{block_info_response_rpc::BlockInfoResponseRpc, 
block_pending_response_rpc::BlocksPerAccountResponse, publish_block_request_api::PublishBlockRequestApi}, cache::api_cache::API_CACHE}};

pub async fn get_balance_api(Path(address): Path<String>) -> Result<ApiSuccessResult<BalanceResponseApi>, ApiErrorResult> {
    if !address.starts_with("nano_") && !address.starts_with("xrb_") {
        return Err(AccountError::InvalidAddress.to_response());
    }

    let balance = account_balance_rpc(&address).await?;
    let balance_nano = raw_to_nano(&balance.balance);
    let pending_nano = raw_to_nano(&balance.pending);
    Ok(api_success(BalanceResponseApi {
        account: address,
        balance: balance_nano,
        balance_raw: balance.balance,
        pending: pending_nano,
        pending_raw: balance.pending,
    }))
}

pub async fn create_wallet_api() -> Result<ApiSuccessResult<WalletData>, ApiErrorResult> {
    let generated_api_key = create_api_key_api().await?;
    let private_wallet_seed = create_wallet_rpc().await?;
    let active_index = 0;
    let address_data = wallet_private_key_index_to_nano_address_rpc(&private_wallet_seed.wallet, active_index).await?;
    let wallet_date = WalletData{
        address: address_data.account.clone(),
        public_key: address_data.public.to_uppercase(),
        private_key: address_data.private.to_uppercase(),
        wallet_private_seed: private_wallet_seed.wallet.to_uppercase(),
        api_key: generated_api_key,
    };

    Ok(api_success(wallet_date))
}

pub async fn pending_api(Path(address): Path<String>) -> Result<ApiSuccessResult<BlocksPerAccountResponse>, ApiErrorResult> {
    if !address.starts_with("nano_") && !address.starts_with("xrb_") {
        return Err(AccountError::InvalidAddress.to_response());
    }

    let receivable = receivable_rpc(&address).await?;
    Ok(api_success(receivable))
}


pub async fn block_info_api(Path(block_hash): Path<String>) -> Result<ApiSuccessResult<BlockInfoResponseRpc>, ApiErrorResult> {
    let block_info = block_info_rpc(&block_hash).await?;

    Ok(api_success(block_info))
}

pub async fn publish_block_api(Json(payload): Json<PublishBlockRequestApi>) -> Result<ApiSuccessResult<()>, ApiErrorResult> {
    let subtype = payload.subtype.as_str();
    let block = payload.block;

    let request = json!({
        "action": "process",
        "json_block": "true",
        "subtype": subtype,
        "block": block
    });

    let response = get_rpc_response(request, "Failed to send process request").await?;

    if !response.status().is_success() {
        return Err(RpcError::ResponseError("Failed to send process request".to_string()).to_response());
    }
    let response_text = response.text().await.map_err(|_| {
        RpcError::ReadError.to_response()
    })?;
    let response_json: serde_json::Value = serde_json::from_str(&response_text).map_err(|_| {
        RpcError::ParseError("Failed to parse RPC response".to_string()).to_response()
    })?;
    if let Some(error) = response_json.get("error") {
        return Err(RpcError::ResponseError(format!("Block rejected: {}", error)).to_response());
    }
    
    if let Some(_hash) = response_json.get("hash") {
        Ok(api_success_no_data())
    } else {
        Err(RpcError::ResponseError("No hash returned from node".to_string()).to_response())
    }
}

pub async fn account_info_api(Path(address): Path<String>) -> Result<ApiSuccessResult<AccountInfoResponseApi>, ApiErrorResult> {
    if !address.starts_with("nano_") && !address.starts_with("xrb_") {
        return Err(AccountError::InvalidAddress.to_response());
    }

    account_exists_rpc(&address).await?;

    let account_info = account_info_rpc(&address).await?;
    let difficulty = active_difficulty_rpc().await?;

    Ok(api_success(AccountInfoResponseApi {
        frontier: account_info.frontier,
        balance: account_info.balance,
        representative: account_info.representative,
        difficulty_send: difficulty.network_current,
        difficulty_receive: difficulty.network_receive_current,
    }))
}

async fn create_api_key_api() -> Result<String, ApiErrorResult> {
    let db = get_pool();
    let (mut api_key, mut encrypted_api_key) = generate_api_key();

    while encrypted_api_key_exists(encrypted_api_key.as_str()).await.unwrap_or(false) {
        (api_key, encrypted_api_key) = generate_api_key();
    }

    sqlx::query!(
        r#"
        INSERT INTO api_keys (api_key, credits)
        VALUES ($1, $2)
        RETURNING id
        "#,
        encrypted_api_key,
        0,
    )
    .fetch_one(db)
    .await
    .map_err(|_| DatabaseError::StoreError.to_response())?;

    let api_key_id = sqlx::query!(
        r#"
        SELECT id FROM api_keys WHERE api_key = $1
        "#,
        encrypted_api_key,
    )
    .fetch_one(db)
    .await
    .map_err(|_| DatabaseError::SelectError.to_response())?;

    let api_cache = API_CACHE.get().unwrap();
    
    api_cache.api_key_data.insert(encrypted_api_key.clone(), ApiKeyData {
        id: api_key_id.id,
        api_key: api_key.clone(),
        user_id: None,
        name: None,
        nano_address: None,
        is_active: true,
        credits: 0,
    });
    
    Ok(api_key)
}