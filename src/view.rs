use iced::Element;
use iced::widget::{button, text, text_input, Column, Row, Scrollable};
use crate::{Message, State};

pub fn view(state: &State) -> Element<Message> {
    if state.loading {
        return text("Loading...").size(20).into();
    }

    if state.instruments.is_empty() {
        return text("Nothing found").size(20).into();
    }

    let mut button = button(text("Add to watchlist"));
    if !state.symbol.is_empty() {
        button = button.on_press(Message::AddSymbol);
    }

    let input_row = Row::new()
        .spacing(10)
        .push(text_input("Add instrument...", &state.symbol).on_input(Message::SymbolChanged))
        .push(button);

    let mut watch_list = Column::new().spacing(10);

    for item in &state.watchlist {
        let formatted_price = item
            .price
            .parse::<f64>()
            .map(|p| format!("{:.4}", p))
            .unwrap_or_else(|_| "N/A".to_string());

        let row = Row::new().push(text(format!("{} - ${}", item.symbol, formatted_price)).size(20));
        watch_list = watch_list.push(row);
    }

    Column::new()
        .spacing(20)
        .padding(20)
        .push(input_row)
        .push(Scrollable::new(watch_list))
        .into()
}
