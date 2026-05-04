use std::collections::HashSet;

use axum::{
    extract::ws::{Message, WebSocket}
};
use tokio::sync::mpsc;

use crate::structs::{
    api_key::api_key::ParseApiKeyHeader, cache::api_cache::API_CACHE, websocket::websocket_message::{
        WebsocketMessage, WebsocketResponse, WebsocketTransactionData
    }
};

pub async fn setup_websocket_connection(
    mut socket: WebSocket, 
    transaction_id: String,
    api_key_header: ParseApiKeyHeader,

) {
    let (tx, mut rx) = mpsc::channel::<Message>(100);
    let sender_ptr = &tx as *const _ as usize;
    let api_cache = API_CACHE.get().unwrap();

    if !api_cache.transactions_to_process.contains_key(&transaction_id) {
        let _ = socket.send(Message::Text(serde_json::to_string(&WebsocketResponse {
            success: false,
            message: "Invalid transaction ID".to_string(),
        }).unwrap().into())).await;
        return;
    }

    if api_key_header.encrypted_api_key != api_cache.transactions_to_process.get(&transaction_id).unwrap().encrypted_api_key {
        let _ = socket.send(Message::Text(serde_json::to_string(&WebsocketResponse {
            success: false,
            message: "API key does not match transaction".to_string(),
        }).unwrap().into())).await;
        return;
    }

    api_cache.websocket_connections_index
        .entry(sender_ptr)
        .or_insert_with(|| (tx.clone(), HashSet::new()))
        .1
        .insert(transaction_id.clone());

    api_cache.websocket_connections
        .insert(transaction_id.clone(), tx.clone());

    if socket
        .send(Message::Text(serde_json::to_string(&WebsocketResponse {
            success: true,
            message: "Connected successfully".to_string(),
        }).unwrap().into()))
        .await
        .is_err()
    {
        api_cache.websocket_connections.remove(&transaction_id);
        if let Some(mut entry) = api_cache.websocket_connections_index.get_mut(&sender_ptr) {
            entry.1.remove(&transaction_id);
        }
        return;
    }

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if socket.send(msg).await.is_err() {
                break;
            }
        }
    });
}


pub async fn broadcast_transaction_to_client(
    transaction_id: String,
) {
    let api_cache = API_CACHE.get().unwrap();
    
    if let Some(sender_entry) = api_cache.websocket_connections.get(&transaction_id) {
        let sender = sender_entry.value().clone();
        let sender_ptr = &*sender_entry.value() as *const _ as usize;
        
        let message = WebsocketMessage {
            event: "transaction_confirmed".to_string(),
            data: WebsocketTransactionData {
                transaction_id: transaction_id.clone(),
                status: "confirmed".to_string(),
            },
        };

        let msg_text = serde_json::to_string(&message).unwrap();
        
        if sender.send(Message::Text(msg_text.into())).await.is_err() {
            drop(sender_entry);
            api_cache.websocket_connections.remove(&transaction_id);
            
            if let Some((_, entry)) = api_cache.websocket_connections_index.remove(&sender_ptr) {
                let (_, mut transaction_ids) = entry;
                transaction_ids.remove(&transaction_id);
                
                if !transaction_ids.is_empty() {
                    api_cache.websocket_connections_index.insert(sender_ptr, (sender, transaction_ids));
                }
            }
        }
    }
}


pub async fn remove_websocket_connection(transaction_id: &str) {
    let api_cache = API_CACHE.get().unwrap();
    
    if let Some((_, sender)) = api_cache.websocket_connections.remove(transaction_id) {
        let sender_ptr = &sender as *const _ as usize;
        
        if let Some(mut index_entry) = api_cache.websocket_connections_index.get_mut(&sender_ptr) {
            index_entry.1.remove(transaction_id);
            
            if index_entry.1.is_empty() {
                drop(index_entry);
                api_cache.websocket_connections_index.remove(&sender_ptr);
            }
        }
    }
}

