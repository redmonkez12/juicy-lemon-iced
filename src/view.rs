use crate::ui::instrument_select::render_select;
use crate::{Message, State};
use iced::widget::image::{self, Image};
use iced::widget::{Column, Row, Rule, button, text};
use iced::{Background, Element, Length, Padding, widget};

fn vertical_rule() -> Column<'static, Message> {
    Column::new()
        .height(Length::Fill)
        .width(Length::Shrink)
        .push(Rule::vertical(1))
}

fn horizontal_rule() -> Row<'static, Message> {
    Row::new()
        .height(Length::Shrink)
        .width(Length::Fill)
        .push(Rule::horizontal(1))
}

pub fn view(state: &State) -> Element<Message> {
    if state.loading {
        return text("Loading...").size(20).into();
    }

    if state.instruments.is_empty() {
        return text("Nothing found").size(20).into();
    }

    // let input_row = Row::new()
    //     .spacing(10)
    //     .push(render_select(&state).size(14.0));

    // let mut watch_list = Column::new();

    // if !state.watchlist.is_empty() {
    //     watch_list = watch_list.push(Rule::horizontal(1));
    // }

    let mut symbols_column = widget::column![
        widget::row![render_select(&state).size(14.0)].padding(Padding {
            top: 0.0,
            right: 10.0,
            bottom: 10.0,
            left: 10.0,
        })
    ]
    .width(Length::Fixed(250.0))
    .padding(Padding {
        top: 10.0,
        right: 0.0,
        bottom: 10.0,
        left: 0.0,
    });

    for item in &state.watchlist {
        let formatted_price = if item.price == "-9999" {
            "Loading...".to_string()
        } else {
            item.price
                .parse::<f64>()
                .map(|p| format!("{:.1$}", p, item.decimals))
                .unwrap_or_else(|_| "N/A".to_string())
        };

        let icon_handle = image::Handle::from_path("icons/trash.png");

        symbols_column = symbols_column
            .push(
                widget::row![
                    widget::text(item.symbol.clone()),
                    widget::text(formatted_price).width(Length::Fill),
                    iced::widget::button(Image::new(icon_handle).height(20))
                        .style(|_, _| button::Style {
                            background: Some(Background::Color([0.012, 0.027, 0.071].into())),
                            text_color: [0.976, 0.980, 0.984].into(),
                            border: Default::default(),
                            shadow: Default::default(),
                        })
                        .height(24)
                        .padding(2)
                        .on_press(Message::SymbolRemove(item.symbol.clone())),
                ]
                .spacing(5)
                .padding(Padding {
                    top: 5.0,
                    right: 0.0,
                    bottom: 5.0,
                    left: 5.0,
                }),
            )
            .push(horizontal_rule());

        // let table_row = widget::row![
        //     vertical_rule(),
        //     widget::container(widget::text(item.symbol.clone()))
        //         .width(Length::FillPortion(1))
        //         .padding(14),
        //     vertical_rule(),
        //     widget::container(widget::text(formatted_price))
        //         .width(Length::FillPortion(3))
        //         .padding(14),
        //     vertical_rule(),
        //     widget::container(
        //         render_button("Remove".to_string(), Message::SymbolRemove(item.symbol.clone()))
        //     )
        //     .width(Length::Shrink)
        //     .padding(10),
        //     vertical_rule(),
        // ];

        // watch_list = watch_list.push(table_row).push(Rule::horizontal(1));
    }

    let layout = widget::row![
        widget::container(widget::text("Graph"))
            .width(Length::FillPortion(1))
            .padding(Padding {
                top: 10.0,
                right: 10.0,
                bottom: 10.0,
                left: 10.0,
            }),
        vertical_rule(),
        symbols_column,
    ];

    Column::new()
        .spacing(20)
        // .push(text(state.error_message.clone()))
        .push(layout)
        .into()
}
