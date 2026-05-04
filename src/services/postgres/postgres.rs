use once_cell::sync::OnceCell;
use sqlx::PgPool;

use crate::structs::cache::env::ENV_CACHE;

static DB_POOL: OnceCell<PgPool> = OnceCell::new();

pub async fn init_pool() {
    let env = ENV_CACHE.get().unwrap();
    let url = env.database_url.clone();
    let pool = PgPool::connect(&url).await.expect("Failed to connect to Postgres");
    DB_POOL.set(pool).expect("Pool already set");
}
pub fn get_pool() -> &'static PgPool {
    DB_POOL.get().expect("Pool not initialized")
}