use std::cell::RefCell;
use std::rc::Rc;
use iced::{Element};
use crate::graph::candle::Candle;
use crate::graph::chart_data::ChartData;
use crate::graph::chart_renderer::ChartRenderer;
use crate::graph::y_axis::YAxis;
use crate::Message;

pub struct Chart {
    pub renderer: ChartRenderer,
    pub y_axis: YAxis,
    pub chart_data: Rc<RefCell<ChartData>>,
}

impl Chart {
    pub fn new(candles: &[Candle]) -> Self {
        let chart_data = Rc::new(RefCell::new(ChartData::new(candles.to_vec())));
        let renderer = ChartRenderer::new(chart_data.clone());
        let y_axis = YAxis::new(chart_data.clone());

        // chart_data.borrow_mut().compute_height();

        Chart {
            renderer,
            y_axis,
            chart_data,
        }
    }

    pub fn draw(&self) -> Element<Message> {
        self.renderer.render()
    }
}