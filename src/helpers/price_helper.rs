use crate::{services::price::price_service, structs::{cache::env::ENV_CACHE, credits::credits::CreditsPrices}};

/// Converteer Nano bedrag naar USD
/// 
/// Voorbeeld: nano_to_usd("1.5") geeft Some(1.875) als Nano $1.25 is
pub async fn nano_to_usd(nano_amount: &str) -> Option<f64> {
    let amount: f64 = nano_amount.parse().ok()?;
    let price = price_service::get_cached_price().await?;
    Some(amount * price.usd)
}

/// Converteer Nano bedrag naar EUR
pub async fn nano_to_eur(nano_amount: &str) -> Option<f64> {
    let amount: f64 = nano_amount.parse().ok()?;
    let price = price_service::get_cached_price().await?;
    Some(amount * price.eur)
}

/// Format een Nano bedrag met USD waarde
/// 
/// Voorbeeld: "5.25 NANO (~$6.56 USD)"
pub async fn format_nano_with_usd(nano_amount: &str) -> String {
    match nano_to_usd(nano_amount).await {
        Some(usd_value) => {
            format!("{} NANO (~${:.2} USD)", nano_amount, usd_value)
        },
        None => {
            format!("{} NANO", nano_amount)
        }
    }
}

pub async fn format_nano_with_eur(nano_amount: &str) -> String {
    match nano_to_eur(nano_amount).await {
        Some(eur_value) => {
            format!("{} NANO (~€{:.2} EUR)", nano_amount, eur_value)
        },
        None => {
            format!("{} NANO", nano_amount)
        }
    }
}

/// Converteer USD bedrag naar NANO
/// 
/// Voorbeeld: usd_to_nano(0.50) geeft Some(0.40) als Nano $1.25 is
pub async fn usd_to_nano(usd_amount: f64) -> Option<f64> {
    let price = price_service::get_cached_price().await?;
    if price.usd == 0.0 {
        return None;
    }
    let nano_amount = usd_amount / price.usd;
    // Afronden tot 4 decimalen
    Some((nano_amount * 10000.0).round() / 10000.0)
}

/// Haal de Nano prijs voor credits op uit .env en converteer naar NANO
/// 
/// Verwacht .env variabelen zoals:
/// - CREDITS_PRICE_100=0.10 (USD prijs voor 100 credits)
/// - CREDITS_PRICE_500=0.45
/// etc.
pub async fn get_credits_price_in_nano(credits_amount: u32) -> Option<f64> {
    let env = ENV_CACHE.get().unwrap();
    let usd_price = match credits_amount {
        10 => env.credits_price_10,
        50 => env.credits_price_50,
        100 => env.credits_price_100,
        500 => env.credits_price_500,
        1000 => env.credits_price_1000,
        5000 => env.credits_price_5000,
        10000 => env.credits_price_10000,
        50000 => env.credits_price_50000,
        100000 => env.credits_price_100000,
        _ => return None,
    };
    
    usd_to_nano(usd_price).await
}

/// Helper functie om naar 4 decimalen af te ronden
fn round_to_4_decimals(value: f64) -> f64 {
    (value * 10000.0).round() / 10000.0
}

/// Haal alle credit prijzen op in NANO
/// Returns error als een prijs 0 is of niet gevonden kan worden
pub async fn get_all_credits_prices() -> Result<CreditsPrices, String> {
    let price_10 = get_credits_price_in_nano(10).await
        .ok_or_else(|| "Failed to calculate price for 10 credits".to_string())?;
    let price_50 = get_credits_price_in_nano(50).await
        .ok_or_else(|| "Failed to calculate price for 50 credits".to_string())?;
    let price_100 = get_credits_price_in_nano(100).await
        .ok_or_else(|| "Failed to calculate price for 100 credits".to_string())?;
    let price_500 = get_credits_price_in_nano(500).await
        .ok_or_else(|| "Failed to calculate price for 500 credits".to_string())?;
    let price_1000 = get_credits_price_in_nano(1000).await
        .ok_or_else(|| "Failed to calculate price for 1000 credits".to_string())?;
    let price_5000 = get_credits_price_in_nano(5000).await
        .ok_or_else(|| "Failed to calculate price for 5000 credits".to_string())?;
    let price_10000 = get_credits_price_in_nano(10000).await
        .ok_or_else(|| "Failed to calculate price for 10000 credits".to_string())?;
    let price_50000 = get_credits_price_in_nano(50000).await
        .ok_or_else(|| "Failed to calculate price for 50000 credits".to_string())?;
    let price_100000 = get_credits_price_in_nano(100000).await
        .ok_or_else(|| "Failed to calculate price for 100000 credits".to_string())?;

    if price_10 <= 0.0 || price_50 <= 0.0 || price_100 <= 0.0 || price_500 <= 0.0 || price_1000 <= 0.0 || 
       price_5000 <= 0.0 || price_10000 <= 0.0 || price_50000 <= 0.0 || price_100000 <= 0.0 {
        return Err("One or more credit prices is zero or negative - pricing unavailable".to_string());
    }

    Ok(CreditsPrices {
        price_10: round_to_4_decimals(price_10),
        price_50: round_to_4_decimals(price_50),
        price_100: round_to_4_decimals(price_100),
        price_500: round_to_4_decimals(price_500),
        price_1000: round_to_4_decimals(price_1000),
        price_5000: round_to_4_decimals(price_5000),
        price_10000: round_to_4_decimals(price_10000),
        price_50000: round_to_4_decimals(price_50000),
        price_100000: round_to_4_decimals(price_100000),
    })
}


