mod graph;
mod symbols;
mod ui;
mod update;
mod utils;
mod view;

use iced::{
    Color, Degrees, Font, Point, Radians, Rectangle, Renderer, Settings, Size, Subscription, Task,
    Vector, alignment, mouse, window,
};
use std::time::Duration;

use crate::symbols::{Symbol, SymbolWithPrice};
use crate::update::update;
use crate::view::view;
use iced::Theme;
use iced::time::{self};
use iced::widget::canvas::{Cache, Geometry, Path};
use iced::widget::{canvas, combo_box};

pub const BULL_COLOR: Color = Color::from_rgb(66.0 / 255.0, 149.0 / 255.0, 137.0 / 255.0);
pub const BEAR_COLOR: Color = Color::from_rgb(252.0 / 255.0, 79.0 / 255.0, 111.0 / 255.0);

#[derive(Debug, Clone)]
struct Candle {
    open: f32,
    close: f32,
}

impl Candle {
    fn new(open: f32, close: f32) -> Self {
        Self { open, close }
    }

    fn get_color(&self) -> Color {
        if self.close > self.open {
            BULL_COLOR
        } else {
            BEAR_COLOR
        }
    }
}

fn load_candles() -> Vec<Candle> {
    vec![
        Candle::new(100000.0, 99200.0),
        Candle::new(99200.0, 94600.0),
        Candle::new(94600.0, 97987.0),
        Candle::new(97987.0, 103000.0),
        Candle::new(103000.0, 108500.0),
        Candle::new(108500.0, 110000.0),
        Candle::new(110000.0, 107500.0),
        Candle::new(107500.0, 109500.0),
        Candle::new(109500.0, 112000.0),
        Candle::new(112000.0, 115000.0),
    ]
}

#[derive(Debug, Clone)]
enum Message {
    SymbolsFetched(Vec<Symbol>),
    RefetchPrice,
    AddSymbol(String),
    SymbolRemove(String),
    FetchError(String),
    PricesUpdated(Vec<SymbolWithPrice>),
    CandlesFetched(Vec<Candle>),
    FilterInput(String),
    UpdateSelectOptions,
    InitApp,
    WindowResized(Size),
    Tick(chrono::DateTime<chrono::Local>),
}

#[derive(Default)]
struct WatchListItem {
    price: String,
    symbol: String,
    decimals: usize,
}

impl WatchListItem {
    fn new(symbol: String, price: String, decimals: usize) -> Self {
        Self {
            symbol,
            price,
            decimals,
        }
    }
}

fn price_to_y(price: f32, min_price: f32, max_price: f32, height: f32) -> f32 {
    let normalized = (price - min_price) / (max_price - min_price);
    height - normalized * height
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
        use chrono::Timelike;

        let rectangle = self.clock.draw(renderer, bounds.size(), |frame| {
            let palette = theme.extended_palette();

            // let center = frame.center();

            // let candles = load_candles();
            let candles = &self.candles;
            
            let max_price = candles
                .iter()
                .fold(0.0f32, |acc, c| acc.max(c.open.max(c.close)));
            let min_price = candles
                .iter()
                .fold(f32::MAX, |acc, c| acc.min(c.open.min(c.close)));
            let price_range = max_price - min_price;

            let settings = window::Settings::default();
            let screen_height = settings.size.height - 40.0; // padding
            let screen_width = settings.size.width - 251.0 - 40.0; // sidebar, padding

            // 0
            // 1000

            // 100000
            // 90000

            // 10000
            // height / range - one point

            // mapped_x = a + ((x - min) / (max - min)) * (b - a)
            // let normalized = (x - min) / (max - min);
            // let mapped = a + normalized * (b - a);
            // x = candle.open
            // min = min_price
            // max = max_price
            // a = screen_height
            // b = 0.0
            // let open_y = screen_height + ((candle.open - min_price) / price_range) * (0.0 - screen_height);
            // let open_y = screen_height * (1.0 - (candle.open - min_price) / price_range);
            let unit_width = screen_width / candles.len() as f32;
            let candle_width = unit_width * 0.9;
            let candle_spacing = unit_width * 0.1;

            println!("Price range - {}", price_range);

            for (i, candle) in candles.iter().enumerate() {
                let open_y = price_to_y(candle.open, min_price, max_price, screen_height);
                let close_y = price_to_y(candle.close, min_price, max_price, screen_height);
                let height = (open_y - close_y).abs().max(1.0);

                println!("{}: {} - {}", i, open_y, close_y);

                let x_position = i as f32 * unit_width;

                let rectangle = Path::rectangle(
                    Point {
                        x: x_position + (candle_spacing / 2.0),
                        y: open_y.min(close_y),
                    },
                    Size {
                        width: candle_width,
                        height,
                    },
                );

                frame.fill(&rectangle, candle.get_color());
            }

            // for i in 0..3 {
            //     let rectangle = Path::rectangle(Point {
            //         x: center.x + 40.0 * i as f32,
            //         y: center.y,
            //     }, Size {
            //         width: 30.0,
            //         height: 80.0,
            //     });
            //
            //     frame.fill(&rectangle, palette.secondary.strong.color);
            // }
        });

        vec![rectangle]
    }
}

