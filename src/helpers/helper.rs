use crate::{r#enum::api::error::transaction::TransactionError, structs::api::api::ApiErrorResult};


pub fn raw_to_nano(raw: &str) -> String {
    let raw_value: u128 = raw.trim().parse().unwrap_or(0);
    let raw_str = raw_value.to_string();

    if raw_str == "0" {
        return "0".to_string();
    }

    if raw_str.len() <= 30 {
        let mut fractional = format!("{:0>30}", raw_str);
        while fractional.ends_with('0') {
            fractional.pop();
        }
        return format!("0.{}", fractional);
    }

    let split_index = raw_str.len() - 30;
    let whole = &raw_str[..split_index];
    let mut fractional = raw_str[split_index..].to_string();

    while fractional.ends_with('0') {
        fractional.pop();
    }

    if fractional.is_empty() {
        whole.to_string()
    } else {
        format!("{}.{}", whole, fractional)
    }
}



pub fn nano_to_raw(nano: &str) -> Result<String, ApiErrorResult> {
    // Trim whitespace
    let nano = nano.trim();
    
    // Check for negative
    if nano.starts_with('-') {
        return Err(TransactionError::InvalidNegativeAmount.to_response());
    }
    
    // Split into whole and fractional parts
    let parts: Vec<&str> = nano.split('.').collect();
    
    if parts.len() > 2 {
        return Err(TransactionError::InvalidNumberFormat.to_response());
    }
    
    // Parse whole part
    let whole: u128 = if parts[0].is_empty() {
        0
    } else {
        parts[0].parse()
            .map_err(|_| TransactionError::InvalidWholeNumber.to_response())?
    };
    
    // Parse fractional part (up to 30 decimals)
    let fractional: u128 = if parts.len() == 2 {
        let frac_str = parts[1];
        
        // Nano has max 30 decimal places
        if frac_str.len() > 30 {
            return Err(TransactionError::TooManyDecimalPlaces.to_response());
        }
        
        // Pad with zeros to make 30 digits
        let padded = format!("{:0<30}", frac_str);
        padded.parse()
            .map_err(|_| TransactionError::InvalidFractionalPart.to_response())?
    } else {
        0
    };
    
    // Calculate: whole * 10^30 + fractional
    let raw = whole.checked_mul(1_000_000_000_000_000_000_000_000_000_000)
        .ok_or(TransactionError::AmountToLarge.to_response())?
        .checked_add(fractional)
        .ok_or(TransactionError::AmountToLarge.to_response())?;
    
    Ok(raw.to_string())
}