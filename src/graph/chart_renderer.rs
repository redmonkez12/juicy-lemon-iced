use std::cell::RefCell;
use std::rc::Rc;
use iced::{mouse, Color, Element, Length, Point, Rectangle, Renderer, Size};
use iced::widget::{canvas, Canvas};
use iced::widget::canvas::{Cache, Frame, Geometry, LineCap, Path, Stroke, Style};
use crate::graph::candle::{Candle, CandleType};
use crate::graph::chart_data::ChartData;
use crate::graph::y_axis::YAxis;
use crate::Message;

pub struct ChartRenderer {
    pub bearish_color: Color,
    pub bullish_color: Color,
    pub chart_data: Rc<RefCell<ChartData>>,
    pub sidebar_width: f32,
    pub padding_left: f32,
    pub padding_right: f32,
    cache: Cache,
}

impl ChartRenderer {
    pub const MARGIN_TOP: i64 = 0;
    pub const Y_AXIS_WIDTH: f32 = 2.0;

    pub fn new(chart_data: Rc<RefCell<ChartData>>) -> Self {
        Self {
            bullish_color: Color::from_rgb(69.0 / 255.0, 178.0 / 255.0, 123.0 / 255.0),
            bearish_color: Color::from_rgb(237.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0),
            chart_data,
            sidebar_width: 0.0,
            padding_left: 0.0,
            padding_right: 0.0,
            cache: Cache::default(),
        }
    }
}

impl canvas::Program<Message> for ChartRenderer {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let chart_data = self.chart_data.borrow();
        let candles = &chart_data.visible_candle_set.candles;

        if candles.is_empty() {
            return vec![self.cache.draw(renderer, bounds.size(), |_| {})];
        }

        let chart_width = bounds.width - Self::Y_AXIS_WIDTH - self.sidebar_width
            - self.padding_left - self.padding_right;

        let chart_width = chart_width.max(10.0);
        let candle_count = candles.len() as f32;

        println!("Available chart width: {}, Candle count: {}", chart_width, candle_count);
        println!("Total width: {}, Sidebar: {}, Y-axis: {}, Padding: {} + {}",
                 bounds.width, self.sidebar_width, Self::Y_AXIS_WIDTH,
                 self.padding_left, self.padding_right);

        let ultra_compact = candle_count > 1000.0 && chart_width / candle_count < 1.0;

        let spacing_ratio = if ultra_compact {
            0.0
        } else {
            let density = candle_count / chart_width;
            let base_spacing = 0.2;
            (base_spacing * (0.9_f32.powf(density * 20.0))).max(0.01)
        };

        println!("Using spacing ratio: {}", spacing_ratio);

        let total_spacing_units = (candle_count + 1.0) * spacing_ratio;
        let total_units = candle_count + total_spacing_units;

        let unit_width = chart_width / total_units;
        let candle_width = unit_width;
        let candle_spacing = unit_width * spacing_ratio;

        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            self.draw_chart(frame, bounds, candle_width, candle_spacing);
        });

        vec![geometry]
    }
}

impl ChartRenderer {
    fn draw_chart(&self, frame: &mut Frame, bounds: Rectangle, candle_width: f32, candle_spacing: f32) {
        let chart_area = Rectangle {
            x: bounds.x + Self::Y_AXIS_WIDTH + self.padding_left,
            y: bounds.y + Self::MARGIN_TOP as f32,
            width: bounds.width - Self::Y_AXIS_WIDTH - self.sidebar_width - self.padding_left - self.padding_right,
            height: bounds.height - (Self::MARGIN_TOP as f32),
        };

        let chart_data = self.chart_data.borrow();
        let y_axis = YAxis::new(Rc::clone(&self.chart_data));

        let mut x = chart_area.x + candle_spacing;

        for candle in chart_data.visible_candle_set.candles.iter() {
            self.draw_candle(frame, candle, x, chart_area, &y_axis, candle_width);
            x += candle_width + candle_spacing;
        }
    }

    fn draw_candle(&self, frame: &mut Frame, candle: &Candle, x: f32, chart_area: Rectangle, y_axis: &YAxis, candle_width: f32) {
        let candle_type = candle.get_type();
        let color = match candle_type {
            CandleType::Bearish => self.bearish_color,
            CandleType::Bullish => self.bullish_color,
        };

        let high_y = chart_area.y + chart_area.height - (y_axis.price_to_height(candle.high) as f32);
        let low_y = chart_area.y + chart_area.height - (y_axis.price_to_height(candle.low) as f32);
        let open_y = chart_area.y + chart_area.height - (y_axis.price_to_height(candle.open) as f32);
        let close_y = chart_area.y + chart_area.height - (y_axis.price_to_height(candle.close) as f32);

        if candle_width < 0.5 {
            let line_path = Path::line(
                Point::new(x, high_y),
                Point::new(x, low_y),
            );
            frame.stroke(
                &line_path,
                Stroke {
                    width: 1.0,
                    line_cap: LineCap::Round,
                    style: Style::Solid(color),
                    ..Stroke::default()
                },
            );
            return;
        }

        let wick_path = Path::line(
            Point::new(x + candle_width / 2.0, high_y),
            Point::new(x + candle_width / 2.0, low_y),
        );
        frame.stroke(
            &wick_path,
            Stroke {
                width: 1.0,
                line_cap: LineCap::Round,
                style: Style::Solid(color),
                ..Stroke::default()
            },
        );

        let (body_top, body_bottom) = if candle.open > candle.close {
            (open_y, close_y)
        } else {
            (close_y, open_y)
        };

        let body_height = (body_bottom - body_top).max(1.0);

        let body_rect = Path::rectangle(
            Point::new(x, body_top),
            Size::new(candle_width, body_height),
        );

        match candle_type {
            CandleType::Bullish => {
                frame.fill(&body_rect, color);
            }
            CandleType::Bearish => {
                frame.fill(&body_rect, color);
                if candle_width >= 1.0 {
                    frame.stroke(
                        &body_rect,
                        Stroke {
                            width: 1.0,
                            style: Style::Solid(color),
                            ..Stroke::default()
                        },
                    );
                }
            }
        }
    }

    pub fn render(&self) -> Element<Message> {
        self.cache.clear();

        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill).into()
    }
}