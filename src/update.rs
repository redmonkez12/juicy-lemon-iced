use crate::symbols::{fetch_symbol_prices, get_symbols};
use crate::{Message, State, WatchListItem};
use iced::Task;
use tokio::runtime::Runtime;

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::FetchError(error) => {
            println!("Error fetching prices: {}", error);
            Task::none()
        },
        Message::PricesUpdated(prices) => {
            for item in state.watchlist.iter_mut() {
                if let Some(instrument_response) = prices.iter().find(|p| p.symbol == item.symbol) {
                    item.price = instrument_response.price.clone();
                }
            }

            Task::none()
        },
        Message::RefetchPrice => {
            println!("Refetching price");
            if state.watchlist.is_empty() {
                return Task::none();
            }

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
        },
        Message::SymbolChanged(symbol) => {
            println!("Symbol changed: {}", symbol);
            state.symbol = symbol;
            Task::none()
        }
        Message::AddSymbol => {
            println!("Symbol added");
            if let Some(valid_symbol) = state.instruments.iter().find(|i| i.symbol == state.symbol) {
                let rt = Runtime::new().unwrap();
                let prices = rt
                    .block_on(fetch_symbol_prices(vec![valid_symbol.symbol.clone()]))
                    .unwrap();

                if let Some(instrument_response) = prices.get(0) {
                    state.watchlist.push(WatchListItem::new(
                        state.symbol.clone(),
                        instrument_response.price.clone(),
                        valid_symbol.decimals,
                    ));
                }

                state.symbol = "".to_string();
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
            Task::perform(
                async { get_symbols().await },
                Message::SymbolsFetched,
            )
        }
        Message::SymbolsFetched(Ok(instruments)) => {
            println!("Symbols fetched: {} instruments", instruments.len());
            state.instruments = instruments;
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
