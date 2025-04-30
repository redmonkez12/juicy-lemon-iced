use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Instrument {
    status: String,
    symbol: String,
}

#[derive(Deserialize, Debug)]
struct Response {
    symbols: Vec<Instrument>,
}

async fn get_symbols() -> Result<Vec<String>, String> {
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

#[tokio::main]
async fn main() {
    match get_symbols().await {
        Ok(instruments) => {
            for symbol in instruments {
                println!("{}", symbol);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
