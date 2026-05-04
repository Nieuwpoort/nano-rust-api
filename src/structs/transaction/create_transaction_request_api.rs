use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransactionRequestApi{
    pub receive_address: String,
    pub amount: String,
    pub redirect_url: Option<String>,
}