use crate::price_to_y;
use crate::utils::{calculate_tick_count, estimate_y_axis_width};
use chrono::Duration;
use chrono::prelude::*;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::canvas::{Frame, Path, Stroke, Text};
use iced::{Color, Pixels, Point, Renderer, Theme};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::prelude::*;
use crate::colors::{GRAY_COLOR, WHITE_COLOR};

pub struct YAxisRenderer {
    pub screen_width: f32,
    pub screen_height: f32,
    pub display_min: Decimal,
    pub display_max: Decimal,
    pub offset: f32,
    pub decimal_places: u32,
    pub text_color: Color,
}

impl YAxisRenderer {
    pub fn render_axis(&self, frame: &mut Frame<Renderer>) -> (Decimal, Decimal, f32) {
        let (mut tick_start, tick_count, tick_interval, axis_y_width) =
            self.render_axis_line(frame);
        tick_start.rescale(self.decimal_places);

        let (display_min, display_max) = self.render_values(
            frame,
            tick_start,
            tick_count,
            tick_interval,
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
                Stroke::default().with_color(GRAY_COLOR.into()),
            );

            frame.fill_text(Text {
                content: tick_value.to_string(),
                position: Point {
                    x: label_x,
                    y: y_pos + self.offset,
                },
                color: self.text_color,
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

pub struct XAxisRenderer {
    pub screen_width: f32,
    pub screen_height: f32,
    pub start_time: u64,
    pub end_time: u64,
    pub timeframe: String,
}

impl XAxisRenderer {
    pub fn render_axis(self, frame: &mut Frame<Renderer>) {
        let x_axis = Path::line(
            Point {
                x: 0.0,
                y: self.screen_height + 100.0,
            },
            Point {
                x: self.screen_width,
                y: self.screen_height + 100.0,
            },
        );

        frame.stroke(
            &x_axis,
            Stroke::default().with_color(WHITE_COLOR.into()),
        );

        let font_size = 12.0;

        if self.timeframe == "1m" {
            let start_ts = self.start_time.to_i64().unwrap();
            let end_ts = self.end_time.to_i64().unwrap();

            let start: DateTime<Utc> = Utc.timestamp_millis_opt(start_ts).unwrap();
            let end: DateTime<Utc> = Utc.timestamp_millis_opt(end_ts).unwrap();

            let start_local: DateTime<Local> = DateTime::from(start);
            let end_local: DateTime<Local> = DateTime::from(end);

            let time_range_minutes = (start - end).num_minutes();
            println!("Time range in minutes: {}", time_range_minutes);

            let interval_minutes = if time_range_minutes > 180 {
                30
            } else {
                15
            };

            let current_minute = start_local.minute() as i64;
            let remainder = current_minute % interval_minutes;
            let first_label_offset = if remainder == 0 {
                0
            } else {
                interval_minutes - remainder
            };
            let mut current_time = start_local + Duration::minutes(first_label_offset);

            let seconds_per_pixel = (end_local - start_local).num_seconds() as f32 / self.screen_width;

            while current_time <= end_local {
                let offset_seconds = (current_time - start_local).num_seconds() as f32;
                let x_pos = offset_seconds / seconds_per_pixel;

                let label_text = current_time.format("%H:%M").to_string();

                if x_pos >= 0.0 && x_pos <= self.screen_width {
                    self.draw_label(frame, &label_text, x_pos, font_size);

                    let tick = Path::line(
                        Point {
                            x: x_pos,
                            y: self.screen_height + 100.0,
                        },
                        Point {
                            x: x_pos,
                            y: self.screen_height + 107.0,
                        },
                    );

                    frame.stroke(&tick, Stroke::default().with_color([0.6, 0.6, 0.6].into()));
                }

                current_time = current_time + Duration::minutes(interval_minutes);
            }
        }
    }

    fn draw_label(&self, frame: &mut Frame<Renderer>, text: &str, x_pos: f32, font_size: f32) {
        let y_pos = self.screen_height + 110.0;
        let text_width = text.len() as f32 * font_size * 0.6;

        let label_x = x_pos - (text_width / 2.0);

        frame.fill_text(Text {
            content: text.to_string(),
            position: Point {
                x: label_x,
                y: y_pos,
            },
            size: Pixels(font_size),
            color: [0.4, 0.4, 0.4].into(),
            ..Text::default()
        });
    }
}
