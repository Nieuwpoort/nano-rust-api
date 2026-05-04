use std::{time::Duration};

use axum::{Extension, Json, extract::Path};
use tokio::time::sleep;

use crate::{r#enum::api::error::{auth::AuthError, credit::CreditError, database::DatabaseError}, handlers::api::{account::publish_block_api, transaction::{create_transaction_api, get_transaction_status_api}}, 
helpers::{api_helper::api_success, price_helper::get_all_credits_prices}, services::postgres::postgres::get_pool, structs::{api::api::{ApiErrorResult, ApiSuccessResult}, api_key::api_key::ParseApiKeyHeader, 
    block::publish_block_request_api::PublishBlockRequestApi, cache::{api_cache::API_CACHE, env::ENV_CACHE}, credits::credits::{GetCreditsResponse, 
    TopupCreditsFinalRequest, TopupCreditsResponse, TopupCreditsSuccessResponse}, transaction::create_transaction_request_api::CreateTransactionRequestApi}};

pub async fn get_credits_info_api(
    Extension(api_key_header): Extension<ParseApiKeyHeader>,
) -> Result<ApiSuccessResult<GetCreditsResponse>, ApiErrorResult> {
    let api_cache = API_CACHE.get().unwrap();
    let api_key_data  = api_cache.api_key_data.get(&api_key_header.encrypted_api_key).ok_or_else(|| {
        AuthError::InvalidApiKey.to_response()
    })?;

    let prices = get_all_credits_prices().await.map_err(|_| {
        CreditError::PriceCalculationError.to_response()
    })?;

    Ok(api_success(GetCreditsResponse {
        credits: api_key_data.credits,
        current_credits_price_10: prices.price_10,
        current_credits_price_50: prices.price_50,
        current_credits_price_100: prices.price_100,
        current_credits_price_500: prices.price_500,
        current_credits_price_1000: prices.price_1000,
        current_credits_price_5000: prices.price_5000,
        current_credits_price_10000: prices.price_10000,
        current_credits_price_50000: prices.price_50000,
        current_credits_price_100000: prices.price_100000,
    }))
}

pub async fn get_topup_payment_request_api(
    Extension(api_key_header): Extension<ParseApiKeyHeader>,
    Path(credits_amount): Path<u32>,
) -> Result<ApiSuccessResult<TopupCreditsResponse>, ApiErrorResult> {
    let api_cache = API_CACHE.get().unwrap();
    let prices = get_all_credits_prices().await.map_err(|_| {
        CreditError::PriceCalculationError.to_response()
    })?;

    let price_in_nano = match credits_amount {
        10 => prices.price_10,
        50 => prices.price_50,
        100 => prices.price_100,
        500 => prices.price_500,
        1000 => prices.price_1000,
        5000 => prices.price_5000,
        10000 => prices.price_10000,
        50000 => prices.price_50000,
        100000 => prices.price_100000,
        _ => {
            return Err(CreditError::InvalidCreditsAmount.to_response());
        }
    };
    let env= ENV_CACHE.get().unwrap();
    let topup_address = env.topup_credits_address.clone();

    let transaction_response = create_transaction_api(
        Extension(api_key_header.clone()), 
        Json(CreateTransactionRequestApi {
            receive_address: topup_address.clone(),
            amount: price_in_nano.to_string(),
            redirect_url: None,
        })
    ).await?.0.data.ok_or_else(|| CreditError::TransactionCreationFailed.to_response())?;

    
    api_cache.topup_transactions_to_process.insert(transaction_response.transaction_id.clone(), credits_amount);
    Ok(api_success(TopupCreditsResponse {
        payment_address: topup_address.clone(),
        nano_amount: transaction_response.amount.clone(),
        transaction_id: transaction_response.transaction_id.clone(),
    }))
}

pub async fn topup_credits_api(
    Extension(api_key_header): Extension<ParseApiKeyHeader>,
    Json(payload): Json<TopupCreditsFinalRequest>
) -> Result<ApiSuccessResult<TopupCreditsSuccessResponse>, ApiErrorResult> {
    let _  = publish_block_api(Json(PublishBlockRequestApi {
        subtype: payload.subtype,
        block: payload.block,
    })).await?;

    sleep(Duration::from_secs(5)).await;
    {
        let transaction_status = get_transaction_status_api(Path(payload.transaction_id.clone())).await?;

        if !transaction_status.0.success {
            return Err(CreditError::TransactionNotConfirmed.to_response());
        }
    }
  
    let api_cache = API_CACHE.get().unwrap();

    let cached_topup_data= api_cache.topup_transactions_to_process.get(&payload.transaction_id).ok_or_else(|| {
        CreditError::NoTransactionFound.to_response()
    })?;

    let db = get_pool();

    sqlx::query!(
        r#"
        UPDATE api_keys
        SET credits = credits + $1
        WHERE api_key = $2
        "#,
        cached_topup_data.value().clone() as i32,
        api_key_header.encrypted_api_key.clone(),
    )
    .execute(db)
    .await
    .map_err(|_| DatabaseError::FailedToUpdate.to_response())?;


    sqlx::query!(
        r#"
        INSERT INTO topup_api_key_credits (transaction_id, api_key, topup_amount)
        VALUES ($1, $2, $3)
        "#,
        cached_topup_data.key().clone(),
        api_key_header.encrypted_api_key.clone(),
        cached_topup_data.value().clone() as i32,
    )
    .execute(db)
    .await
    .map_err(|_| DatabaseError::StoreError.to_response())?;
    let topup_credits = cached_topup_data.value().clone();
    drop(cached_topup_data);
    api_cache.topup_transactions_to_process.remove(&payload.transaction_id);
    
    let new_credits = api_cache.api_key_data.get_mut(&api_key_header.encrypted_api_key)
    .map(|mut api_key_data| {
        api_key_data.credits += topup_credits as i32;
        api_key_data.credits  // Return nieuwe balance
    })
    .ok_or_else(|| CreditError::ApiKeyNotFound.to_response())?;


    Ok(api_success(TopupCreditsSuccessResponse {
        topped_up_credits: topup_credits,
        new_credits_balance: new_credits,
    }))
}
