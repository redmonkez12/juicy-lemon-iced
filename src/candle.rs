use iced::Color;
use serde::Deserialize;
use serde_json::Value;

pub const BULL_COLOR: Color = Color::from_rgb(66.0 / 255.0, 149.0 / 255.0, 137.0 / 255.0);
pub const BEAR_COLOR: Color = Color::from_rgb(252.0 / 255.0, 79.0 / 255.0, 111.0 / 255.0);

#[derive(Debug, Clone, Deserialize)]
pub struct Candle {
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
}

impl Candle {
    pub fn get_color(&self) -> Color {
        if self.close > self.open {
            BULL_COLOR
        } else {
            BEAR_COLOR
        }
    }
}

pub async fn get_candles(symbol: &str) -> Result<Vec<Candle>, String> {
    let url = format!(
        "https://api.binance.com/api/v3/klines?symbol={}&limit=100&interval=1d",
        symbol
    );

    match reqwest::get(&url).await {
        Ok(response) => {
            match response.json::<Vec<Vec<Value>>>().await {
                Ok(raw_klines) => {
                    let candles = raw_klines
                        .into_iter()
                        .filter_map(|entry| {
                            if entry.len() < 12 {
                                return None;
                            }

                            Some(Candle {
                                // open_time: entry[0].as_u64()?,
                                open: entry[1].as_str()?.parse::<f32>().ok()?,
                                high: entry[2].as_str()?.parse::<f32>().ok()?,
                                low: entry[3].as_str()?.parse::<f32>().ok()?,
                                close: entry[4].as_str()?.parse::<f32>().ok()?,
                                // volume: entry[5].as_str()?.to_string(),
                                // close_time: entry[6].as_u64()?,
                                // quote_asset_volume: entry[7].as_str()?.to_string(),
                                // number_of_trades: entry[8].as_u64()?,
                                // taker_buy_base_asset_volume: entry[9].as_str()?.to_string(),
                                // taker_buy_quote_asset_volume: entry[10].as_str()?.to_string(),
                                // ignore: entry[11].as_str()?.to_string(),
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
            }
        },
        Err(err) => {
            println!("Error fetching candles: {}", err);
            Err(String::from("Failed to fetch candles"))
        }
    }
}