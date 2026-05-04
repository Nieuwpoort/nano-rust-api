use tokio::sync::mpsc::{self, Sender, Receiver};

use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::tungstenite::Message;
use tokio::time::{sleep, Duration};

use crate::{r#enum::nano_web_socket_enum::NanoNodeWebSocketCommand, services::{transaction::transaction_service::db_create_paid_transaction, websocket::web_socket::broadcast_transaction_to_client}, structs::{cache::{api_cache::API_CACHE, env::ENV_CACHE, transaction_cache_data::TransactionDashMapKey}, websocket::nano_node_websocket::NanoNodeWebsocketResponse}};

use once_cell::sync::OnceCell;
pub static WEBSOCKET_COMMAND_SENDER: OnceCell<Sender<NanoNodeWebSocketCommand>> = OnceCell::new();

pub async fn intialize_nano_node_websocket_bg_service() {
    let api_cache = API_CACHE.get().unwrap();
    let env = ENV_CACHE.get().unwrap();
    let url = env.web_socket_base_url.clone();
    let mut attempt: u32 = 0;

    let (tx, mut rx): (Sender<NanoNodeWebSocketCommand>, Receiver<NanoNodeWebSocketCommand>) = mpsc::channel(32);
    WEBSOCKET_COMMAND_SENDER.set(tx).ok();

    loop {
        match tokio_tungstenite::connect_async(&url).await {
            Ok((mut ws_stream, _)) => {
                attempt = 0;
                let payment_address = env.topup_credits_address.clone();
                let subscribe_msg = json!({
                    "action": "subscribe",
                    "topic": "confirmation",
                    "options": { 
                        "accounts": [payment_address],
                    }
                });
                if let Err(_) = ws_stream.send(Message::Text(subscribe_msg.to_string().into())).await {
                    continue;
                }

                loop {
                    tokio::select! {
                        ws_msg = ws_stream.next() => {
                            match ws_msg {
                                Some(Ok(Message::Text(txt))) => {
                                    if let Ok(event) = serde_json::from_str::<serde_json::Value>(&txt) {
                                        if event.get("topic") == Some(&serde_json::Value::String("confirmation".to_string())) {
                                            if let Ok(response) = serde_json::from_value::<NanoNodeWebsocketResponse>(event["message"].clone()) {
                                                println!("Received WebSocket confirmation for with amount: {} to address: {}",  response.amount, response.block.link_as_account);
                                                if response.block.subtype != "send" {
                                                    continue;
                                                }
                                                let transaction_dashmap_key = TransactionDashMapKey {
                                                    receive_address: response.block.link_as_account.clone(),
                                                    amount: response.amount.clone(),
                                                };
                                                if api_cache.transactions_to_process_index.contains_key(&transaction_dashmap_key) {
                                                    let transaction_id = api_cache.transactions_to_process_index.get(&transaction_dashmap_key).unwrap().clone();
                                                    let transaction_data = api_cache.transactions_to_process.get(&transaction_id).unwrap().clone();
                                                    let api_key_id = api_cache.api_key_data.get(&transaction_data.encrypted_api_key).unwrap().id;
                                                    
                                                    db_create_paid_transaction(&transaction_data, transaction_id.clone(), api_key_id).await.unwrap();
                                                    println!("Transaction {} marked as paid.", transaction_id);

                                                    api_cache.payed_transactions.insert(transaction_id.clone(), ());
                                                    api_cache.transactions_to_process.remove(&transaction_id);
                                                    api_cache.transactions_to_process_index.remove(&transaction_dashmap_key);

                                                    let _ = broadcast_transaction_to_client(
                                                        transaction_id.clone(),
                                                    ).await;
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(Ok(_)) => {},
                                Some(Err(_)) | None => { break; }
                            }
                        }
                        // Verwerk commands van andere delen van de app
                        Some(cmd) = rx.recv() => {
                            match cmd {
                                NanoNodeWebSocketCommand::AddAccount(addr) => {
                                    let msg = json!({
                                        "action": "update",
                                        "topic": "confirmation",
                                        "options": { "accounts_add": [addr] }
                                    });
                                    let _ = ws_stream.send(Message::Text(msg.to_string().into())).await;
                                }
                                NanoNodeWebSocketCommand::RemoveAccount(addr) => {
                                    let msg = json!({
                                        "action": "update",
                                        "topic": "confirmation",
                                        "options": { "accounts_del": [addr] }
                                    });
                                    let _ = ws_stream.send(Message::Text(msg.to_string().into())).await;
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("[WebSocket] Can't connect: {}", e);
            }
        }
        attempt += 1;
        let wait = (2u64.pow(attempt.min(4))) * 1000;
        sleep(Duration::from_millis(wait)).await;
    }
}