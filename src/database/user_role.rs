use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserRoleDb {
    pub user_id: i64,
    pub role_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
