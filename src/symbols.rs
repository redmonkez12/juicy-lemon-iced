use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Instrument {
    status: String,
    symbol: String,
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

pub async fn get_symbols() -> Result<Vec<String>, String> {
    match reqwest::get("https://api.binance.com/api/v3/exchangeInfo").await {
        Ok(response) => {
            let json = response.json::<Response>().await.unwrap();
            let symbols = json
                .symbols
                .into_iter()
                .filter(|i| i.status == "TRADING")
                .map(|i| i.symbol)
                .collect();
            Ok(symbols)
        }
        Err(err) => {
            println!("Error: {}", err);
            Err(String::from("Cannot fetch instruments"))
        }
    }
}

pub async fn fetch_symbol_prices(symbols: Vec<String>) -> Result<Vec<InstrumentPriceResponse>, String> {
    println!("Fetching symbols {}", symbols.join(", "));
    
    let url = format!(
        "https://www.binance.com/api/v3/ticker/price?symbols=[{}]",
        symbols.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(",")
    );
    
    match reqwest::get(&url).await {
        Ok(response) => {
            let json = response.json::<Vec<InstrumentPriceResponse>>().await.unwrap();
            Ok(json)
        }
        Err(err) => {
            println!("Error: {}", err);
            Err(String::from("Cannot fetch price"))
        }
    }
}
