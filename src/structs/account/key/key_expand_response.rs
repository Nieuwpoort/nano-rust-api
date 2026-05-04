use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyExpandResponse {
    pub private: String,
    pub public: String,
    pub account: String,
}