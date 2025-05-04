use crate::symbols::{fetch_symbol_prices, get_symbols};
use crate::utils::{get_current_select_state, get_default_select_state};
use crate::{Message, State, WatchListItem};
use iced::Task;
use iced::widget::combo_box;

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::UpdateSelectOptions => {
            let mut options: Vec<String> = if state.input_text.is_empty() {
                get_default_select_state(state.instruments.clone(), &state.watchlist)
            } else {
                get_current_select_state(&state.instruments, &state.input_text, &state.watchlist)
            };

            if options.is_empty() {
                options.push(format!("There are no results for - {}", state.input_text).to_string());
            }

            state.symbol_select_state = combo_box::State::with_selection(options, Some(&state.input_text));
            Task::none()
        }
        Message::FilterInput(input) => {
            println!("Input text: {}", input);
            state.input_text = input;

            Task::perform(async {}, |_| Message::UpdateSelectOptions)
        }
        Message::SymbolRemove(symbol) => {
            state.watchlist.retain(|w| w.symbol != symbol);
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
        Message::RefetchPrice => {
            if state.watchlist.is_empty() {
                return Task::none();
            }

            println!("Refetching price");

            let symbols = state.watchlist.iter().map(|s| s.symbol.clone()).collect();

            Task::perform(
                async move {
                    match fetch_symbol_prices(symbols).await {
                        Ok(prices) => Message::PricesUpdated(prices),
                        Err(err) => Message::FetchError(err.to_string()),
                    }
                },
                |msg| msg,
            )
        }
        Message::AddSymbol(symbol) => {
            println!("Symbol added");
            if symbol.contains("There are no results for") {
                state.input_text = "".to_string();
                state.error_message = "".to_string();
                return Task::perform(async {}, |_| Message::UpdateSelectOptions);
            }

            state.watchlist.push(WatchListItem::new(
                symbol,
                "-9999".to_string(),
                8,
            ));

            state.watchlist.sort_by(|a, b| a.symbol.cmp(&b.symbol));

            state.input_text = "".to_string();
            state.error_message = "".to_string();

            Task::perform(async {}, |_| Message::UpdateSelectOptions)
        }
        Message::FetchSymbols => {
            println!("Fetching symbols");
            state.loading = true;
            Task::perform(
                async move {
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
            Task::perform(async {}, |_| Message::UpdateSelectOptions)
        }
    }
}
