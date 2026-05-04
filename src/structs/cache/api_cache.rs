use std::{collections::HashSet, sync::Arc};

use axum::extract::ws::Message;
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use tokio::sync::mpsc::Sender;

use crate::{structs::{api_key::{api_key_data::ApiKeyData, rate_limiter_key_api::{RateLimiterDataApi, RateLimiterKeyApi}}, cache::transaction_cache_data::{TransactionCacheData, TransactionDashMapKey}}};

pub static API_CACHE: OnceCell<Arc<ApiCache>> = OnceCell::new();

#[derive(Debug)]
pub struct ApiCache {
    pub api_key_data: DashMap<String, ApiKeyData>,
    pub rate_limiters: DashMap<RateLimiterKeyApi, RateLimiterDataApi>,
    pub payed_transactions: DashMap<String, ()>,

    pub transactions_to_process: DashMap<String, TransactionCacheData>,
    pub transactions_to_process_index: DashMap<TransactionDashMapKey, String>,  // (TransactionDashMapKey, transaction_id)
    pub topup_transactions_to_process: DashMap<String, u32>, // transaction_id -> credits_amount

    pub websocket_connections: DashMap<String, Sender<Message>>, // transaction_id -> sender
    pub websocket_connections_index: DashMap<usize, (Sender<Message>, HashSet<String>)>, // sender -> transaction_ids
}

impl ApiCache {
    pub fn new() -> Self {
        Self {
            api_key_data: DashMap::new(),
            rate_limiters: DashMap::new(),
            payed_transactions: DashMap::new(),

            transactions_to_process: DashMap::new(),
            transactions_to_process_index: DashMap::new(),

            topup_transactions_to_process: DashMap::new(),

            websocket_connections: DashMap::new(),
            websocket_connections_index: DashMap::new(),
        }
    }
}