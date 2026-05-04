use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrepareSendBlockResponseApi {
    pub previous: String,
    pub representative: String,
    pub new_balance: String,
    pub destination_public_key: String,
    pub work: String,
}