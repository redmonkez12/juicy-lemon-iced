use crate::ui::instrument_select::render_select;
use crate::{Message, State};
use iced::widget::{Column, Row, Rule, text};
use iced::{Element, Length, widget};

fn vertical_rule() -> Column<'static, Message> {
    Column::new()
        .height(50)
        .width(Length::Shrink)
        .push(Rule::vertical(1))
}

pub fn view(state: &State) -> Element<Message> {
    if state.loading {
        return text("Loading...").size(20).into();
    }

    if state.instruments.is_empty() {
        return text("Nothing found").size(20).into();
    }

    let input_row = Row::new().spacing(10).push(render_select(&state).size(14.0));

    let mut watch_list = Column::new();

    if !state.watchlist.is_empty() {
        watch_list = watch_list.push(Rule::horizontal(1));
    }

    for item in &state.watchlist {
        let formatted_price = if item.price == "-9999" {
            "Loading...".to_string()
        } else {
            item.price
                .parse::<f64>()
                .map(|p| format!("{:.1$}", p, item.decimals))
                .unwrap_or_else(|_| "N/A".to_string())
        };

        let table_row = widget::row![
            vertical_rule(),
            widget::container(widget::text(item.symbol.clone()))
                .width(Length::FillPortion(1))
                .padding(14),
            vertical_rule(),
            widget::container(widget::text(formatted_price))
                .width(Length::FillPortion(3))
                .padding(14),
            vertical_rule(),
            widget::container(
                iced::widget::button(text("Remove"))
                    .on_press(Message::SymbolRemove(item.symbol.clone()))
            )
            .width(Length::Shrink)
            .padding(10),
            vertical_rule(),
        ];

        watch_list = watch_list.push(table_row).push(Rule::horizontal(1));
    }

    Column::new()
        .spacing(20)
        .padding(20)
        .push(text(state.error_message.clone()))
        .push(input_row)
        .push(watch_list)
        .into()
}
