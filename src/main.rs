use serde::Deserialize;
use std::io;

#[derive(Deserialize, Debug)]
struct ExchangeResponse {
    result: String,
    conversion_rates: std::collections::HashMap<String, f64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(" --- Unit converter (currency) ---- ");

    println!("Enter the base currency (e.g., USD):");
    let mut base_currency = String::new();
    io::stdin().read_line(&mut base_currency)?;
    let base_currency = base_currency.trim().to_uppercase();

    println!("Enter the target currency (e.g., EUR):");
    let mut target_currency = String::new();
    io::stdin().read_line(&mut target_currency)?;
    let target_currency = target_currency.trim().to_uppercase();

    println!("Enter the amount:");
    let mut amount_str = String::new();
    io::stdin().read_line(&mut amount_str)?;
    let amount: f64 = amount_str.trim().parse().unwrap_or(0.0);

    let api_key = "7ad3e8a17666dfcd547acdd0";
    let url = format!(
        "https://v6.exchangerate-api.com/v6/{}/latest/{}",
        api_key, base_currency
    );

    println!("Fetching latest rates...");

    let response = reqwest::get(&url).await?.json::<ExchangeResponse>().await?;

    if response.result == "success" {
        if let Some(rate) = response.conversion_rates.get(&target_currency) {
            let converted_amount = amount * rate;
            println!(
                "{:.2} {} is equal to {:.2} {}",
                amount, base_currency, converted_amount, target_currency
            );
        } else {
            println!("Target currency not found.");
        }
    } else {
        println!("Failed to fetch data from API. Please check your API key or Base Currency.");
    }

    Ok(())
}
