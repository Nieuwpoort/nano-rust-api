use tokio::time::{sleep, Duration};

use crate::{services::postgres::postgres::get_pool, structs::cache::{counter_cache::{COUNTER_CACHE, CounterCache}, env::ENV_CACHE}};

pub async fn daily_peak_tps_worker() {
    let env = ENV_CACHE.get().unwrap();
    if env.tph_enabled != true {
        tracing::info!("TPH background service is disabled.");
        return;
    }
    
    let mut first_sleep_duration_initilized = false;
    let counter_cache = COUNTER_CACHE.get().unwrap();
    let sleep_duration = Duration::from_secs(60 * 60); 
    let now = chrono::Utc::now();
    let next_day = now.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1) - chrono::Duration::seconds(2);
    let mut first_sleep_duration = (next_day - now.naive_utc()).to_std().unwrap();

    first_sleep_duration -= Duration::from_secs(2);

    loop {
        if first_sleep_duration_initilized {
            process_daily_peak_tps_statistics(&counter_cache).await;

            sleep(sleep_duration).await; 
        }
        else {
            first_sleep_duration_initilized = true;

            sleep(first_sleep_duration).await;
        }

    }
}

async fn process_daily_peak_tps_statistics(counter_cache: &CounterCache) {
    let time_now = chrono::Utc::now();
    let current_peak_tps = counter_cache.peak_counters.get("tps").unwrap().clone();

    *counter_cache.peak_counters.get_mut("tps").unwrap() = 0;

    let db = get_pool();

    let _ = sqlx::query!(
        r#"
        INSERT INTO daily_peak_tps (peak_tps, created_at)
        VALUES ($1, $2)
        "#,
        current_peak_tps as i64,
        time_now.naive_utc(),
    ).execute(db)
    .await
    .unwrap();
}