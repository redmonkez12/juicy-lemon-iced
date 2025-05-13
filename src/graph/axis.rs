use crate::utils::{calculate_tick_count, count_decimal_places, truncate_to_decimals};
use iced::widget::canvas::{Frame, Path, Stroke, Text};
use iced::{Pixels, Point, Renderer, Theme};
use iced::alignment::{Horizontal, Vertical};
use rust_decimal::prelude::ToPrimitive;
use crate::price_to_y;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;

pub struct YAxisRenderer {
    pub screen_width: f32,
    pub screen_height: f32,
    pub display_min: f32,
    pub display_max: f32,
    pub offset: f32,
}

impl YAxisRenderer {
    pub fn render_axis(&self, frame: &mut Frame<Renderer>) -> (f32, usize, f32) {
        let (tick_count, tick_interval) = calculate_tick_count(self.display_min, self.display_max);
        let tick_start = (self.display_min / tick_interval).floor() * tick_interval;

        let y_axis_x = self.screen_width - 75.0;
        let y_axis = Path::line(
            Point { x: y_axis_x, y: 0.0 },
            Point { x: y_axis_x, y: self.screen_height + self.offset },
        );

        frame.stroke(
            &y_axis,
            Stroke::default().with_color([0.976, 0.980, 0.984].into()),
        );

        (tick_start, tick_count, tick_interval)
    }

    pub fn render_values(
        &self,
        frame: &mut Frame<Renderer>,
        tick_start: f32,
        tick_count: usize,
        tick_interval: f32,
        theme: &Theme,
    ) -> (f32, f32) {
        let axis_x = self.screen_width - 75.0;
        let label_x = self.screen_width - 70.0;
        let decimals = count_decimal_places(self.display_min);
        let tick_start_decimal = Decimal::from_f32(tick_start).unwrap();
        let tick_end_decimal = Decimal::from_f32(tick_start + (tick_count - 1) as f32 * tick_interval).unwrap();

        let display_min = truncate_to_decimals(tick_start_decimal, decimals);
        let display_max = truncate_to_decimals(tick_end_decimal, decimals);
        
        for i in 0..tick_count {
            let tick_value = truncate_to_decimals(Decimal::from_f32(tick_start + i as f32 * tick_interval).unwrap(), decimals);
            let y_pos =
                price_to_y(tick_value.to_f32().unwrap(), display_min.to_f32().unwrap(), display_max.to_f32().unwrap(), self.screen_height)
                    - self.offset;

            let tick_line = Path::line(
                Point { x: 0.0, y: y_pos },
                Point { x: axis_x, y: y_pos },
            );

            frame.stroke(
                &tick_line,
                Stroke::default().with_color([0.2, 0.2, 0.2].into()),
            );

            frame.fill_text(Text {
                content: format!("{0:.1$}", tick_value, decimals as usize),
                position: Point { x: label_x, y: y_pos },
                color: theme.palette().text,
                size: Pixels(12.0),
                line_height: Default::default(),
                font: Default::default(),
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Center,
                shaping: Default::default(),
            });
        }

        (display_min.to_f32().unwrap(), display_max.to_f32().unwrap())
    }
}
