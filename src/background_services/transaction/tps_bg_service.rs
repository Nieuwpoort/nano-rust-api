use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

use tokio::{spawn, time::{Duration, sleep}};

use crate::{services::mail::alert::alert_mail_api::{send_tps_critical_email, send_tps_warning_email}, structs::cache::{counter_cache::{COUNTER_CACHE, CounterCache}, env::ENV_CACHE}};
static CAGE_KEY: &str = "tps";

static WARNING_MAIL_SEND: AtomicBool = AtomicBool::new(false);
static CRITICAL_MAIL_SEND: AtomicBool = AtomicBool::new(false);

pub async fn tps_worker() {
    let env = ENV_CACHE.get().unwrap();
    if env.tps_enabled != true {
        tracing::info!("TPS background service is disabled.");
        return;
    }
    let counter_cache = COUNTER_CACHE.get().unwrap();
    
    loop {
        process_tps_statistics(&counter_cache).await;

        sleep(Duration::from_secs(1)).await; 
    }
}

async fn process_tps_statistics(counter_cache: &Arc<CounterCache>) {
    let env = ENV_CACHE.get().unwrap();
    let current_tps = counter_cache.counters.get(CAGE_KEY).unwrap().clone();
    let peack_tps = counter_cache.peak_counters.get(CAGE_KEY).unwrap().clone();
    let warning_treshold = env.tps_transaction_warning_threshold;
    let critical_treshold = env.tps_critical_transaction_threshold;
    
    *counter_cache.counters.get_mut(CAGE_KEY).unwrap() = 0;
    
    if current_tps > peack_tps {
        *counter_cache.peak_counters.get_mut(CAGE_KEY).unwrap() = current_tps;
    }

    if current_tps >= critical_treshold && !CRITICAL_MAIL_SEND.load(Ordering::Relaxed) {
        spawn(send_tps_critical_email());

        CRITICAL_MAIL_SEND.store(true, Ordering::Relaxed);
    }     if current_tps >= warning_treshold && !WARNING_MAIL_SEND.load(Ordering::Relaxed) {
        spawn(send_tps_warning_email());

        WARNING_MAIL_SEND.store(true, Ordering::Relaxed);
    } 
}