use crate::symbols::{fetch_symbol_prices, get_symbols};
use crate::{Message, State, WatchListItem};
use iced::Task;
use iced::widget::combo_box;
use tokio::runtime::Runtime;

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::FilterInput(input) => {
            println!("Input text: {}", input);

            let filtered_options = state
                .instruments
                .iter()
                .filter(|i| i.symbol.to_lowercase().contains(&input.to_lowercase()))
                .map(|i| i.symbol.clone())
                .collect::<Vec<_>>();

            state.symbol_select_state = combo_box::State::with_selection(filtered_options, Some(&input));
            state.input_text = input;

            Task::none()
        }
        Message::SymbolRemove(symbol) => {
            state.watchlist.retain(|w| w.symbol != symbol);
            Task::none()
        }
        Message::FetchError(error) => {
            println!("Error fetching prices: {}", error);
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
            if let Some(valid_symbol) = state.instruments.iter().find(|i| i.symbol == symbol)
            {
                let rt = Runtime::new().unwrap();
                let prices = rt
                    .block_on(fetch_symbol_prices(vec![valid_symbol.symbol.clone()]))
                    .unwrap();

                if let Some(instrument_response) = prices.get(0) {
                    state.watchlist.push(WatchListItem::new(
                        symbol,
                        instrument_response.price.clone(),
                        valid_symbol.decimals,
                    ));

                    state.watchlist.sort_by(|a, b| a.symbol.cmp(&b.symbol));
                }

                state.input_text = "".to_string();
                state.error_message = "".to_string();
            } else {
                state.error_message = "Invalid symbol".to_string();
            }

            Task::none()
        }
        Message::FetchSymbols => {
            println!("Fetching symbols");
            let rt = Runtime::new().unwrap();
            state.loading = true;
            Task::perform(async { get_symbols().await }, Message::SymbolsFetched)
        }
        Message::SymbolsFetched(Ok(instruments)) => {
            println!("Symbols fetched: {} instruments", instruments.len());
            state.instruments = instruments.clone();
            let mut sorted_instruments = instruments.clone();
            sorted_instruments.sort_by_key(|i| i.symbol.clone());

            let select_state = combo_box::State::new(
                sorted_instruments
                    .iter()
                    .take(10)
                    .map(|i| i.symbol.clone())
                    .collect(),
            );

            state.symbol_select_state = select_state;
            state.loading = false;
            Task::none()
        }
        Message::SymbolsFetched(Err(error)) => {
            println!("Error fetching symbols: {}", error);
            state.loading = false;
            Task::none()
        }
    }
}
