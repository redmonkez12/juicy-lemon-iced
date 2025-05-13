use crate::graph::candle::get_candles;
use crate::symbols::{Symbol, fetch_symbol_prices, get_symbols};
use crate::utils::{get_current_select_state, get_default_select_state};
use crate::{DisplayedSymbol, Message, State, WatchListItem};
use iced::Task;
use iced::widget::combo_box;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::ChangeTimeframe(timeframe) => {
            state.selected_timeframe = Some(timeframe.clone());

            if let Some(displayed_symbol) = state.displayed_symbol.as_mut() {
                displayed_symbol.timeframe = timeframe.clone();

                let symbol = displayed_symbol.symbol.clone();
                let tf = displayed_symbol.timeframe.clone();
                let decimals = displayed_symbol.decimals;

                Task::perform(
                    async move {
                        match get_candles(&symbol, &tf, decimals).await {
                            Ok(candles) => Message::CandlesFetched(candles, symbol),
                            Err(err) => Message::FetchError(err),
                        }
                    },
                    |msg| msg,
                )
            } else {
                Task::none()
            }
        }
        Message::SelectSymbol(symbol) => {
            if let Some(displayed_symbol) = &state.displayed_symbol {
                if displayed_symbol.symbol == symbol {
                    return Task::none();
                }
            }

            let timeframe = match state.selected_timeframe.as_ref() {
                Some(timeframe) => timeframe.clone(),
                None => return Task::none(),
            };

            let instrument = state
                .instruments
                .iter()
                .find(|s| s.symbol == symbol)
                .unwrap();

            state.displayed_symbol = Some(DisplayedSymbol {
                timeframe: timeframe.clone(),
                symbol: symbol.clone(),
                decimals: instrument.decimals,
            });

            let decimals = state.displayed_symbol.as_ref().map(|s| s.decimals).unwrap_or(8);

            Task::perform(
                async move {
                    match get_candles(&symbol, &timeframe, decimals).await {
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
                let symbol_entry = state
                    .candles
                    .entry(symbol.clone())
                    .or_insert_with(HashMap::new);
                let old_candles = symbol_entry
                    .entry(timeframe.clone())
                    .or_insert_with(VecDeque::new);
                
                // if old_candles.is_empty() {
                //     *old_candles = VecDeque::from(candles.clone());
                // }

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
                if let (Some(symbol), Some(timeframe)) =
                    (&state.displayed_symbol, &state.selected_timeframe)
                {
                    if let Some(symbol_map) = state.candles.get_mut(symbol.symbol.as_str()) {
                        for timeframe in state.timeframe_select_state.options().iter() {
                            symbol_map.remove(timeframe);       
                        }
                    }
                }
                
                state.graph.clear();
                return Task::perform(async {}, |_| Message::UpdateSelectOptions);
            }

            if let Some(displayed_symbol) = &state.displayed_symbol {
                if Some(symbol) == Some(displayed_symbol.symbol.clone()) {
                    let new_symbol = state.watchlist.first().unwrap().symbol.clone();
                    return Task::perform(async {}, move |_| {
                        Message::SelectSymbol(new_symbol.clone())
                    });
                }
            } else {
                return Task::none();
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

            let symbol = match state.displayed_symbol.as_ref() {
                Some(s) => s.clone(),
                None => return Task::none(),
            };

            let timeframe = match state.selected_timeframe.as_ref() {
                Some(tf) => tf.clone(),
                None => return Task::none(),
            };

            Task::batch(vec![
                Task::perform(
                    {
                        let symbol_str = symbol.symbol.clone();
                        let timeframe_str = timeframe.clone();

                        async move {
                            match get_candles(&symbol_str, &timeframe_str, symbol.decimals).await {
                                Ok(candles) => Message::CandlesFetched(candles, symbol_str),
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

            let instrument = state
                .instruments
                .iter()
                .find(|s| s.symbol == symbol)
                .unwrap();

            state.watchlist.push(WatchListItem::new(
                symbol.clone(),
                "-9999".to_string(),
                instrument.decimals,
            ));

            state.watchlist.sort_by(|a, b| a.symbol.cmp(&b.symbol));

            state.input_text = "".to_string();
            state.error_message = "".to_string();

            let timeframe = match state.selected_timeframe.as_ref() {
                Some(timeframe) => timeframe.clone(),
                None => return Task::none(),
            };

            state.displayed_symbol = Some(DisplayedSymbol {
                timeframe: timeframe.clone(),
                symbol: symbol.clone(),
                decimals: instrument.decimals,
            });

            Task::perform(
                {
                    let symbol = symbol.clone();
                    let timeframe = timeframe.clone();
                    let decimals = instrument.decimals.clone();

                    async move {
                        match get_candles(&symbol, &timeframe, decimals).await {
                            Ok(candles) => Message::CandlesFetched(candles, symbol),
                            Err(err) => Message::FetchError(err),
                        }
                    }
                },
                |msg| msg,
            )
        }
        Message::InitApp => {
            state.loading = true;

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
