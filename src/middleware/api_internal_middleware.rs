use std::net::SocketAddr;
use axum::{
    body::Body,
    extract::ConnectInfo,
    http::{Request},
    middleware::Next,
    response::Response,
};

use crate::{r#enum::api::error::auth::AuthError, structs::api::api::ApiErrorResult};

pub async fn internal_api_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, ApiErrorResult> {
    let ip = addr.ip();
    
    if ip.is_loopback() {
        Ok(next.run(request).await)
    } else {
        Err(AuthError::Forbidden.to_response())
    }
}
