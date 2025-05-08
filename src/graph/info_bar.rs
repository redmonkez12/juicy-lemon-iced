use std::{cell::RefCell, rc::Rc};
use iced::{Color, Element, Length, Alignment};
use iced::widget::{Container, Row, Text};
use crate::graph::chart_data::ChartData;
use crate::Message;

pub struct InfoBar {
    pub name: String,
    chart_data: Rc<RefCell<ChartData>>,
}

impl InfoBar {
    pub const HEIGHT: f32 = 40.0; // Height in pixels

    pub fn new(name: String, chart_data: Rc<RefCell<ChartData>>) -> InfoBar {
        InfoBar { name, chart_data }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let chart_data = self.chart_data.borrow();
        let main_set = chart_data.main_candle_set.clone();

        // Define colors
        let green = Color::from_rgb(0.2, 0.8, 0.3);
        let red = Color::from_rgb(0.9, 0.3, 0.3);
        let yellow = Color::from_rgb(0.9, 0.85, 0.1);

        let price_text = Text::new(format!("{:.2}", main_set.last_price))
            .color(green);

        let high_text = Text::new(format!("{:.2}", main_set.max_price))
            .color(green);

        let low_text = Text::new(format!("{:.2}", main_set.min_price))
            .color(red);

        let (var_symbol, var_color) = if main_set.variation > 0.0 {
            ("↖", green)
        } else {
            ("↙", red)
        };

        let variation_text = Text::new(format!("{} {:>+.2}%", var_symbol, main_set.variation))
            .color(var_color);

        let avg_color = match main_set.last_price {
            lp if lp > main_set.average => red,
            lp if lp < main_set.average => green,
            _ => yellow,
        };

        let avg_text = Text::new(format!("{:.2}", main_set.average))
            .color(avg_color);

        let volume_text = Text::new(format!("{:.0}", main_set.cumulative_volume))
            .color(green);

        let info_row = Row::new()
            .spacing(15)
            .align_y(Alignment::Center)
            .push(Text::new(&self.name).width(Length::Fill))
            .push(label_value_pair("Price:", price_text))
            .push(label_value_pair("Highest:", high_text))
            .push(label_value_pair("Lowest:", low_text))
            .push(label_value_pair("Var.:", variation_text))
            .push(label_value_pair("Avg.:", avg_text))
            .push(label_value_pair("Cum. Vol:", volume_text));

        Container::new(info_row)
            .width(Length::Fill)
            .height(Length::Fixed(Self::HEIGHT))
            .into()
    }
}

fn label_value_pair<'a>(label: &'a str, value: Text<'a>) -> Element<'a, Message> {
    Row::new()
        .spacing(5)
        .push(Text::new(label))
        .push(value)
        .into()
}
