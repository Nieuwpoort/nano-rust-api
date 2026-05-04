use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionCacheData{
    pub encrypted_api_key: String,
    pub receive_address: String,
    pub amount: String,
    pub created_at: i64,
    pub redirect_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransactionDashMapKey{
    pub receive_address: String,
    pub amount: String,
}
