mod symbols;
use iced::Task;

use crate::symbols::get_symbols;
use iced::widget::{Column, text, Row, Scrollable};
use iced::{Element, Theme};
use tokio::runtime::Runtime;

#[derive(Debug, Clone)]
enum Message {
    FetchSymbols,
    SymbolsFetched(Result<Vec<String>, String>),
}

#[derive(Default)]
struct State {
    instruments: Vec<String>,
    loading: bool,
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

    println!("{:?}", &state.instruments);

    let mut instrument_list = Column::new().spacing(10);

    for instrument in &state.instruments {
        let row = Row::new().push(text(instrument).size(20));
        instrument_list = instrument_list.push(row);
    }

    Scrollable::new(instrument_list).into()
}


fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
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