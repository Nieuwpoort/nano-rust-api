pub mod background_services;
pub mod database;
pub mod r#enum;
pub mod handlers;
pub mod helpers;
pub mod middleware;
pub mod services;
pub mod structs;
use std::sync::Arc;

use tokio::spawn;
use tracing_subscriber;

use crate::{background_services::{price::start_price_update_service, transaction::{tph_bg_service::tph_worker, tps_bg_service::tps_worker, 
    transaction_cache_cleanup_bg_service::transaction_cleanup_worker}, 
    websocket::{nano_node_websocket_bg_service::intialize_nano_node_websocket_bg_service, websocket_bg_service::maintain_websocket_connections}}, 
services::{cache::api_cache::initilialize_cache, postgres::postgres::init_pool, price::price_service, routers::api_router::initialize_api_routers, 
stats::stats_service::init_start_time}, structs::cache::env::{ENV_CACHE, EnvCache}};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    
    initialize_env_cache();

    init_start_time();

    init_pool().await;

    initialize_caches().await;
    initialize_price().await;
    initialize_bg_services().await;

    initialize_api_routers().await;
}

fn initialize_env_cache() {
    let env_cache = Arc::new(EnvCache::new());
    ENV_CACHE.set(env_cache).expect("Failed to initialize environment cache");
}


async fn initialize_bg_services() {
    spawn(tps_worker());
    spawn(tph_worker());
    spawn(intialize_nano_node_websocket_bg_service());
    spawn(transaction_cleanup_worker());
    spawn(start_price_update_service());
    spawn(maintain_websocket_connections());
}

async fn initialize_caches() {
    initilialize_cache().await.expect("Failed to initialize cache");
}

async fn initialize_price() {
    match price_service::update_price_cache().await {
        Ok(_) => tracing::info!("✅ Nano price initialized successfully"),
        Err(e) => tracing::warn!("⚠️ Failed to initialize Nano price: {}", e),
    }
}
