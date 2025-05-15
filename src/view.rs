use crate::ui::instrument_select::render_select;
use crate::ui::timeframe_select::render_timeframe_select;
use crate::{Message, State};
use iced::widget::image::{self, Image};
use iced::widget::{Column, Row, Rule, button, canvas, container, text};
use iced::{Background, Element, Fill, Length, Padding, Theme, widget, Shrink};

fn vertical_rule() -> Column<'static, Message> {
    Column::new()
        .height(Fill)
        .width(Shrink)
        .push(Rule::vertical(1))
}

fn horizontal_rule() -> Row<'static, Message> {
    Row::new()
        .height(Shrink)
        .width(Fill)
        .push(Rule::horizontal(1))
}

pub fn view(state: &State) -> Element<Message> {
    if state.loading {
        return text("Loading...").size(20).into();
    }

    if state.instruments.is_empty() {
        return text("Nothing found").size(20).into();
    }

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

    let displayed_symbol = match state.displayed_symbol.as_ref() {
        Some(s) => s.symbol.clone(),
        None => "".to_string(),
    };

    for item in &state.watchlist {
        let formatted_price = if item.price == None {
            "Loading...".to_string()
        } else {
            item.price.as_ref().unwrap().to_string()
        };

        let icon_handle = image::Handle::from_path("icons/trash.png");

        let background_color = if Some(displayed_symbol.clone()) == Some(item.symbol.clone()) {
            Background::Color([0.427, 0.157, 0.851].into())
        } else {
            Background::Color([0.012, 0.027, 0.071].into())
        };

        symbols_column = symbols_column
            .push(
                widget::container(
                    widget::row![
                        iced::widget::button(widget::text(item.symbol.clone()))
                            .style(move |theme: &Theme, _| button::Style {
                                background: background_color.into(),
                                text_color: theme.palette().text,
                                border: Default::default(),
                                shadow: Default::default(),
                            })
                            .height(24)
                            .padding(0)
                            .on_press(Message::SelectSymbol(item.symbol.clone())),
                        widget::text(formatted_price).width(Fill),
                        iced::widget::button(Image::new(icon_handle).height(20))
                            .style(move |theme: &Theme, _| button::Style {
                                background: background_color.into(),
                                text_color: theme.palette().text,
                                border: Default::default(),
                                shadow: Default::default(),
                            })
                            .height(24)
                            .padding(2)
                            .on_press(Message::SymbolRemove(item.symbol.clone())),
                    ]
                    .spacing(5),
                )
                .style(move |_| container::Style {
                    background: background_color.into(),
                    border: Default::default(),
                    text_color: None,
                    shadow: Default::default(),
                })
                .width(Fill)
                .padding(Padding {
                    top: 5.0,
                    right: 0.0,
                    bottom: 5.0,
                    left: 5.0,
                }),
            )
            .push(horizontal_rule());
    }

    let canvas = canvas(state).width(Fill).height(Fill);

    let mut top_row = widget::row![
        text(displayed_symbol.clone())
            .style(|theme: &Theme| text::Style {
                color: Some(theme.palette().text),
            })
            .width(Fill)
            .height(20)
    ];

    if !displayed_symbol.is_empty() {
        top_row = top_row.push(render_timeframe_select(&state).size(14.0).width(150.0));
    }

    let layout = widget::row![
        widget::column![
            top_row.spacing(10).padding(Padding {
                top: 10.0,
                left: 10.0,
                bottom: 0.0,
                right: 10.0,
            }),
            container(canvas).padding(Padding {
                top: 10.0,
                left: 10.0,
                bottom: 10.0,
                right: 10.0,
            })
        ]
        .spacing(10),
        vertical_rule(),
        symbols_column,
    ];

    Column::new().spacing(20).push(layout).into()
}
