#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Candle {
    pub open: f64,
    pub close: f64,
    pub low: f64,
    pub high: f64,
    pub timestamp: Option<i64>,
}

pub enum CandleType {
    Bearish,
    Bullish,
}

impl Candle {
    pub fn new(
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        timestamp: Option<i64>,
    ) -> Candle {
        Candle {
            open,
            high,
            low,
            close,
            timestamp,
        }
    }

    pub fn get_type(&self) -> CandleType {
        match self.open < self.close {
            true => CandleType::Bullish,
            false => CandleType::Bearish,
        }
    }
}
