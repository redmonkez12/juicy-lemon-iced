use crate::candle::get_candles;
use crate::symbols::{Symbol, fetch_symbol_prices, get_symbols};
use crate::utils::{get_current_select_state, get_default_select_state};
use crate::{Message, State, WatchListItem};
use iced::Task;
use iced::widget::combo_box;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::ChangeTimeframe(timeframe) => {
            state.selected_timeframe = Some(timeframe);

            let symbol = state.displayed_symbol.as_ref().cloned().unwrap();
            let timeframe = state.selected_timeframe.as_ref().cloned().unwrap();

            Task::perform(
                async move {
                    match get_candles(&symbol, &timeframe).await {
                        Ok(candles) => Message::CandlesFetched(candles, symbol),
                        Err(err) => Message::FetchError(err),
                    }
                },
                |msg| msg,
            )
        }
        Message::SelectSymbol(symbol) => {
            if state.displayed_symbol == Some(symbol.clone()) {
                return Task::none();
            }

            state.graph.clear();
            state.displayed_symbol = Some(symbol.clone());

            println!("Symbol selected: {}", symbol);

            let timeframe = state.selected_timeframe.as_ref().cloned().unwrap();

            Task::perform(
                async move {
                    match get_candles(&symbol, &timeframe).await {
                        Ok(candles) => Message::CandlesFetched(candles, symbol),
                        Err(err) => Message::FetchError(err),
                    }
                },
                |msg| msg,
            )
        }
        Message::CandlesFetched(candles, symbol) => {
            state.graph.clear();
            
            if let Some(timeframe) = state.selected_timeframe.clone() {
                let symbol_entry = state.candles.entry(symbol.clone()).or_insert_with(HashMap::new);
                let old_candles = symbol_entry.entry(timeframe.clone()).or_insert_with(VecDeque::new);

                if let (Some(last_old), Some(last_new)) = (old_candles.back(), candles.last()) {
                    if last_old.open_time == last_new.open_time {
                        old_candles.pop_back();
                        old_candles.push_back(last_new.clone());
                    } else {
                        old_candles.push_back(last_new.clone());
                        old_candles.pop_front();
                    }
                } else {
                    *old_candles = VecDeque::from(candles.clone());
                }
            }

            Task::perform(async {}, |_| Message::UpdateSelectOptions)
        }
        Message::UpdateSelectOptions => {
            let mut options: Vec<String> = if state.input_text.is_empty() {
                get_default_select_state(&state.instruments, &state.watchlist)
            } else {
                get_current_select_state(&state.instruments, &state.input_text, &state.watchlist)
            };

            if options.is_empty() {
                options
                    .push(format!("There are no results for - {}", state.input_text).to_string());
            }

            state.symbol_select_state =
                combo_box::State::with_selection(options, Some(&state.input_text));

            Task::none()
        }
        Message::FilterInput(input) => {
            println!("Input text: {}", input);
            state.input_text = input;

            Task::perform(async {}, |_| Message::UpdateSelectOptions)
        }
        Message::SymbolRemove(symbol) => {
            state.watchlist.retain(|w| w.symbol != symbol);

            if state.watchlist.is_empty() {
                return Task::perform(async {}, |_| Message::UpdateSelectOptions);
            }

            if Some(symbol) == state.displayed_symbol {
                let new_symbol = state.watchlist.first().unwrap().symbol.clone();
                return Task::perform(async {}, move |_| Message::SelectSymbol(new_symbol.clone()));
            }

            Task::perform(async {}, |_| Message::UpdateSelectOptions)
        }
        Message::FetchError(error) => {
            println!("Fetching error: {}", error);
            state.error_message = "Something went wrong".to_string();
            state.loading = false;
            Task::none()
        }
        Message::PricesUpdated(prices) => {
            for item in state.watchlist.iter_mut() {
                if let Some(instrument_response) = prices.iter().find(|p| p.symbol == item.symbol) {
                    item.price = instrument_response.price.clone();
                }
            }

            Task::none()
        }
        Message::RefetchData => {
            if state.watchlist.is_empty() {
                return Task::none();
            }

            println!("Refetching price");

            let symbols: Vec<String> = state.watchlist.iter().map(|s| s.symbol.clone()).collect();

            Task::batch(vec![
                Task::perform(
                    {
                        let symbol = state.displayed_symbol.as_ref().cloned().unwrap();
                        let timeframe = state.selected_timeframe.as_ref().cloned().unwrap();

                        async move {
                            match get_candles(&symbol, &timeframe).await {
                                Ok(candles) => Message::CandlesFetched(candles, symbol),
                                Err(err) => Message::FetchError(err),
                            }
                        }
                    },
                    |msg| msg,
                ),
                Task::perform(
                    {
                        let symbols = symbols.clone();
                        async move {
                            match fetch_symbol_prices(symbols).await {
                                Ok(prices) => Message::PricesUpdated(prices),
                                Err(err) => Message::FetchError(err.to_string()),
                            }
                        }
                    },
                    |msg| msg,
                ),
            ])
        }
        Message::AddSymbol(symbol) => {
            println!("Symbol added");
            if symbol.contains("There are no results for") {
                state.input_text = "".to_string();
                state.error_message = "".to_string();

                return Task::perform(async {}, |_| Message::UpdateSelectOptions);
            }

            state.displayed_symbol = Some(symbol.clone());

            let instrument = state
                .instruments
                .iter()
                .find(|s| s.symbol == symbol)
                .unwrap();

            state.watchlist.push(WatchListItem::new(
                symbol,
                "-9999".to_string(),
                instrument.decimals,
            ));

            state.watchlist.sort_by(|a, b| a.symbol.cmp(&b.symbol));

            state.input_text = "".to_string();
            state.error_message = "".to_string();

            let symbol = match state.watchlist.last() {
                Some(watchitem) => watchitem.symbol.clone(),
                None => return Task::none(),
            };

            let timeframe = state.selected_timeframe.as_ref().cloned().unwrap();

            Task::perform(
                async move {
                    match get_candles(&symbol, &timeframe).await {
                        Ok(candles) => Message::CandlesFetched(candles, symbol),
                        Err(err) => Message::FetchError(err),
                    }
                },
                |msg| msg,
            )
        }
        Message::InitApp => {
            state.loading = true;
            // let csv_content = fs::read_to_string("data/btcusd.csv").unwrap();
            // let candles = parse_csv_data(&csv_content);
            // let chart = Chart::new(&candles);
            // state.chart = chart;

            Task::perform(
                async move {
                    let cached: Option<Vec<Symbol>> = fs::read_to_string("symbols.json")
                        .ok()
                        .and_then(|data| serde_json::from_str(&data).ok());

                    if let Some(symbols) = cached {
                        if !symbols.is_empty() {
                            println!("Loaded symbols from file");
                            return Message::SymbolsFetched(symbols);
                        }
                    }

                    match get_symbols().await {
                        Ok(symbols) => Message::SymbolsFetched(symbols),
                        Err(err) => Message::FetchError(err.to_string()),
                    }
                },
                |msg| msg,
            )
        }
        Message::SymbolsFetched(instruments) => {
            println!("Symbols fetched: {} instruments", instruments.len());
            state.instruments = instruments.clone();
            state.loading = false;

            if let Ok(json) = serde_json::to_string_pretty(&instruments) {
                if let Ok(mut file) = File::create("symbols.json") {
                    if let Err(e) = file.write_all(json.as_bytes()) {
                        eprintln!("Failed to write instruments to file: {}", e);
                    }
                } else {
                    eprintln!("Failed to create file for instruments");
                }
            } else {
                eprintln!("Failed to serialize instruments to JSON");
            }

            Task::perform(async {}, |_| Message::UpdateSelectOptions)
        }
    }
}
