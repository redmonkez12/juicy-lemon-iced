use crate::{Message, State};
use iced::Element;
use iced::widget::{Column, Row, Scrollable, button, text, text_input};

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
        println!("{}", item.decimals);

        let formatted_price = item
            .price
            .parse::<f64>()
            .map(|p| format!("{:.*}", item.decimals as usize, p))
            .unwrap_or_else(|_| "N/A".to_string());

        let row = Row::new()
            .spacing(10)
            .push(text(format!("{} - ${}", item.symbol, formatted_price)).size(20))
            .push(iced::widget::button(text("Remove")).on_press(Message::SymbolRemove(item.symbol.clone())));
        watch_list = watch_list.push(row);
    }

    Column::new()
        .spacing(20)
        .padding(20)
        .push(text(state.error_message.clone()))
        .push(input_row)
        .push(Scrollable::new(watch_list))
        .into()
}
