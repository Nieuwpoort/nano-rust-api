use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusTransactionApi {
    pub status: String,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusTransactionWeb {
    pub transaction_id: String,
    pub is_paid: bool,
    pub amount: Option<String>,
    pub receive_address: Option<String>,
    pub redirect_url: Option<String>,
}