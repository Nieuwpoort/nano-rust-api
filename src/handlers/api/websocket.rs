use axum::{Extension, extract::{Path, WebSocketUpgrade}, response::Response};

use crate::{services::websocket::web_socket::setup_websocket_connection, structs::api_key::api_key::ParseApiKeyHeader};

pub async fn websocket_handler(
    Extension(api_key_header): Extension<ParseApiKeyHeader>,
        ws: WebSocketUpgrade,
    Path(transaction_id): Path<String>,
) -> Response {
    ws.on_upgrade(move |socket| setup_websocket_connection(socket, transaction_id, api_key_header))
}