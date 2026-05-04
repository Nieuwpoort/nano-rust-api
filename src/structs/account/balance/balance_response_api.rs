use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceResponseApi {
    pub account: String,
    pub balance: String, // In Nano
    pub balance_raw: String,
    pub pending: String, // In Nano
    pub pending_raw: String,
}