use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonateResponseApi {
    pub success: bool,
    pub message: String,
}