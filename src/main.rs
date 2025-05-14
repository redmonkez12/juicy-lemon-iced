mod graph;
mod symbols;
mod ui;
mod update;
mod utils;
mod view;
mod colors;

use crate::graph::axis::{XAxisRenderer, YAxisRenderer};
use crate::symbols::{Symbol, SymbolWithPrice};
use crate::update::update;
use crate::view::view;
use graph::candle::Candle;
use iced::Theme;
use iced::theme::{Custom, Palette};
use iced::time::{self};
use iced::widget::canvas::{Cache, Geometry, Path, Stroke};
use iced::widget::{canvas, combo_box};
use iced::{Color, Point, Rectangle, Renderer, Size, Subscription, Task, mouse};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::Duration;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;

#[derive(Debug, Clone)]
enum Message {
    SymbolsFetched(Vec<Symbol>),
    RefetchData,
    AddSymbol(String),
    ChangeTimeframe(String),
    SymbolRemove(String),
    SelectSymbol(String),
    FetchError(String),
    PricesUpdated(Vec<SymbolWithPrice>),
    CandlesFetched(Vec<Candle>, String),
    FilterInput(String),
    UpdateSelectOptions,
    InitApp,
}

fn price_to_y(price: Decimal, min_price: Decimal, max_price: Decimal, height: f32) -> Decimal {
    let normalized = (price - min_price) / (max_price - min_price);
    let height_decimal = Decimal::from_f32(height).unwrap();
    height_decimal - normalized * height_decimal
}

impl<Message> canvas::Program<Message> for State {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let rectangle = self.graph.draw(renderer, bounds.size(), |frame| {
            let mut current_candles = VecDeque::new();
            if let (Some(symbol), Some(timeframe)) =
                (&self.displayed_symbol, &self.selected_timeframe)
            {
                if let Some(symbol_map) = self.candles.get(symbol.symbol.as_str()) {
                    if let Some(candles) = symbol_map.get(timeframe) {
                        current_candles = candles.clone();
                    }
                }
            }

            if current_candles.is_empty() {
                return;
            }

            let display_max = current_candles
                .iter()
                .fold(Decimal::MIN, |acc, c| acc.max(c.high.max(c.low)));
            let display_min = current_candles
                .iter()
                .fold(Decimal::MAX, |acc, c| acc.min(c.high.min(c.low)));

            let offset = 30.0;

            let screen_height = bounds.height - 140.0;
            let screen_width = bounds.width;
            
            let y_axis = YAxisRenderer {
                screen_width,
                screen_height,
                display_min,
                display_max,
                offset,
                decimal_places: self.displayed_symbol.as_ref().unwrap().decimals,
                text_color: theme.palette().text,
            };

            let (display_min, display_max, axis_y_width) = y_axis.render_axis(frame);
            let timeframe = self.selected_timeframe.as_ref().unwrap();
            
            let x_axis = XAxisRenderer {
                screen_width: screen_width - axis_y_width,
                screen_height,
                start_time: current_candles.get(0).unwrap().open_time,
                end_time: current_candles.get(current_candles.len() - 1).unwrap().close_time,
                timeframe: timeframe.to_string(),
            };

            x_axis.render_axis(frame);

            let unit_width = (screen_width - axis_y_width - 10.0) / current_candles.len().max(1) as f32;
            let candle_width = unit_width * 0.9;
            let candle_spacing = unit_width * 0.1;

            for (i, candle) in current_candles.iter().enumerate() {
                let open_y = price_to_y(candle.open, display_min, display_max, screen_height).to_f32().unwrap();
                let close_y = price_to_y(candle.close, display_min, display_max, screen_height).to_f32().unwrap();
                let low_y = price_to_y(candle.low, display_min, display_max, screen_height).to_f32().unwrap();
                let high_y = price_to_y(candle.high, display_min, display_max, screen_height).to_f32().unwrap();
                let height = (open_y - close_y).abs().max(1.0);

                let x_position = i as f32 * unit_width;
                let candle_center_x = x_position + (candle_spacing / 2.0) + (candle_width / 2.0);

                let wick = Path::line(
                    Point {
                        x: candle_center_x,
                        y: high_y + offset,
                    },
                    Point {
                        x: candle_center_x,
                        y: low_y + offset,
                    },
                );

                let rectangle = Path::rectangle(
                    Point {
                        x: x_position + (candle_spacing / 2.0),
                        y: open_y.min(close_y) + offset,
                    },
                    Size {
                        width: candle_width,
                        height,
                    },
                );

                frame.fill(&rectangle, candle.get_color());
                frame.stroke(&wick, Stroke::default().with_color(candle.get_color()));
            }
        });

        vec![rectangle]
    }
}

type Timeframe = String;
type CandleCache = HashMap<String, HashMap<Timeframe, VecDeque<Candle>>>;

#[derive(Debug, Clone)]
struct DisplayedSymbol {
    symbol: String,
    timeframe: String,
    decimals: u32,
}

struct State {
    instruments: Vec<Symbol>,
    watchlist: Vec<Symbol>,
    loading: bool,
    input_text: String,
    error_message: String,
    symbol_select_state: combo_box::State<String>,
    timeframe_select_state: combo_box::State<String>,
    selected_timeframe: Option<String>,
    selected_symbol: Option<String>,
    displayed_symbol: Option<DisplayedSymbol>,
    candles: CandleCache,
    graph: Cache,
}

fn theme(_: &State) -> Theme {
    let custom_theme = Arc::new(Custom::new(
        "My Dark Theme".into(),
        Palette {
            background: [0.012, 0.027, 0.071].into(),
            text: [0.976, 0.980, 0.984].into(),
            primary: Color::from_rgb(0.3, 0.6, 0.9),
            success: Color::from_rgb(0.2, 0.8, 0.4),
            danger: Color::from_rgb(0.9, 0.2, 0.2),
        },
    ));

    Theme::Custom(custom_theme)
}

fn init() -> (State, Task<Message>) {
    let state = State {
        instruments: Vec::new(),
        watchlist: Vec::new(),
        error_message: "".to_string(),
        input_text: "".to_string(),
        loading: true,
        selected_timeframe: Some("1m".to_string()),
        timeframe_select_state: combo_box::State::new(vec![
            "1m".to_string(),
            "5m".to_string(),
            "1h".to_string(),
            "4h".to_string(),
            "1d".to_string(),
        ]),
        selected_symbol: None,
        displayed_symbol: None,
        symbol_select_state: combo_box::State::default(),
        candles: HashMap::new(),
        graph: Cache::new(),
    };
    (state, Task::perform(async {}, |_| Message::InitApp))
}

fn subscription(state: &State) -> Subscription<Message> {
    if !state.instruments.is_empty() {
        return time::every(Duration::from_secs(1)).map(|_| Message::RefetchData);
    }

    Subscription::none()
}

fn main() -> iced::Result {
    iced::application("Juicy Lemon", update, view)
        .theme(theme)
        .subscription(subscription)
        .run_with(init)
}
