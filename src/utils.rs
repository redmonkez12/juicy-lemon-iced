use crate::symbols::Instrument;

pub fn get_decimals(instrument: &Instrument) -> usize {
    let mut decimals: usize = 8;
    if let Some(found_decimals) = instrument.filters.iter().find_map(|f| {
        if f.filter_type == "PRICE_FILTER" {
            let decimal_size = f.tick_size
                .as_deref()
                .and_then(|s| s.parse::<f64>().ok())
                .map(|n| {
                    n.to_string().split('.').nth(1).map_or(0, |frac| frac.len())
                })
                .unwrap_or(0);

            Some(decimal_size)
        } else {
            None
        }
    }) {
        decimals = found_decimals;
    }
    
    decimals
}
