use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiveResponseApi {
    pub success: bool,
    pub received: String,
    pub new_balance: String,
    pub received_from: String,
}