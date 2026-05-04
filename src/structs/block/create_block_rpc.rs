use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBlockRpc {
    #[serde(rename = "type")]
    pub type_: String,
    pub account: String,
    pub balance: String,
    pub key: String,
    pub representative: String,
    pub link: String,
    pub previous: String,
    pub work: Option<String>,
}