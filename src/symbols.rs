use serde::Deserialize;

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
    filters: Vec<Filter>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InstrumentPriceResponse {
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
                    let mut decimals: usize = 8;
                    if let Some(found_decimals) = i.filters.iter().find_map(|f| {
                        if f.filter_type == "PRICE_FILTER" {
                            let decimal_size = f.tick_size
                                .as_deref()
                                .and_then(|s| s.parse::<f64>().ok())
                                .map(|n| {
                                    let s = format!("{}", n);
                                    s.split('.').nth(1).map_or(0, |frac| frac.len())
                                })
                                .unwrap_or(0);
                            
                            Some(decimal_size)
                        } else {
                            None
                        }
                    }) {
                        decimals = found_decimals;
                    }

                    Symbol {
                        symbol: i.symbol.clone(),
                        decimals,
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
) -> Result<Vec<InstrumentPriceResponse>, String> {
    println!("Fetching symbols {}", symbols.join(", "));

    let url = format!(
        "https://www.binance.com/api/v3/ticker/price?symbols=[{}]",
        symbols
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(",")
    );

    match reqwest::get(&url).await {
        Ok(response) => {
            let json = response
                .json::<Vec<InstrumentPriceResponse>>()
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
