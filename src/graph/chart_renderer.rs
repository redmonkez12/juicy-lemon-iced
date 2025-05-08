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
    cache: Cache,
}

impl ChartRenderer {
    pub const MARGIN_TOP: i64 = 30;
    pub const CANDLE_WIDTH: f32 = 8.0;
    pub const CANDLE_SPACING: f32 = 2.0;

    pub fn new(chart_data: Rc<RefCell<ChartData>>) -> Self {
        Self {
            bullish_color: Color::from_rgb(52.0 / 255.0, 208.0 / 255.0, 88.0 / 255.0),
            bearish_color: Color::from_rgb(234.0 / 255.0, 74.0 / 255.0, 90.0 / 255.0),
            chart_data,
            cache: Cache::default(),
        }
    }

    pub fn view<'a>(self) -> Element<'a, Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl canvas::Program<Message> for ChartRenderer {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        // This is where we'll draw our chart
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            self.draw_chart(frame, bounds);
        });

        vec![geometry]
    }
}

impl ChartRenderer {
    fn draw_chart(&self, frame: &mut Frame, bounds: Rectangle) {
        let y_axis_width = 60.0;
        let chart_area = Rectangle {
            x: bounds.x + y_axis_width,
            y: bounds.y + Self::MARGIN_TOP as f32,
            width: bounds.width - y_axis_width,
            height: bounds.height - (Self::MARGIN_TOP as f32) - 40.0,
        };

        let chart_data = self.chart_data.borrow();
        let y_axis = YAxis::new(Rc::clone(&self.chart_data));

        for (i, candle) in chart_data.visible_candle_set.candles.iter().enumerate() {
            let x = chart_area.x + i as f32 * (Self::CANDLE_WIDTH + Self::CANDLE_SPACING);
            self.draw_candle(frame, candle, x, chart_area, &y_axis);
        }
    }

    fn draw_candle(&self, frame: &mut Frame, candle: &Candle, x: f32, chart_area: Rectangle, y_axis: &YAxis) {
        let candle_type = candle.get_type();
        let color = match candle_type {
            CandleType::Bearish => self.bearish_color,
            CandleType::Bullish => self.bullish_color,
        };

        let high_y = chart_area.y + chart_area.height - (y_axis.price_to_height(candle.high) as f32);
        let low_y = chart_area.y + chart_area.height - (y_axis.price_to_height(candle.low) as f32);
        let open_y = chart_area.y + chart_area.height - (y_axis.price_to_height(candle.open) as f32);
        let close_y = chart_area.y + chart_area.height - (y_axis.price_to_height(candle.close) as f32);

        let wick_path = Path::line(
            Point::new(x + Self::CANDLE_WIDTH / 2.0, high_y),
            Point::new(x + Self::CANDLE_WIDTH / 2.0, low_y),
        );
        frame.stroke(
            &wick_path,
            Stroke {
                width: 1.0,
                line_cap: LineCap::Round,
                style: Style::Solid(Color::WHITE),
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
            Size::new(Self::CANDLE_WIDTH, body_height),
        );

        match candle_type {
            CandleType::Bullish => {
                frame.fill(&body_rect, color);
            }
            CandleType::Bearish => {
                frame.fill(&body_rect, color);
                frame.stroke(
                    &body_rect,
                    Stroke {
                        width: 1.0,
                        style: Style::Solid(Color::WHITE),
                        ..Stroke::default()
                    },
                );
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