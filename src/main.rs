mod symbols;
mod update;
mod view;

use std::time::Duration;
use iced::{time, Subscription, Task};

use crate::update::update;
use crate::view::view;
use iced::Theme;
use crate::symbols::InstrumentPriceResponse;

#[derive(Debug, Clone)]
enum Message {
    FetchSymbols,
    SymbolsFetched(Result<Vec<String>, String>),
    RefetchPrice,
    AddSymbol,
    SymbolChanged(String),
    FetchError(String),
    PricesUpdated(Vec<InstrumentPriceResponse>),
}

#[derive(Default)]
struct WatchListItem {
    price: String,
    symbol: String,
}

impl WatchListItem {
    fn new(symbol: String, price: String) -> Self {
        Self {
            symbol,
            price,
        }
    }
}

#[derive(Default)]
struct State {
    instruments: Vec<String>,
    watchlist: Vec<WatchListItem>,
    loading: bool,
    symbol: String,
}

fn theme(_: &State) -> Theme {
    Theme::Dark
}

fn init() -> (State, Task<Message>) {
    let state = State {
        instruments: Vec::new(),
        watchlist: Vec::new(),
        symbol: "".to_string(),
        loading: true,
    };
    (state, Task::perform(async {}, |_| Message::FetchSymbols))
}

fn subscription(state: &State) -> Subscription<Message> {
    if !state.instruments.is_empty() {
        return time::every(Duration::from_secs(1)).map(|_| Message::RefetchPrice);
    }

    Subscription::none()
}

fn main() -> iced::Result {
    iced::application("Juicy Lemon", update, view)
        .theme(theme)
        .subscription(subscription)
        .run_with(init)
}
