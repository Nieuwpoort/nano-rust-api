use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct NanoNodeWebsocketResponse {
    pub account: String,
    pub amount: String,
    pub block: NanoNodeWebsocketBlock,
    pub confirmation_type: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NanoNodeWebsocketBlock {
    pub account: String,
    pub balance: String,
    pub link: String,
    pub link_as_account: String,
    pub previous: String,
    pub representative: String,
    pub signature: String,
    pub subtype: String,
    #[serde(rename = "type")]
    pub block_type: String,
    pub work: String,
}
