use crate::{Message, State};
use iced::overlay::menu;
use iced::widget::{ComboBox, combo_box, text_input};
use iced::{Border, Color};

pub fn render_select(state: &State) -> ComboBox<String, Message> {
    let border = Border {
        color: Color::from_rgb(31.0, 41.0, 55.0),
        width: 1.0,
        radius: 6.0.into(),
    };

    let combo_box = combo_box(
        &state.symbol_select_state,
        "Select a crypto pair...",
        state.selected_symbol.as_ref(),
        Message::AddSymbol,
    )
    .on_input(Message::FilterInput)
    .input_style(move |_, status| text_input::Style {
        background: match status {
            text_input::Status::Focused => iced::Background::Color([0.012, 0.027, 0.071].into()),
            _ => iced::Background::Color([0.012, 0.027, 0.071].into()),
        },
        border,
        icon: Default::default(),
        placeholder: [0.976, 0.980, 0.984].into(),
        value: [0.976, 0.980, 0.984].into(),
        selection: Default::default(),
    })
    .menu_style(move |_| menu::Style {
        background: iced::Background::Color([0.012, 0.027, 0.071].into()),
        border,
        text_color: [1.0, 1.0, 1.0].into(),
        selected_text_color: [1.0, 1.0, 1.0].into(),
        selected_background: iced::Background::Color([0.196, 0.196, 0.196].into()),
    });

    combo_box
}
