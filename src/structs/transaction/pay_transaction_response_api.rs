use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayTransactionResponseApi {
    pub success: String,
    pub transaction_id: String,
    pub to_address: String,
    pub amount: String,
}