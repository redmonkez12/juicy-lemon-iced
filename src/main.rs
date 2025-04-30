mod symbols;
use iced::Task;

use crate::symbols::get_symbols;
use iced::widget::container;
use iced::{Element, Theme};

#[derive(Debug)]
enum Message {
    FetchSymbols,
    SymbolsFetched(Result<Vec<String>, String>),
}

#[derive(Default)]
struct State {
    instruments: Vec<String>,
}

fn theme(_: &State) -> Theme {
    Theme::Dark
}

fn view(state: &State) -> Element<Message> {
    container("I am 300px tall!").height(300).into()
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::FetchSymbols => {
            println!("Fetching symbols");
            Task::perform(
                get_symbols(),
                Message::SymbolsFetched,
            )
        },
        Message::SymbolsFetched(Ok(instruments)) => {
            state.instruments = instruments;
            Task::none()
        }
        _ => {
            Task::none()
        }
    }
}

// #[tokio::main]
fn main() -> iced::Result {
    iced::application("Juicy Lemon", update, view)
        .theme(theme)
        .run()
}
