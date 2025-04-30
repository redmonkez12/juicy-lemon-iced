use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Deserialize)]
struct ExchangeInfo {
    symbols: Vec<SymbolInfo>,
}

#[derive(Deserialize)]
struct SymbolInfo {
    status: String,
    symbol: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.binance.com/api/v3/exchangeInfo";
    let response = get(url)?.json::<ExchangeInfo>()?;

    let tradable_pairs: Vec<String> = response
        .symbols
        .into_iter()
        .filter(|s| s.status == "TRADING")
        .map(|s| s.symbol)
        .collect();

    println!("Tradable pairs: {:?}", tradable_pairs);
    
    Ok(())
}
