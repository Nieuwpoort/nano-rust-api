use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayTransactionRequestApi {
    pub encrypted_wallet_string: String,
    pub password: String,
    pub amount: String,
    pub transaction_id: String,
}

