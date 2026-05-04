use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransactionResponseApi{
    pub receive_address: String,
    pub amount: String,
    pub transaction_id: String,
}