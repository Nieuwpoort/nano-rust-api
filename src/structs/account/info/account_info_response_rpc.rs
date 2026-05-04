use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfoResponseRpc {
    pub frontier: String,
    pub open_block: String,
    pub representative_block: String,
    pub balance: String,
    pub modified_timestamp: String,
    pub block_count: String,
    pub account_version: String,
    pub confirmation_height: String,
    pub confirmation_height_frontier: String,
    pub representative: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfoResponseApi {
    pub frontier: String,
    pub balance: String,
    pub representative: String,
    pub difficulty_send: String,
    pub difficulty_receive: String,
}
