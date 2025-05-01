mod symbols;
use iced::Task;

use crate::symbols::get_symbols;
use iced::widget::{Column, text, Row, Scrollable, button, text_input};
use iced::{Element, Theme};
use tokio::runtime::Runtime;

#[derive(Debug, Clone)]
enum Message {
    FetchSymbols,
    SymbolsFetched(Result<Vec<String>, String>),
    AddSymbol,
    SymbolChanged(String),
}

#[derive(Default)]
struct State {
    instruments: Vec<String>,
    watchlist: Vec<String>,
    loading: bool,
    symbol: String,
}

fn theme(_: &State) -> Theme {
    Theme::Dark
}

fn view(state: &State) -> Element<Message> {
    if state.loading {
        return text("Loading...").size(20).into();
    }

    if state.instruments.is_empty() {
        return text("Nothing found").size(20).into();
    }

    let input_row = Row::new()
        .spacing(10)
        .push(text_input("Add instrument...", &state.symbol).on_input(Message::SymbolChanged))
        .push(button(text("Add to watchlist")).on_press(Message::AddSymbol));

    let mut watch_list = Column::new().spacing(10);

    for item in &state.watchlist {
        let row = Row::new().push(text(item).size(20));
        watch_list = watch_list.push(row);
    }

    Column::new()
        .spacing(20)
        .padding(20)
        .push(input_row)
        .push(Scrollable::new(watch_list))
        .into()
}


fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::SymbolChanged(symbol) => {
            state.symbol = symbol;
            Task::none()
        }
        Message::AddSymbol => {
            state.watchlist.push(state.symbol.clone());
            state.symbol = "".to_string();
            Task::none()
        },
        Message::FetchSymbols => {
            let rt = Runtime::new().unwrap();
            println!("Fetching symbols");
            state.loading = true;
            Task::perform(
                async move { rt.block_on(get_symbols()) },
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

fn init() -> (State, Task<Message>) {
    let state = State {
        instruments: Vec::new(),
        watchlist: Vec::new(),
        symbol: "".to_string(),
        loading: true,
    };
    let rt = Runtime::new().unwrap();

    (
        state,
        Task::perform(
            async move { rt.block_on(get_symbols()) },
            Message::SymbolsFetched,
        ),
    )
}

fn main() -> iced::Result {
    iced::application("Juicy Lemon", update, view)
        .theme(theme)
        .run_with(init)
}