use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrepareReceiveBlockResponseApi {
    pub previous: String,
    pub representative: String,
    pub new_balance: String,
    pub pending_block_hash: String,
    pub work: String,
}