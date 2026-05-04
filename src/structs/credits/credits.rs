use serde::{Deserialize, Serialize};

use crate::structs::block::block_info::StateBlock;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCreditsResponse {
    pub credits: i32,
    pub current_credits_price_10: f64,
    pub current_credits_price_50: f64,
    pub current_credits_price_100: f64,
    pub current_credits_price_500: f64,
    pub current_credits_price_1000: f64,
    pub current_credits_price_5000: f64,
    pub current_credits_price_10000: f64,
    pub current_credits_price_50000: f64,
    pub current_credits_price_100000: f64,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct TopupCreditsResponse {
    pub payment_address: String,
    pub nano_amount: String,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopupCreditsFinalRequest {
    pub subtype: String,
    pub block: StateBlock,
    pub transaction_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopupCreditsSuccessResponse {
    pub topped_up_credits: u32,
    pub new_credits_balance: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreditsPrices {
    pub price_10: f64,
    pub price_50: f64,
    pub price_100: f64,
    pub price_500: f64,
    pub price_1000: f64,
    pub price_5000: f64,
    pub price_10000: f64,
    pub price_50000: f64,
    pub price_100000: f64,
}