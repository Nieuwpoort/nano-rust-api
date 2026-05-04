use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonateRequestApi {
    pub encrypted_wallet_string: String,
    pub password: String,
    pub amount: String,
}