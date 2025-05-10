use crate::ui::instrument_select::render_select;
use crate::{Message, State};
use iced::widget::image::{self, Image};
use iced::widget::{Column, Row, Rule, button, canvas, container, text};
use iced::{Background, Element, Fill, Length, Padding, Theme, widget};

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

        println!("{:?}", state.displayed_symbol);
        println!("{}", item.symbol.clone());

        let background_color = if state.displayed_symbol == Some(item.symbol.clone()) {
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
    let display_symbol = state.displayed_symbol.as_deref().unwrap_or_default();

    let layout = widget::row![
        widget::column![
            widget::row![
                text(display_symbol)
                    .style(|theme: &Theme| text::Style {
                        color: Some(theme.palette().text),
                    })
                    .height(20)
            ].padding(Padding {
                top: 20.0,
                left: 20.0,
                bottom: 0.0,
                right: 20.0,
            }),
            container(canvas).padding(Padding {
                top: 20.0,
                left: 20.0,
                bottom: 20.0,
                right: 20.0,
            })
        ]
        .spacing(10),
        vertical_rule(),
        symbols_column,
    ];

    Column::new().spacing(20).push(layout).into()
}
