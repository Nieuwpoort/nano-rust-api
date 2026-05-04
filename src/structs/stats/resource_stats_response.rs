use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceStatsResponse {
    /// Memory used by this application in bytes
    pub memory_used_bytes: u64,
    /// Memory used by this application in MB
    pub memory_used_mb: f64,
    
    /// CPU usage percentage of this application (0-100)
    pub cpu_usage_percent: f64,
    /// Number of CPU cores available
    pub cpu_cores: usize,
    
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// Uptime in human-readable format (e.g., "2d 5h 30m")
    pub uptime_human: String,
    
    /// Current transactions per second
    pub current_tps: u64,
    /// Current transactions per hour
    pub current_tph: u64,
    /// Peak transactions per second
    pub peak_tps: u64,
    /// Total transactions processed
    pub total_transactions: u64,
    
    /// Number of active API key cache entries
    pub cache_api_keys: usize,
    /// Number of transaction cache entries
    pub cache_transactions: usize,
    
    /// Database connection pool size
    pub db_pool_size: u32,
    /// Database active connections
    pub db_active_connections: u32,
    /// Database idle connections
    pub db_idle_connections: u32,
    
    /// Nano price in USD
    pub nano_price_usd: Option<f64>,
    /// Nano price in EUR
    pub nano_price_eur: Option<f64>,
    /// Nano price in BTC
    pub nano_price_btc: Option<f64>,
}

impl ResourceStatsResponse {
    pub fn format_uptime(seconds: u64) -> String {
        let days = seconds / 86400;
        let hours = (seconds % 86400) / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;
        
        if days > 0 {
            format!("{}d {}h {}m {}s", days, hours, minutes, secs)
        } else if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, secs)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, secs)
        } else {
            format!("{}s", secs)
        }
    }
}
