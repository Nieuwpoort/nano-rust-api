use axum::response::{Html, IntoResponse};

pub async fn privacy_policy() -> impl IntoResponse {
    let html = include_str!("../../../static/privacy.html");
    Html(html)
}
