use chrono::Utc;
use dashmap::DashMap;
use tokio::time::{sleep, Duration};

use crate::{services::websocket::{nano_node_websocket_service::remove_address_from_nano_node_websocket_subscriptions, web_socket::remove_websocket_connection}, structs::cache::{api_cache::API_CACHE, env::ENV_CACHE, transaction_cache_data::TransactionDashMapKey}};

pub async fn transaction_cleanup_worker() {
    let api_cache = API_CACHE.get().unwrap();
    let sleep_duration = Duration::from_secs(60); // 1 minute
    let env = ENV_CACHE.get().unwrap();
    let payment_address = env.topup_credits_address.clone();

    loop {
        sleep(sleep_duration).await;
        
        let now = Utc::now().timestamp();
        let expired_transactions = DashMap::new();
        let remove_transactions_after_minutes = env.remove_transactions_after_minutes;
        
        for entry in api_cache.transactions_to_process.iter() {
            let transaction_id = entry.key().clone();
            let create_at = entry.value().created_at;
            
            if  create_at + (remove_transactions_after_minutes * 60) < now {
                expired_transactions.insert(transaction_id, entry.value().clone());
            }
        }
        
        for transaction in expired_transactions.iter() {
            let transaction_id = transaction.key();
            let value = transaction.value();
            if value.receive_address == payment_address {
                continue; 
            }

            remove_address_from_nano_node_websocket_subscriptions(&value.receive_address).await.unwrap_or_else(|err| {
                println!("[Transaction Cache Cleanup] Failed to remove address {} from WebSocket subscriptions: {:?}", value.receive_address, err);
            });

            api_cache.transactions_to_process.remove(transaction_id);
            api_cache.transactions_to_process_index.remove(&TransactionDashMapKey {
                receive_address: value.receive_address.clone(),
                amount: value.amount.clone(),
            });
            
            remove_websocket_connection(transaction_id).await;

            if api_cache.topup_transactions_to_process.contains_key(transaction_id) {
                api_cache.topup_transactions_to_process.remove(transaction_id);
            }
        }
    }
}
    
