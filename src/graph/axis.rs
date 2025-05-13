use crate::price_to_y;
use crate::utils::{
    calculate_tick_count, count_decimal_places, estimate_y_axis_width, truncate_to_decimals,
};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::canvas::{Frame, Path, Stroke, Text};
use iced::{Pixels, Point, Renderer, Theme};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::prelude::*;

pub struct YAxisRenderer<'a> {
    pub screen_width: f32,
    pub screen_height: f32,
    pub display_min: Decimal,
    pub display_max: Decimal,
    pub offset: f32,
    pub decimal_places: u32,
    pub theme: &'a Theme,
}

impl<'a> YAxisRenderer<'a> {
    pub fn render_axis(&self, frame: &mut Frame<Renderer>) -> (Decimal, Decimal, f32) {
        let (mut tick_start, tick_count, tick_interval, axis_y_width) =
            self.render_axis_line(frame);
        tick_start.rescale(self.decimal_places);

        let (display_min, display_max) = self.render_values(
            frame,
            tick_start,
            tick_count,
            tick_interval,
            self.theme,
            axis_y_width,
        );
        (display_min, display_max, axis_y_width)
    }

    fn render_axis_line(&self, frame: &mut Frame<Renderer>) -> (Decimal, usize, Decimal, f32) {
        let (tick_count, tick_interval) = calculate_tick_count(self.display_min, self.display_max);
        let axis_y_width = estimate_y_axis_width(self.display_min, tick_count, tick_interval, 12.0);
        let tick_start = (self.display_min / tick_interval).floor() * tick_interval;

        let y_axis_x = self.screen_width - axis_y_width;
        let y_axis = Path::line(
            Point {
                x: y_axis_x,
                y: 0.0,
            },
            Point {
                x: y_axis_x,
                y: self.screen_height + 100.0,
            },
        );

        frame.stroke(
            &y_axis,
            Stroke::default().with_color([0.976, 0.980, 0.984].into()),
        );

        (tick_start, tick_count, tick_interval, axis_y_width)
    }

    fn render_values(
        &self,
        frame: &mut Frame<Renderer>,
        tick_start: Decimal,
        tick_count: usize,
        tick_interval: Decimal,
        theme: &Theme,
        axis_y_width: f32,
    ) -> (Decimal, Decimal) {
        let axis_x = self.screen_width - axis_y_width;
        let label_x = self.screen_width - axis_y_width + 10.0;
        let display_min = tick_start;
        let display_max = tick_start + Decimal::from_usize(tick_count - 1).unwrap() * tick_interval;

        for i in 0..tick_count {
            let tick_value = tick_start + Decimal::from_usize(i).unwrap() * tick_interval;

            let y_pos = price_to_y(tick_value, display_min, display_max, self.screen_height)
                .to_f32()
                .unwrap();

            let tick_line = Path::line(
                Point {
                    x: 0.0,
                    y: y_pos + self.offset,
                },
                Point {
                    x: axis_x,
                    y: y_pos + self.offset,
                },
            );

            frame.stroke(
                &tick_line,
                Stroke::default().with_color([0.2, 0.2, 0.2].into()),
            );

            frame.fill_text(Text {
                content: tick_value.to_string(),
                position: Point {
                    x: label_x,
                    y: y_pos + self.offset,
                },
                color: theme.palette().text,
                size: Pixels(12.0),
                line_height: Default::default(),
                font: Default::default(),
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Center,
                shaping: Default::default(),
            });
        }

        (display_min, display_max)
    }
}
