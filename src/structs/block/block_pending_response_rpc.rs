use dashmap::DashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlocksPerAccountResponse {
    pub blocks: DashMap<String, Vec<String>>,
}