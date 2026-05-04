use std::net::SocketAddr;

use axum::{extract::{ConnectInfo, Request}, middleware::Next, response::Response};
use reqwest::{Method};
use tokio::spawn;

use crate::{services::postgres::postgres::get_pool, structs::api::api::ApiErrorResult};

pub async fn api_log_middleware(
    addr: ConnectInfo<SocketAddr>, 
    request: Request, next: Next
) -> Result<Response, ApiErrorResult> {
    let headers = request.headers().clone();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let client_ip = if let Some(forwarded) = headers
    .get("x-forwarded-for")
    .and_then(|v| v.to_str().ok())
    .and_then(|s| s.split(',').next())
    {
        forwarded.trim().to_string()
    } else {
        addr.ip().to_string()
    };

    let response = next.run(request).await;

    let success = response.status().is_success();
    
    spawn(db_create_api_log(method, uri.to_string(), success, client_ip));
    
    Ok(response)
}


pub async fn db_create_api_log(
    method: Method, 
    uri: String, 
    success: bool, 
    client_ip: String
){
    let db = get_pool();
    let _ = sqlx::query!(
            r#"
            INSERT INTO api_router_logs (client_ip, method, uri, success)
            VALUES ($1, $2, $3, $4)
            "#,
            client_ip,
            method.to_string(),
            uri.to_string(),
            success,
        )
        .execute(db)
        .await
        .unwrap();
}
