use serde::{Deserialize, Serialize};

use crate::structs::block::block_info::StateBlock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRequestRpc {
    pub action: String, // "process"
    pub json_block: String, // "true"
    pub subtype: String, // "send", "receive", "change"
    pub block: StateBlock,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_work: Option<bool>,
}

