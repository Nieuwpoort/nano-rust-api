use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveDifficultyResponseRpc {
    pub deprecated: String,
    pub network_minimum: String,
    pub network_receive_minimum: String,
    pub network_current: String,
    pub network_receive_current: String,
    pub multiplier: String,
}
