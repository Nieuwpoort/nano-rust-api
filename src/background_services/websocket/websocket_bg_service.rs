use axum::extract::ws::Message;
use futures::stream::{FuturesUnordered, StreamExt};
use tokio::time::{sleep, Duration};
use crate::structs::cache::api_cache::API_CACHE;

pub async fn maintain_websocket_connections() {
    loop {
        sleep(Duration::from_secs(20)).await;

        let api_cache = API_CACHE.get().unwrap();
        let ping_msg = Message::Ping(Vec::new().into());
        let mut dead_senders = Vec::new();

        let mut futures = FuturesUnordered::new();

        for entry in api_cache.websocket_connections_index.iter() {
            let sender_ptr = *entry.key();
            let sender = entry.value().0.clone();
            let ping = ping_msg.clone();

            futures.push(tokio::spawn(async move {
                let result = sender.send(ping).await;
                (sender_ptr, result)
            }));
        }

        while let Some(result) = futures.next().await {
            if let Ok((sender_ptr, send_result)) = result {
                if send_result.is_err() {
                    dead_senders.push(sender_ptr);
                }
            }
        }

        for sender_ptr in dead_senders {
            if let Some((_, entry)) = api_cache.websocket_connections_index.remove(&sender_ptr) {
                let (_, transaction_ids) = entry;
                // Verwijder ook alle transaction_id entries
                for transaction_id in transaction_ids {
                    api_cache.websocket_connections.remove(&transaction_id);
                }
            }
        }
    }
}