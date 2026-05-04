use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrepareReceiveBlockApi {
    pub address: String,
    pub public_key: String,
}