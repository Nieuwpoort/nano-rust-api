use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockWalletRequestApi {
    pub encrypted_wallet_string: String,
    pub password: String,
}