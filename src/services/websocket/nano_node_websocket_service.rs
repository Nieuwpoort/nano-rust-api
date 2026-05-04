use crate::r#enum::api::error::websocket::WebsocketError;
use crate::r#enum::nano_web_socket_enum::NanoNodeWebSocketCommand;
use crate::background_services::websocket::nano_node_websocket_bg_service::{WEBSOCKET_COMMAND_SENDER};
use crate::structs::api::api::{ApiErrorResult};

pub async fn add_address_to_nano_node_websocket_subscriptions(address: &str) -> Result<(), ApiErrorResult> {
    if let Some(sender) = WEBSOCKET_COMMAND_SENDER.get() {
        sender.send(NanoNodeWebSocketCommand::AddAccount(address.to_string())).await.map_err(|_| WebsocketError::WebSocketAddAccountError.to_response())?;

        Ok(())
    } else {
        Err(WebsocketError::WebSocketNotInitialized.to_response())
    }
}

pub async fn remove_address_from_nano_node_websocket_subscriptions(address: &str) -> Result<(), ApiErrorResult> {
    if let Some(sender) = WEBSOCKET_COMMAND_SENDER.get() {
        sender.send(NanoNodeWebSocketCommand::RemoveAccount(address.to_string())).await.map_err(|_| WebsocketError::WebSocketRemoveAccountError.to_response())?;
        Ok(())
    } else {
        Err(WebsocketError::WebSocketNotInitialized.to_response())
    }
}