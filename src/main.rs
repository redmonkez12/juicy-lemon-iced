mod symbols;
mod ui;
mod update;
mod utils;
mod view;

use std::sync::Arc;
use iced::{Subscription, Task, time, Color};
use std::time::Duration;

use crate::symbols::{Symbol, SymbolWithPrice};
use crate::update::update;
use crate::view::view;
use iced::Theme;
use iced::theme::{palette, Custom, Palette};
use iced::widget::combo_box;

#[derive(Debug, Clone)]
enum Message {
    FetchSymbols,
    SymbolsFetched(Vec<Symbol>),
    RefetchPrice,
    AddSymbol(String),
    SymbolRemove(String),
    FetchError(String),
    PricesUpdated(Vec<SymbolWithPrice>),
    FilterInput(String),
    UpdateSelectOptions,
}

#[derive(Default)]
struct WatchListItem {
    price: String,
    symbol: String,
    decimals: usize,
}

impl WatchListItem {
    fn new(symbol: String, price: String, decimals: usize) -> Self {
        Self {
            symbol,
            price,
            decimals,
        }
    }
}

#[derive(Default)]
struct State {
    instruments: Vec<Symbol>,
    watchlist: Vec<WatchListItem>,
    loading: bool,
    input_text: String,
    error_message: String,
    symbol_select_state: combo_box::State<String>,
    selected_symbol: Option<String>,
}

fn theme(_: &State) -> Theme {
    let custom_theme = Arc::new(Custom::new("My Dark Theme".into(), Palette {
        background: [0.012, 0.027, 0.071].into(),
        text: Color::WHITE,
        primary: Color::from_rgb(0.3, 0.6, 0.9),
        success: Color::from_rgb(0.2, 0.8, 0.4),
        danger: Color::from_rgb(0.9, 0.2, 0.2),
    }));

    Theme::Custom(custom_theme)
}

fn init() -> (State, Task<Message>) {
    let state = State {
        instruments: Vec::new(),
        watchlist: Vec::new(),
        error_message: "".to_string(),
        input_text: "".to_string(),
        loading: true,
        selected_symbol: None,
        symbol_select_state: combo_box::State::default(),
    };
    (state, Task::perform(async {}, |_| Message::FetchSymbols))
}

fn subscription(state: &State) -> Subscription<Message> {
    if !state.instruments.is_empty() {
        return time::every(Duration::from_secs(2)).map(|_| Message::RefetchPrice);
    }

    Subscription::none()
}

fn main() -> iced::Result {
    iced::application("Juicy Lemon", update, view)
        .theme(theme)
        .subscription(subscription)
        .run_with(init)
}
