use crate::Message;
use iced::Background;
use iced::widget::{Button, text, button};

pub fn render_button(label: String, message: Message) -> Button<'static, Message> {
    iced::widget::button(text(label).size(14.0))
        .style(|_, _| button::Style {
            background: Some(Background::Color([0.427, 0.157, 0.851].into())),
            text_color: [0.976, 0.980, 0.984].into(),
            border: Default::default(),
            shadow: Default::default(),
        })
        .on_press(message)
}
