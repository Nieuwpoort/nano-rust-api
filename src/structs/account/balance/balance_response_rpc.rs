use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceResponseRpc {
    pub balance: String,
    pub pending: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receivable: Option<String>,
}