use std::collections::HashMap;

pub fn convert_currency(
    amount: f64,
    from_currency: &str,
    to_currency: &str,
    rate_map: &HashMap<String, f64>,
) -> f64 {
    if from_currency == to_currency {
        return amount;
    }

    // Convert to USD first if needed
    let to_usd = format!("{}/USD", from_currency);
    let usd_to_target = if to_currency == "USD" {
        1.0
    } else {
        let usd_target = format!("USD/{}", to_currency);
        if let Some(&rate) = rate_map.get(&usd_target) {
            rate
        } else {
            println!("⚠️  Warning: No conversion rate found for USD to {}", to_currency);
            1.0 // Return 1.0 as fallback
        }
    };

    // Do the conversion
    if let Some(&rate) = rate_map.get(&to_usd) {
        amount * rate * usd_to_target
    } else {
        println!("⚠️  Warning: No conversion rate found for {} to {}", from_currency, to_currency);
        amount // Return unconverted amount as fallback
    }
}