pub fn hand_rotation(n: u32, total: u32) -> Degrees {
    let turns = n as f32 / total as f32;

    Degrees(360.0 * turns)
}

struct State {
    instruments: Vec<Symbol>,
    watchlist: Vec<WatchListItem>,
    loading: bool,
    input_text: String,
    error_message: String,
    symbol_select_state: combo_box::State<String>,
    selected_symbol: Option<String>,
    candles: Vec<Candle>,
    // chart: Chart,
    width: f32,
    height: f32,
    now: chrono::DateTime<chrono::Local>,
    clock: Cache,
}

impl Default for State {
    fn default() -> Self {
        Self {
            instruments: Vec::new(),
            watchlist: Vec::new(),
            loading: false,
            input_text: String::new(),
            error_message: String::new(),
            symbol_select_state: combo_box::State::default(),
            selected_symbol: None,
            candles: Vec::new(),
            // chart: Chart::new(&Vec::new()),
            width: 0.0,
            height: 0.0,
            now: chrono::offset::Local::now(),
            clock: Cache::new(),
        }
    }
}

fn theme(state: &State) -> Theme {
    // let custom_theme = Arc::new(Custom::new(
    //     "My Dark Theme".into(),
    //     Palette {
    //         background: [0.012, 0.027, 0.071].into(),
    //         text: Color::WHITE,
    //         primary: Color::from_rgb(0.3, 0.6, 0.9),
    //         success: Color::from_rgb(0.2, 0.8, 0.4),
    //         danger: Color::from_rgb(0.9, 0.2, 0.2),
    //     },
    // ));
    //
    // Theme::Custom(custom_theme)

    Theme::CatppuccinFrappe
}

fn init() -> (State, Task<Message>) {
    let state = State {
        instruments: Vec::new(),
        watchlist: Vec::new(),
        error_message: "".to_string(),
        input_text: "".to_string(),
        loading: true,
        selected_symbol: None,
        symbol_select_state: combo_box::State::default(),
        candles: Vec::new(),
        // chart: Chart::new(&Vec::new()),
        width: 0.0,
        height: 0.0,
        now: chrono::offset::Local::now(),
        clock: Cache::new(),
    };
    (state, Task::perform(async {}, |_| Message::InitApp))
}

fn subscription(_: &State) -> Subscription<Message> {
    time::every(Duration::from_millis(500)).map(|_| Message::Tick(chrono::offset::Local::now()))
    // if !state.instruments.is_empty() {
    //     return time::every(Duration::from_secs(2)).map(|_| Message::RefetchPrice);
    // }

    // Subscription::none()
}

fn window_resized_subscription(_: &State) -> Subscription<Message> {
    window::resize_events().map(|(_id, size)| Message::WindowResized(size))
}

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application("Juicy Lemon", update, view)
        .theme(theme)
        .subscription(subscription)
        // .subscription(window_resized_subscription)
        .run_with(init)
}
