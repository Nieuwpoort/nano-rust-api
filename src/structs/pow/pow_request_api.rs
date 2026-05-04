use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PowRequestApi {
    pub hash: String,
    pub difficulty: String,
}
