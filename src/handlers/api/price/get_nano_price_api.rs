use axum::{response::IntoResponse, Json};
use serde_json::json;

use crate::services::price::price_service;
// 0000 mag dit weg ?!
pub async fn get_nano_price_api() -> impl IntoResponse {
    match price_service::get_nano_price().await {
        Ok(price) => {
            Json(json!({
                "usd": price.usd,
                "eur": price.eur,
                "btc": price.btc,
                "last_updated": price.last_updated.to_rfc3339(),
            })).into_response()
        },
        Err(err) => {
            tracing::error!("Failed to fetch Nano price: {}", err);
            
            Json(json!({
                "error": "price_fetch_failed",
                "message": "Failed to fetch current Nano price"
            })).into_response()
        }
    }
}
