use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserApiUsageDb {
    pub id: i64,
    pub user_id: i64,
    pub api_key_id: i64,
    pub api_calls: i64,
}
