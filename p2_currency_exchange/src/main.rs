use reqwest::blocking::Client;
use std::error::Error;
use std::fs;
use dialoguer::{theme::ColorfulTheme, Select};
use serde_json::Value;


fn main() -> Result<(), Box<dyn Error>> {

    let api_key = fs::read_to_string("src/API.key")
        .expect("Unable to read API key file")
        .trim()  // Remove any surrounding whitespace
        .to_string();

    println!("Currency converter - Akaash Kashyap");
    // Currency Options

    let currencies = vec![
        "USD",
        "EUR",
        "JPY",
        "GBP",
        "AUD",
        "CAD",
        "CHF",
        "CNY",
        "SEK",
        "NZD"
    ];

    // Select Currency
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Currency")
        .items(&currencies)
        .default(0)
        .interact()?;
    // println!("Selected Currency: {}", currencies[selection]);

    let mut input = String::new();
    println!("Enter Amount: ");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<f64>() {
        Ok(amount) => {
            println!("Amount: {}", amount);
        },
        Err(_) => {
            println!("Invalid Amount");
        }
    }
    
    // create a client
    let client = Client::new();
    // constructing url
    let url = format!("https://v6.exchangerate-api.com/v6/{}/latest/{}",&api_key, currencies[selection]);


    // make a request
    let response = client.get(&url)
        .send()?
        .text()?;
    
    // select second currency
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Currency")
        .items(&currencies)
        .default(0)
        .interact()?;
    // println!("Selected Currency: {}", currencies[selection]);

    // Parse the response as JSON
    let json: Value = serde_json::from_str(&response)?;
    // println!("Response: {}", response);

    if let Some(rates) = json.get("conversion_rates") {
        // Extract the base currency rate
        let base_rate = json.get("base_code").unwrap().as_str().unwrap();
        // Extract a specific currency rate, e.g., EUR
        if let Some(convert_curr) = rates.get(currencies[selection]) {
            println!("Exchange rate from {} to {}: {}", &base_rate, currencies[selection], convert_curr);
            println!("Amount in {}: {}", currencies[selection], convert_curr.as_f64().unwrap() * input.trim().parse::<f64>().unwrap());
        } else {
            println!("Rate not found in the response.");
        }
    } else {
        println!("Rates field not found in the response.");
    }

    // println!("Response: {}", response);
    Ok(())
}
