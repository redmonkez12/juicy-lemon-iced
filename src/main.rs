mod graph;
mod symbols;
mod ui;
mod update;
mod utils;
mod view;

use std::sync::Arc;
use iced::{Size, Subscription, Task, window, Color};
use std::time::Duration;

use crate::symbols::{Symbol, SymbolWithPrice};
use crate::update::update;
use crate::view::view;
use iced::Theme;
use iced::theme::{Custom, Palette};
use iced::widget::{canvas, combo_box};
use iced::time::{self};

#[derive(Debug, Clone)]
enum Message {
    SymbolsFetched(Vec<Symbol>),
    RefetchPrice,
    AddSymbol(String),
    SymbolRemove(String),
    FetchError(String),
    PricesUpdated(Vec<SymbolWithPrice>),
    FilterInput(String),
    UpdateSelectOptions,
    InitApp,
    WindowResized(Size),
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

struct State {
    instruments: Vec<Symbol>,
    watchlist: Vec<WatchListItem>,
    loading: bool,
    input_text: String,
    error_message: String,
    symbol_select_state: combo_box::State<String>,
    selected_symbol: Option<String>,
    width: f32,
    height: f32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            instruments: Vec::new(),
            watchlist: Vec::new(),
            loading: false,
            input_text: String::new(),
            error_message: String::new(),
            symbol_select_state: combo_box::State::default(),
            selected_symbol: None,
            width: 0.0,
            height: 0.0,
        }
    }
}

fn theme(_: &State) -> Theme {
    let custom_theme = Arc::new(Custom::new(
        "My Dark Theme".into(),
        Palette {
            background: [0.012, 0.027, 0.071].into(),
            text: Color::WHITE,
            primary: Color::from_rgb(0.3, 0.6, 0.9),
            success: Color::from_rgb(0.2, 0.8, 0.4),
            danger: Color::from_rgb(0.9, 0.2, 0.2),
        },
    ));

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
        width: 0.0,
        height: 0.0,
    };
    (state, Task::perform(async {}, |_| Message::InitApp))
}

fn subscription(state: &State) -> Subscription<Message> {
    if !state.instruments.is_empty() {
        return time::every(Duration::from_secs(2)).map(|_| Message::RefetchPrice);
    }

    Subscription::none()
}

fn window_resized_subscription(_: &State) -> Subscription<Message> {
    window::resize_events().map(|(_id, size)| Message::WindowResized(size))
}

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application("Juicy Lemon", update, view)
        .theme(theme)
        .subscription(subscription)
        // .subscription(window_resized_subscription)
        .run_with(init)
}
