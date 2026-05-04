use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerchantKeyToAddressRequest {
    pub api_key: String,
}