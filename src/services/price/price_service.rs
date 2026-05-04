use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// Cache voor de prijs die elke 60 seconden wordt bijgewerkt
pub static PRICE_CACHE: Lazy<Arc<RwLock<Option<NanoPrice>>>> = 
    Lazy::new(|| Arc::new(RwLock::new(None)));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NanoPrice {
    pub usd: f64,
    pub eur: f64,
    pub btc: f64,
    pub last_updated: DateTime<Utc>,
}

/// Fetch Nano price van Kraken API (geen API key nodig)
pub async fn fetch_nano_price() -> Result<NanoPrice, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("ifenpay/1.0")
        .build()
        .map_err(|e| format!("Failed to build client: {}", e))?;
    
    // Kraken gebruikt NANO/USD en NANO/EUR pairs
    let url = "https://api.kraken.com/0/public/Ticker?pair=NANOUSD,NANOEUR,XBTEUR";
    
    let response = client.get(url).send().await
        .map_err(|e| format!("Failed to fetch from Kraken: {}", e))?;
    
    let text = response.text().await
        .map_err(|e| format!("Failed to read response: {}", e))?;
    
    tracing::debug!("Kraken response: {}", text);
    
    let data: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    // Kraken result heeft keys zoals NANOUSD, NANOEUR
    let result = data.get("result")
        .ok_or_else(|| format!("No result in response"))?;
    
    // Parse USD price (laatste trade prijs is index 0 in 'c' array)
    let usd_str = result.get("NANOUSD")
        .and_then(|v| v.get("c"))
        .and_then(|v| v.get(0))
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("Failed to get NANOUSD price from response"))?;
    
    let usd: f64 = usd_str.parse()
        .map_err(|e| format!("Failed to parse USD price '{}': {}", usd_str, e))?;
    
    // Parse EUR price
    let eur_str = result.get("NANOEUR")
        .and_then(|v| v.get("c"))
        .and_then(|v| v.get(0))
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("Failed to get NANOEUR price from response"))?;
    
    let eur: f64 = eur_str.parse()
        .map_err(|e| format!("Failed to parse EUR price '{}': {}", eur_str, e))?;
    
    // Haal BTC/EUR prijs op om NANO/BTC te berekenen
    let btc_eur_str = result.get("XXBTZEUR")
        .and_then(|v| v.get("c"))
        .and_then(|v| v.get(0))
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("Failed to get XBTEUR price from response"))?;
    
    let btc_eur: f64 = btc_eur_str.parse()
        .map_err(|e| format!("Failed to parse BTC/EUR price: {}", e))?;
    
    // Bereken NANO/BTC (NANO_EUR / BTC_EUR)
    let btc = eur / btc_eur;
    
    tracing::info!("Fetched Nano prices - USD: ${}, EUR: €{}, BTC: ₿{:.8}", usd, eur, btc);
    
    Ok(NanoPrice {
        usd,
        eur,
        btc,
        last_updated: Utc::now(),
    })
}

/// Update de price cache
pub async fn update_price_cache() -> Result<(), String> {
    let price = fetch_nano_price().await?;
    
    let mut cache = PRICE_CACHE.write().await;
    *cache = Some(price.clone());
    
    tracing::info!("Updated Nano price: ${} USD, €{} EUR", price.usd, price.eur);
    
    Ok(())
}

/// Haal de gecachte prijs op
pub async fn get_cached_price() -> Option<NanoPrice> {
    let cache = PRICE_CACHE.read().await;
    cache.clone()
}

/// Check of de cache nog vers is (jonger dan 5 minuten)
pub async fn is_cache_fresh() -> bool {
    let cache = PRICE_CACHE.read().await;
    
    match cache.as_ref() {
        Some(price) => {
            let age = Utc::now() - price.last_updated;
            age.num_seconds() < 300 // 5 minuten
        },
        None => false,
    }
}

/// Haal prijs op (met fallback naar API als cache oud is)
pub async fn get_nano_price() -> Result<NanoPrice, String> {
    // Probeer eerst de cache
    if is_cache_fresh().await {
        if let Some(price) = get_cached_price().await {
            return Ok(price);
        }
    }
    
    // Als cache oud of leeg is, haal nieuwe data op
    update_price_cache().await?;
    
    get_cached_price()
        .await
        .ok_or_else(|| "Failed to get price from cache after update".to_string())
}
