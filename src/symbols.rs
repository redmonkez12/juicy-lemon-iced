use serde::Deserialize;
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
    symbol: String,
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

#[derive(Deserialize, Debug, Clone)]
pub struct Symbol {
    pub symbol: String,
    pub decimals: usize,
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
                .filter(|i| i.status == "TRADING")
                .map(|i| {
                    Symbol {
                        symbol: i.symbol.clone(),
                        decimals: get_decimals(&i),
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
            .collect::<Vec<String>>()
            .join(",")
    );

    match reqwest::get(&url).await {
        Ok(response) => {
            let json = response
                .json::<Vec<SymbolWithPrice>>()
                .await
                .unwrap();
            Ok(json)
        }
        Err(err) => {
            println!("Error: {}", err);
            Err(String::from("Cannot fetch price"))
        }
    }
}
