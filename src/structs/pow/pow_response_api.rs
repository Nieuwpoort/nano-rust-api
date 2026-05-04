use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PowResponseApi {
    pub work: String,
    pub hash: String,
    pub difficulty: String,
}
