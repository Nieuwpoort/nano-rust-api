use serde::{Deserialize, Serialize};

use crate::structs::block::block_info::StateBlock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishBlockRequestApi {
    pub subtype: String,
    pub block: StateBlock,
}
