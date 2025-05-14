use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Filter {
    #[serde(rename = "filterType")]
    pub filter_type: String,

    #[serde(rename = "tickSize")]
    pub tick_size: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Instrument {
    status: String,
    #[serde(rename = "baseAssetPrecision")]
    base_asset_precision: i32,
    pub symbol: String,
    pub filters: Vec<Filter>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SymbolWithPrice {
    pub symbol: String,
    pub price: String,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    symbols: Vec<Instrument>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Symbol {
    pub symbol: String,
    pub price: Option<Decimal>,
    pub decimals: u32,
}

impl Symbol {
    fn new(symbol: String, price: Decimal, decimals: u32) -> Self {
        Self {
            symbol,
            price: Some(price),
            decimals,
        }
    }
}

pub async fn get_symbols() -> Result<Vec<Symbol>, String> {
    match reqwest::get("https://api.binance.com/api/v3/exchangeInfo").await {
        Ok(response) => {
            let json = response.json::<Response>().await.unwrap();
            let symbols = json
                .symbols
                .into_iter()
                .filter_map(|i| {
                    if i.status == "TRADING" {
                        Some(Symbol {
                            symbol: i.symbol.clone(),
                            decimals: i.base_asset_precision as u32,
                            price: None,
                        })
                    } else {
                        None
                    }
                })
                .collect();
            Ok(symbols)
        }
        Err(err) => {
            println!("Error: {}", err);
            Err(String::from("Cannot fetch instruments"))
        }
    }
}

pub async fn fetch_symbol_prices(symbols: Vec<String>) -> Result<Vec<SymbolWithPrice>, String> {
    let url = format!(
        "https://www.binance.com/api/v3/ticker/price?symbols=[{}]",
        symbols
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(",")
    );

    println!("Fetching prices: {}", url);

    match reqwest::get(&url).await {
        Ok(response) => match response.text().await {
            Ok(body) => {
                println!("Response body: {}", body);

                match serde_json::from_str::<Vec<SymbolWithPrice>>(&body) {
                    Ok(json) => Ok(json),
                    Err(err) => {
                        println!("Error parsing JSON: {}", err);
                        Err(String::from("Failed to parse JSON"))
                    }
                }
            }
            Err(err) => {
                println!("Error reading response body: {}", err);
                Err(String::from("Failed to read response body"))
            }
        },
        Err(err) => {
            println!("Error: {}", err);
            Err(String::from("Cannot fetch price"))
        }
    }
}
