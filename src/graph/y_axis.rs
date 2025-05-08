use std::{cell::RefCell, rc::Rc};
use crate::graph::chart_data::ChartData;

pub struct YAxis {
    pub chart_data: Rc<RefCell<ChartData>>,
}

impl YAxis {
    pub const CHAR_PRECISION: i64 = 6;
    pub const DEC_PRECISION: i64 = 2;
    pub const MARGIN_RIGHT: i64 = 4;

    pub const WIDTH: i64 = YAxis::CHAR_PRECISION
        + YAxis::MARGIN_RIGHT
        + 1
        + YAxis::DEC_PRECISION
        + YAxis::MARGIN_RIGHT;

    pub fn new(chart_data: Rc<RefCell<ChartData>>) -> YAxis {
        YAxis { chart_data }
    }

    pub fn price_to_height(&self, price: f64) -> f64 {
        let chart_data = self.chart_data.borrow();
        let height = chart_data.height;

        let min_value = chart_data.visible_candle_set.min_price;
        let max_value = chart_data.visible_candle_set.max_price;

        (price - min_value) / (max_value - min_value) * height as f64
    }
}