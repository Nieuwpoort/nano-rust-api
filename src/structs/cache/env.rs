use std::{env, sync::Arc};

use once_cell::sync::OnceCell;

pub static ENV_CACHE: OnceCell<Arc<EnvCache>> = OnceCell::new();

#[derive(Debug)]
pub struct EnvCache {
    pub server_is_live: bool,

    pub database_url: String,
    pub http_listen_addr: String,
    pub web_http_listen_addr: String,
    pub api_base_url: String,
    pub web_base_url: String,
    pub web_socket_base_url: String,

    pub nano_node_rpc_url: String,
    pub nano_default_representative: String,

    pub twofa_encrypt_key: String,
    pub twofa_encrypt_iv: String,
    pub user_encrypt_key: String,
    pub user_encrypt_iv: String,
    pub api_encrypt_key: String,
    pub api_encrypt_iv: String,

    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,

    pub tph_enabled: bool,
    pub tps_enabled: bool,
    pub daily_peak_tps_enabled: bool,
    pub tps_transaction_warning_threshold: i64,
    pub tps_critical_transaction_threshold: i64,
    pub alert_email_recipients: Vec<String>,

    pub credits_price_10: f64,
    pub credits_price_50: f64,
    pub credits_price_100: f64,
    pub credits_price_500: f64,
    pub credits_price_1000: f64,
    pub credits_price_5000: f64,
    pub credits_price_10000: f64,
    pub credits_price_50000: f64,
    pub credits_price_100000: f64,

    pub credits_cost_per_pow: u32,
    pub credits_cost_per_publish: u32,

    pub topup_credits_address: String,
    pub donate_address: String,

    pub remove_transactions_after_minutes: i64,
}

impl EnvCache {
    pub fn new() -> Self {
        Self {
            server_is_live: env::var("SERVER_IS_LIVE").unwrap().parse().unwrap(),

            database_url: env::var("DATABASE_URL").unwrap(),
            http_listen_addr: env::var("HTTP_LISTEN_ADDR").unwrap(),
            web_http_listen_addr: env::var("WEB_HTTP_LISTEN_ADDR").unwrap(),
            api_base_url: env::var("API_BASE_URL").unwrap(),
            web_base_url: env::var("WEB_BASE_URL").unwrap(),
            web_socket_base_url: env::var("WEB_SOCKET_BASE_URL").unwrap(),

            nano_node_rpc_url: env::var("NANO_NODE_RPC_URL").unwrap(),
            nano_default_representative: env::var("NANO_DEFAULT_REPRESENTATIVE").unwrap(),

            twofa_encrypt_key: env::var("TWOFA_ENCRYPT_KEY").unwrap(),
            twofa_encrypt_iv: env::var("TWOFA_ENCRYPT_IV").unwrap(),
            user_encrypt_key: env::var("USER_ENCRYPT_KEY").unwrap(),
            user_encrypt_iv: env::var("USER_ENCRYPT_IV").unwrap(),
            api_encrypt_key: env::var("API_ENCRYPT_KEY").unwrap(),
            api_encrypt_iv: env::var("API_ENCRYPT_IV").unwrap(),

            smtp_host: env::var("SMTP_HOST").unwrap(),
            smtp_port: env::var("SMTP_PORT").unwrap().parse().unwrap(),
            smtp_user: env::var("SMTP_USER").unwrap(),
            smtp_pass: env::var("SMTP_PASS").unwrap(),

            tph_enabled: env::var("TPH_ENABLED").unwrap().parse().unwrap(),
            tps_enabled: env::var("TPS_ENABLED").unwrap().parse().unwrap(),
            daily_peak_tps_enabled: env::var("DAILY_PEAK_TPS_ENABLED").unwrap().parse().unwrap(),
            tps_transaction_warning_threshold: env::var("TPS_TRANSACTION_WARNING_THRESHOLD").unwrap().parse().unwrap(),
            tps_critical_transaction_threshold: env::var("TPS_CRITICAL_TRANSACTION_THRESHOLD").unwrap().parse().unwrap(),
            alert_email_recipients: env::var("ALERT_EMAIL_RECIPIENTS").unwrap().split(',').map(|s| s.trim().to_string()).collect(),

            credits_price_10: env::var("CREDITS_PRICE_10").unwrap().parse().unwrap(),
            credits_price_50: env::var("CREDITS_PRICE_50").unwrap().parse().unwrap(),
            credits_price_100: env::var("CREDITS_PRICE_100").unwrap().parse().unwrap(),
            credits_price_500: env::var("CREDITS_PRICE_500").unwrap().parse().unwrap(),
            credits_price_1000: env::var("CREDITS_PRICE_1000").unwrap().parse().unwrap(),
            credits_price_5000: env::var("CREDITS_PRICE_5000").unwrap().parse().unwrap(),
            credits_price_10000: env::var("CREDITS_PRICE_10000").unwrap().parse().unwrap(),
            credits_price_50000: env::var("CREDITS_PRICE_50000").unwrap().parse().unwrap(),
            credits_price_100000: env::var("CREDITS_PRICE_100000").unwrap().parse().unwrap(),

            credits_cost_per_pow: env::var("CREDITS_COST_PER_POW").unwrap().parse().unwrap(),
            credits_cost_per_publish: env::var("CREDITS_COST_PER_PUBLISH").unwrap().parse().unwrap(),

            topup_credits_address: env::var("TOPUP_CREDITS_ADDRESS").unwrap(),
            donate_address: env::var("DONATE_ADDRESS").unwrap(),

            remove_transactions_after_minutes: env::var("REMOVE_TRANSACTIONS_AFTER_MINUTES").unwrap().parse().unwrap(),
        }
    }
}