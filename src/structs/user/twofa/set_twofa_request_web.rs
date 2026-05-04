use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SetTwoFaRequestWeb {
    pub password: String,
    pub twofa: String,
}