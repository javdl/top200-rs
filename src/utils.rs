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

    // Handle special cases for currency subunits and alternative codes
    let (adjusted_amount, adjusted_from_currency) = match from_currency {
        "GBp" => (amount / 100.0, "GBP"),  // Convert pence to pounds
        "ZAc" => (amount / 100.0, "ZAR"),
        "ILA" => (amount, "ILS"),
        _ => (amount, from_currency),
    };

    // Adjust target currency if needed
    let adjusted_to_currency = match to_currency {
        "GBp" => "GBP",  // Also handle GBp as target currency
        "ZAc" => "ZAR",  // Also handle ZAc as target currency
        "ILA" => "ILS",
        _ => to_currency,
    };

    // Convert to USD first if needed
    let to_usd = format!("{}/USD", adjusted_from_currency);
    let usd_to_target = if adjusted_to_currency == "USD" {
        1.0
    } else {
        let usd_target = format!("USD/{}", adjusted_to_currency);
        if let Some(&rate) = rate_map.get(&usd_target) {
            rate
        } else {
            println!("⚠️  Warning: No conversion rate found for USD to {}", adjusted_to_currency);
            1.0 // Return 1.0 as fallback
        }
    };

    // Do the conversion
    if let Some(&rate) = rate_map.get(&to_usd) {
        let result = adjusted_amount * rate * usd_to_target;
        // Convert back to pence or cents if needed
        match to_currency {
            "GBp" => result * 100.0,
            "ZAc" => result * 100.0,
            _ => result,
        }
    } else {
        println!("⚠️  Warning: No conversion rate found for {} to {}", adjusted_from_currency, adjusted_to_currency);
        adjusted_amount // Return unconverted amount as fallback
    }
}
