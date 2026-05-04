use std::time::Duration;
use tokio::time::interval;

use crate::services::price::price_service;

/// Background service die elke 60 seconden de Nano prijs bijwerkt
pub async fn start_price_update_service() {
    tokio::spawn(async {
        // Dan elke 60 seconden updaten (eerste update gebeurt al in main.rs)
        let mut ticker = interval(Duration::from_secs(60));
        
        loop {
            ticker.tick().await;
            
            if let Err(e) = price_service::update_price_cache().await {
                tracing::error!("Failed to update Nano price: {}", e);
            }
        }
    });
    
    tracing::info!("Started Nano price update service (updates every 60 seconds)");
}
