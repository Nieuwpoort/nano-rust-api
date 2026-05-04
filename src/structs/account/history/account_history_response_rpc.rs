use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountHistoryResponseRpc {
    pub account: String,
    pub history: Vec<HistoryItemRpc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryItemRpc {
    #[serde(rename = "type")]
    pub block_type: String,
    pub account: String,
    pub amount: String,
    pub local_timestamp: String,
    pub height: String,
    pub hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmed: Option<String>,
}