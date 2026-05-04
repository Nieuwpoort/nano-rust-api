use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ApiKeyDb {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub key: String,
    pub nano_address: String,
    pub private_key: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
