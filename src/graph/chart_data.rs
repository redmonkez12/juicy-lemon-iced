use iced::window;
use crate::graph::candle::Candle;
use crate::graph::candle_set::CandleSet;

#[derive(Debug, Clone)]
pub struct ChartData {
    pub main_candle_set: CandleSet,
    pub visible_candle_set: CandleSet,
    pub chart_size: (f32, f32),
    pub height: i64,
}

impl ChartData {
    pub fn new(candles: Vec<Candle>) -> ChartData {
        let settings = window::Settings::default();
        let (w, h) = (500.0, 658.0);
        
        
        let mut chart_data = ChartData {
            main_candle_set: CandleSet::new(candles),
            visible_candle_set: CandleSet::new(Vec::new()),
            chart_size: (w, h),
            height: h as i64,
        };

        chart_data.compute_visible_candles();
        chart_data
    }

    // pub fn compute_height(&mut self) {
    //     self.height = self.chart_size.1 as i64
    //         - ChartRenderer::MARGIN_TOP
    // }

    pub fn compute_visible_candles(&mut self) {
        // let chart_width = self.chart_size.0 as usize as i64;
        // let nb_candles = self.main_candle_set.candles.len();
        // 
        // let nb_visible_candles = chart_width - YAxis::WIDTH;

        self.visible_candle_set = self.main_candle_set.clone();
        
        // self.visible_candle_set.set_candles(
        //     self.main_candle_set
        //         .candles
        //         .iter()
        //         .skip((nb_candles as i64 - nb_visible_candles).max(0) as usize)
        //         .cloned()
        //         .collect::<Vec<Candle>>(),
        // );
    }
}