use serde::{Deserialize, Serialize};

use crate::structs::block::block_info::StateBlock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfoResponseRpc {
    pub block_account: String,
    pub amount: String,
    pub balance: String,
    pub height: String,
    pub local_timestamp: String,
    pub confirmed: String,
    pub contents: StateBlock,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
}