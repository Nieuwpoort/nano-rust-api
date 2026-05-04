use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParseApiKeyHeader {
    pub api_key: String,
    pub encrypted_api_key: String,
}