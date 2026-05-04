use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfo {
    pub block_account: String,
    pub amount: String,
    pub balance: String,
    pub height: String,
    pub local_timestamp: String,
    pub successor: String,
    pub confirmed: String,
    pub contents: StateBlock,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateBlock {
    #[serde(rename = "type")]
    pub block_type: String, // Always "state"
    pub account: String,
    pub previous: String,
    pub representative: String,
    pub balance: String,
    pub link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_as_account: Option<String>,
    pub signature: String,
    pub work: String,
}