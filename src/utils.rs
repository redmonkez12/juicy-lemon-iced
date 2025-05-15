use crate::symbols::{Symbol};
use std::collections::HashSet;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;

pub fn get_current_select_state(
    instruments: &Vec<Symbol>,
    input: &str,
    watchlist: &Vec<Symbol>,
) -> Vec<String> {
    let lowercase_input = input.to_lowercase();

    let watchlist_symbols: HashSet<String> =
        watchlist.iter().map(|item| item.symbol.clone()).collect();

    let mut sorted_instruments = instruments
        .iter()
        .filter_map(|i| {
            if i.symbol.to_lowercase().contains(&lowercase_input)
                && !watchlist_symbols.contains(&i.symbol)
            {
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
    watchlist: &[Symbol],
) -> Vec<String> {
    let mut sorted_instruments: Vec<&Symbol> = instruments.iter().collect();
    sorted_instruments.sort_by_key(|i| i.symbol.clone());

    let watchlist_symbols: HashSet<&str> =
        watchlist.iter().map(|item| item.symbol.as_str()).collect();

    sorted_instruments
        .into_iter()
        .filter(|i| !watchlist_symbols.contains(i.symbol.as_str()))
        .take(10)
        .map(|i| i.symbol.clone())
        .collect()
}

pub fn calculate_tick_count(min: Decimal, max: Decimal) -> (usize, Decimal) {
    let range = max - min;
    let nice_step = nice_step_from_range(range);
    let tick_min = (min / nice_step).floor() * nice_step;
    let tick_max = (max / nice_step).ceil() * nice_step;
    let tick_count = ((tick_max - tick_min) / nice_step).round().to_usize().unwrap() + 1;

    (tick_count, nice_step)
}

pub fn nice_step_from_range(range: Decimal) -> Decimal {
    if range <= Decimal::ZERO {
        return Decimal::ZERO;
    }

    let range_f64 = range.to_f64().unwrap_or(0.0);
    let exponent_f64 = range_f64.log10().floor();
    let base = Decimal::from_f64(10f64.powf(exponent_f64)).unwrap_or(dec!(1.0));

    let fraction = range / base;

    let nice_fraction = if fraction <= dec!(1.0) {
        dec!(0.1)
    } else if fraction <= dec!(2.0) {
        dec!(0.2)
    } else if fraction <= dec!(5.0) {
        dec!(0.5)
    } else if fraction <= dec!(10.0) {
        dec!(1.0)
    } else if fraction <= dec!(20.0) {
        dec!(2.0)
    } else if fraction <= dec!(50.0) {
        dec!(5.0)
    } else {
        dec!(10.0)
    };

    nice_fraction * base
}

pub fn estimate_y_axis_width(
    tick_start: Decimal,
    tick_count: usize,
    tick_interval: Decimal,
    font_size: f32,
) -> f32 {
    let mut max_label_len = 0;

    for i in 0..tick_count {
        let tick_value = tick_start + Decimal::from(i) * tick_interval;
        let label = format!("{}", tick_value);
        max_label_len = max_label_len.max(label.len());
    }

    let avg_char_width = font_size * 0.6;
        let padding = 10.0;

    max_label_len as f32 * avg_char_width + padding
}
