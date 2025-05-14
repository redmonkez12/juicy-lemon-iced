use iced::Color;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use serde::Deserialize;
use serde_json::Value;

pub const BULL_COLOR: Color = Color::from_rgb(66.0 / 255.0, 149.0 / 255.0, 137.0 / 255.0);
pub const BEAR_COLOR: Color = Color::from_rgb(252.0 / 255.0, 79.0 / 255.0, 111.0 / 255.0);

#[derive(Debug, Clone, Deserialize)]
pub struct Candle {
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub open_time: u64,
    pub close_time: u64,
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

pub async fn get_candles(symbol: &str, timeframe: &str, decimals: u32) -> Result<Vec<Candle>, String> {
    let url = format!(
        "https://api.binance.com/api/v3/klines?symbol={symbol}&limit=100&interval={timeframe}"
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
                            
                            let open_str = entry[1].as_str()?;
                            let high_str = entry[2].as_str()?;
                            let low_str = entry[3].as_str()?;
                            let close_str = entry[4].as_str()?;

                            let mut open = open_str.parse::<Decimal>().ok()?;
                            let mut high = high_str.parse::<Decimal>().ok()?;
                            let mut low = low_str.parse::<Decimal>().ok()?;
                            let mut close = close_str.parse::<Decimal>().ok()?;

                            let _ = open.rescale(decimals);
                            let _ = high.rescale(decimals);
                            let _ = low.rescale(decimals);
                            let _ = close.rescale(decimals);

                            Some(Candle {
                                open_time: entry[0].as_u64()?,
                                close_time: entry[6].as_u64()?,
                                open,
                                high,
                                low,
                                close,
                            })
                        })
                        .collect::<Vec<Candle>>();

                    Ok(candles)
                }
                Err(err) => {
                    println!("Error parsing JSON: {}", err);
                    Err(String::from("Failed to parse candle JSON"))
                }
            }
        }
        Err(err) => {
            println!("Error fetching candles: {}", err);
            Err(String::from("Failed to fetch candles"))
        }
    }
}