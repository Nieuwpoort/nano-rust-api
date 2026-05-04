use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserApiUsageHistoryDb {
    pub id: i64,
    pub user_id: i64,
    pub api_key_id: i64,
    pub api_calls: i64,
    pub month: i32,
    pub year: i32,
    pub costs: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
