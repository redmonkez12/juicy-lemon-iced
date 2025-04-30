use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Instrument {
    status: String,
    symbol: String,
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
