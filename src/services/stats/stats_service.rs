use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};
use axum::Json;
use sysinfo::{System, CpuRefreshKind, MemoryRefreshKind, RefreshKind, Pid, ProcessesToUpdate};

use crate::{
    services::{postgres::postgres::get_pool, price::price_service},
    structs::{
        cache::{counter_cache::COUNTER_CACHE, api_cache::API_CACHE},
        stats::resource_stats_response::ResourceStatsResponse,
    },
};

// Global start time to track uptime
static START_TIME: OnceLock<u64> = OnceLock::new();

pub fn init_start_time() {
    START_TIME.get_or_init(|| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    });
}

pub async fn get_memory_stats() -> Json<ResourceStatsResponse> {
    // Get current process ID
    let current_pid = Pid::from_u32(std::process::id());
    
    // Initialize sysinfo
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything())
    );
    
    // Refresh to get current values
    sys.refresh_cpu_all();
    sys.refresh_memory();
    
    // Refresh only the current process
    sys.refresh_processes(ProcessesToUpdate::Some(&[current_pid]), false);
    
    // Calculate uptime
    let start_time = START_TIME.get().copied().unwrap_or(0);
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let uptime_seconds = current_time.saturating_sub(start_time);
    
    // Get process-specific stats (this application only)
    let (memory_used_bytes, memory_used_mb, cpu_usage_percent) = sys.process(current_pid)
        .map(|process| {
            let bytes = process.memory();
            let mb = bytes as f64 / 1_048_576.0;
            let cpu = process.cpu_usage() as f64;
            (bytes, mb, cpu)
        })
        .unwrap_or((0, 0.0, 0.0));
    
    // Get CPU cores count
    let cpu_cores = sys.cpus().len();
    
    // Get transaction stats from cache
    let current_tps = COUNTER_CACHE
        .get()
        .and_then(|cache| cache.counters.get("tps"))
        .map(|v| *v.value() as u64)
        .unwrap_or(0);
    
    let current_tph = COUNTER_CACHE
        .get()
        .and_then(|cache| cache.counters.get("tph"))
        .map(|v| *v.value() as u64)
        .unwrap_or(0);
    
    let peak_tps = COUNTER_CACHE
        .get()
        .and_then(|cache| cache.peak_counters.get("tps"))
        .map(|v| *v.value() as u64)
        .unwrap_or(0);
    
    let total_transactions = COUNTER_CACHE
        .get()
        .and_then(|cache| cache.counters.get("total_transactions"))
        .map(|v| *v.value() as u64)
        .unwrap_or(0);
    
    // Get cache sizes
    let (cache_api_keys, cache_transactions) = API_CACHE
        .get()
        .map(|cache| (
            cache.api_key_data.len(),
            cache.transactions_to_process.len(),
        ))
        .unwrap_or((0, 0));
    
    // Get database pool stats
    let pool = get_pool();
    let db_pool_size = pool.options().get_max_connections();
    let db_active_connections = pool.size() as u32;
    let db_idle_connections = pool.num_idle() as u32;
    
    // Get Nano price from cache
    let (nano_price_usd, nano_price_eur, nano_price_btc) = price_service::get_cached_price()
        .await
        .map(|price| (Some(price.usd), Some(price.eur), Some(price.btc)))
        .unwrap_or((None, None, None));
    
    Json(ResourceStatsResponse {
        memory_used_bytes,
        memory_used_mb: (memory_used_mb * 100.0).round() / 100.0, // Round to 2 decimals
        
        cpu_usage_percent: (cpu_usage_percent * 100.0).round() / 100.0,
        cpu_cores,
        
        uptime_seconds,
        uptime_human: ResourceStatsResponse::format_uptime(uptime_seconds),
        
        current_tps,
        current_tph,
        peak_tps,
        total_transactions,
        
        cache_api_keys,
        cache_transactions,
        
        db_pool_size,
        db_active_connections,
        db_idle_connections,
        
        nano_price_usd,
        nano_price_eur,
        nano_price_btc,
    })
}