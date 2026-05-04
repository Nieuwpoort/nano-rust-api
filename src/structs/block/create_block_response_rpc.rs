use serde::{Deserialize, Serialize};
use super::block_info::StateBlock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBlockResponseRpc {
    pub hash: String,
    pub difficulty: String,
    pub block: StateBlock,
}
