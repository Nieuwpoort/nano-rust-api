use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendResponseApi {
    pub success: bool,
    pub send_to: String,
    pub ammount: String,
}