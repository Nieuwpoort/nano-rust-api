use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebsocketMessage {
    pub event: String,
    pub data: WebsocketTransactionData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebsocketTransactionData {
    pub transaction_id: String,
    pub status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebsocketResponse {
    pub success: bool,
    pub message: String,
}