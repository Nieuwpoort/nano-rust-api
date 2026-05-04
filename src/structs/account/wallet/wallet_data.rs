use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletData {
    pub address: String,
    pub public_key: String,
    pub private_key: String,
    pub wallet_private_seed: String,
    pub api_key: String,
}
