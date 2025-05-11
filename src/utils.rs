use std::collections::HashSet;
use crate::symbols::{Instrument, Symbol};
use crate::WatchListItem;

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

pub fn get_current_select_state(
    instruments: &Vec<Symbol>,
    input: &str,
    watchlist: &Vec<WatchListItem>,
) -> Vec<String> {
    let lowercase_input = input.to_lowercase();

    let watchlist_symbols: HashSet<String> = watchlist.iter()
        .map(|item| item.symbol.clone())
        .collect();

    let mut sorted_instruments = instruments
        .iter()
        .filter_map(|i| {
            if i.symbol.to_lowercase().contains(&lowercase_input)
                && !watchlist_symbols.contains(&i.symbol) {
                Some(i.symbol.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    sorted_instruments.sort();

    sorted_instruments
}

pub fn get_default_select_state(
    instruments: &[Symbol],
    watchlist: &[WatchListItem],
) -> Vec<String> {
    let mut sorted_instruments: Vec<&Symbol> = instruments.iter().collect();
    sorted_instruments.sort_by_key(|i| i.symbol.clone());

    let watchlist_symbols: HashSet<&str> = watchlist.iter()
        .map(|item| item.symbol.as_str())
        .collect();

    sorted_instruments
        .into_iter()
        .filter(|i| !watchlist_symbols.contains(i.symbol.as_str()))
        .take(10)
        .map(|i| i.symbol.clone())
        .collect()
}

pub fn calculate_tick_count(min: f32, max: f32) -> (usize, f32) {
    let range = max - min;
    let nice_step = nice_step_from_range(range);
    let tick_min = (min / nice_step).floor() * nice_step;
    let tick_max = (max / nice_step).ceil() * nice_step;
    let tick_count = ((tick_max - tick_min) / nice_step).round() as usize + 1;

    (tick_count, nice_step)
}

pub fn nice_step_from_range(range: f32) -> f32 {
    let exponent = range.log10().floor();
    let base = 10f32.powf(exponent);

    let fraction = range / base;

    let nice_fraction = if fraction <= 1.0 {
        0.1
    } else if fraction <= 2.0 {
        0.2
    } else if fraction <= 5.0 {
        0.5
    } else if fraction <= 10.0 {
        1.0
    } else if fraction <= 20.0 {
        2.0
    } else if fraction <= 50.0 {
        5.0
    } else {
        10.0
    };

    nice_fraction * base
}
