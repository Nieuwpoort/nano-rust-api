use std::sync::Arc;

use crate::{structs::cache::{api_cache::{API_CACHE, ApiCache}, counter_cache::{COUNTER_CACHE, CounterCache}}, services::postgres::postgres::get_pool, structs::api_key::{api_key_data::ApiKeyData, api_key_table::ApiKeyTable}};


pub async fn initilialize_cache() -> anyhow::Result<()> {
    let api_cache = ApiCache::new();

    initilize_api_keys(&api_cache).await;
    initialize_counter_cache().await;

    API_CACHE.set(Arc::new(api_cache)).unwrap();

    Ok(())
}

async fn initilize_api_keys(api_cache: &ApiCache) {
    let api_keys = sqlx::query_as!(
        ApiKeyTable,
        r#"
        SELECT id, user_id, name, nano_address, api_key, is_active, credits, created_at, updated_at
        FROM api_keys
        WHERE is_active = true
        "#
    ).fetch_all(get_pool())
    .await
    .expect("Failed to load API keys from database");

    for api_key in api_keys {
        api_cache.api_key_data.insert(api_key.api_key.clone(), ApiKeyData {
            id: api_key.id,
            api_key: api_key.api_key,
            user_id: api_key.user_id,
            name: api_key.name,
            nano_address: api_key.nano_address,
            is_active: api_key.is_active,
            credits: api_key.credits,
        });
    }
}

async fn initialize_counter_cache() {
    let counter_cache = CounterCache::new();

    let completed_transactions_count = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM paid_transactions
        "#
    )
    .fetch_one(get_pool())
    .await
    .expect("Failed to load completed transactions count from database");

    counter_cache.counters.insert("total_transactions".to_string(), completed_transactions_count.count.unwrap_or(0) as i64);

    COUNTER_CACHE.set(Arc::new(counter_cache)).unwrap();
}