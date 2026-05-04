use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeterministicKeyResponseRpc {
    pub private: String,
    pub public: String,
    pub account: String,
}