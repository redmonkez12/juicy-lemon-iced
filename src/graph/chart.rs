use std::cell::RefCell;
use std::rc::Rc;
use iced::{Color, Element};
use crate::graph::candle::Candle;
use crate::graph::chart_data::ChartData;
use crate::graph::chart_renderer::ChartRenderer;
use crate::graph::info_bar::InfoBar;
use crate::graph::y_axis::YAxis;
use crate::Message;

pub struct Chart {
    pub renderer: ChartRenderer,
    pub y_axis: YAxis,
    pub chart_data: Rc<RefCell<ChartData>>,
    pub info_bar: InfoBar,
}

impl Chart {
    pub fn new(candles: &[Candle]) -> Self {
        let chart_data = Rc::new(RefCell::new(ChartData::new(candles.to_vec())));
        let renderer = ChartRenderer::new(chart_data.clone());
        let y_axis = YAxis::new(chart_data.clone());
        let info_bar = InfoBar::new("BTCUSDT".to_string(), chart_data.clone());

        chart_data.borrow_mut().compute_height();

        Chart {
            renderer,
            y_axis,
            chart_data,
            info_bar,
        }
    }

    pub fn draw(&self) -> Element<Message> {
        self.renderer.render()
    }

    pub fn set_name(&mut self, name: String) {
        self.info_bar.name = name;
    }

    pub fn set_bear_color(&mut self, r: u8, g: u8, b: u8) {
        self.renderer.bearish_color = Color::from_rgb8(r, g, b);
    }

    pub fn set_bull_color(&mut self, r: u8, g: u8, b: u8) {
        self.renderer.bullish_color = Color::from_rgb8(r, g, b);
    }
}