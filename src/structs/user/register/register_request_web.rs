use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequestWeb {
    pub full_name: String,
    pub date_of_birth: String,
    pub username: String,
    pub email: String,
    pub phone_number: String,
    pub password: String,
    pub street_address: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub accept_terms: bool,
}