use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiKeyData {
    pub id: i64,
    pub api_key: String,
    pub user_id: Option<i64>,
    pub name: Option<String>,
    pub nano_address: Option<String>,
    pub is_active: bool,
    pub credits: i32,
}