use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserDb {
    pub id: i64,
    pub full_name: String,
    pub date_of_birth: NaiveDate,
    pub username: String,
    pub email: String,
    pub phone_number: String,
    pub street_address: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub accept_terms: bool,
    pub last_login_ip: Option<String>,
    pub failed_login_attempts: i32,
    pub blocked_by_ip: Option<String>,
    pub last_login_date: Option<NaiveDateTime>,
    pub blocked_date: Option<NaiveDateTime>,
    pub twofa: Option<String>,
    pub email_verified_at: Option<NaiveDateTime>,
    pub password: String,
    pub remember_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
