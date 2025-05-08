use crate::graph::candle::Candle;
use crate::graph::candle_set::CandleSet;
use crate::graph::chart_renderer::ChartRenderer;
use crate::graph::info_bar::InfoBar;
use crate::graph::y_axis::YAxis;

#[derive(Debug, Clone)]
pub struct ChartData {
    pub main_candle_set: CandleSet,
    pub visible_candle_set: CandleSet,
    pub terminal_size: (u16, u16),
    pub height: i64,
}

impl ChartData {
    pub fn new(candles: Vec<Candle>) -> ChartData {
        let (w, h) = (600, 600);

        let mut chart_data = ChartData {
            main_candle_set: CandleSet::new(candles),
            visible_candle_set: CandleSet::new(Vec::new()),
            terminal_size: (w, h),
            height: h as i64,
        };

        chart_data.compute_visible_candles();
        chart_data
    }

    pub fn compute_height(&mut self) {
        self.height = self.terminal_size.1 as i64
            - ChartRenderer::MARGIN_TOP
            - InfoBar::HEIGHT as i64;
    }

    pub fn compute_visible_candles(&mut self) {
        let term_width = self.terminal_size.0 as usize as i64;
        let nb_candles = self.main_candle_set.candles.len();

        let nb_visible_candles = term_width - YAxis::WIDTH;

        self.visible_candle_set.set_candles(
            self.main_candle_set
                .candles
                .iter()
                .skip((nb_candles as i64 - nb_visible_candles as i64).max(0) as usize)
                .cloned()
                .collect::<Vec<Candle>>(),
        );
    }
}