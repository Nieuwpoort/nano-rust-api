use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendRequestApi {
    pub encrypted_wallet_string: String,
    pub password: String,
    pub to_address: String,
    pub amount: String,
}

