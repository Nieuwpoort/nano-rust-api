use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportWalletApi {
    pub password: String,
    pub password_confirmation: String,
    pub private_wallet_seed: String,
}