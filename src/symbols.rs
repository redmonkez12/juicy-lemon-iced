use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::graph::candle::Candle;
use crate::utils::get_decimals;

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
    pub decimals: usize,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
struct BinanceKlinesItem {
    open_time: u64,
    open: String,
    high: String,
    low: String,
    close: String,
    volume: String,
    close_time: u64,
    quote_asset_volume: String,
    number_of_trades: u64,
    taker_buy_base_asset_volume: String,
    taker_buy_quote_asset_volume: String,
    ignore: String,
}

impl Symbol {
    pub fn new(symbol: String, decimals: usize) -> Self {
        Self { symbol, decimals }
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
                            decimals: get_decimals(&i),
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

pub async fn fetch_symbol_prices(
    symbols: Vec<String>,
) -> Result<Vec<SymbolWithPrice>, String> {
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
        Ok(response) => {
            match response.text().await {
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
            }
        }
        Err(err) => {
            println!("Error: {}", err);
            Err(String::from("Cannot fetch price"))
        }
    }
}

pub async fn get_candles(symbol: &str) -> Result<Vec<Candle>, String> {
    let url = format!(
        "https://api.binance.com/api/v3/klines?symbol={}&limit=1500&interval=1d",
        symbol
    );

    match reqwest::get(&url).await {
        Ok(response) => match response.json::<Vec<Vec<Value>>>().await {
            Ok(raw_klines) => {
                let candles = raw_klines
                    .into_iter()
                    .filter_map(|entry| {
                        if entry.len() < 6 {
                            return None;
                        }

                        Some(Candle {
                            timestamp: Some(entry[0].as_i64()?),
                            open: entry[1].as_str()?.parse().ok()?,
                            high: entry[2].as_str()?.parse().ok()?,
                            low: entry[3].as_str()?.parse().ok()?,
                            close: entry[4].as_str()?.parse().ok()?,
                        })
                    })
                    .collect::<Vec<Candle>>();

                println!("Fetched {} candles", candles.len());
                
                Ok(candles)
            }
            Err(err) => {
                println!("Error parsing JSON: {}", err);
                Err(String::from("Failed to parse candle JSON"))
            }
        },
        Err(err) => {
            println!("Error fetching candles: {}", err);
            Err(String::from("Failed to fetch candles"))
        }
    }
}