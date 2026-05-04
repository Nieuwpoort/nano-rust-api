use chrono::NaiveDateTime;

#[derive(Clone, Debug)]
pub struct ApiKeyTable {
    pub id: i64,
    pub user_id: Option<i64>,
    pub name: Option<String>,
    pub nano_address: Option<String>,
    pub api_key: String,
    pub is_active: bool,
    pub credits: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}