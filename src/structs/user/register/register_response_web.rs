use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterResponseWeb {
    pub success: bool,
    pub message: String,
}