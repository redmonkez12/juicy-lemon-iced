mod graph;
mod symbols;
mod ui;
mod update;
mod utils;
mod view;

use iced::{Size, Subscription, Task, window, Degrees, Renderer, Rectangle, mouse, Point, Vector, Radians, Font, alignment};
use std::time::Duration;

use crate::symbols::{Symbol, SymbolWithPrice};
use crate::update::update;
use crate::view::view;
use iced::Theme;
use iced::widget::{canvas, combo_box};
use iced::time::{self};
use iced::widget::canvas::{stroke, Cache, Geometry, LineCap, Path, Stroke};

#[derive(Debug, Clone)]
enum Message {
    SymbolsFetched(Vec<Symbol>),
    RefetchPrice,
    AddSymbol(String),
    SymbolRemove(String),
    FetchError(String),
    PricesUpdated(Vec<SymbolWithPrice>),
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

        let clock = self.clock.draw(renderer, bounds.size(), |frame| {
            let palette = theme.extended_palette();

            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.0;

            let background = Path::circle(center, radius);
            frame.fill(&background, palette.secondary.strong.color);

            let short_hand =
                Path::line(Point::ORIGIN, Point::new(0.0, -0.5 * radius));

            let long_hand =
                Path::line(Point::ORIGIN, Point::new(0.0, -0.8 * radius));

            let width = radius / 100.0;

            let thin_stroke = || -> Stroke {
                Stroke {
                    width,
                    style: stroke::Style::Solid(palette.secondary.strong.text),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };

            let wide_stroke = || -> Stroke {
                Stroke {
                    width: width * 3.0,
                    style: stroke::Style::Solid(palette.secondary.strong.text),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };

            frame.translate(Vector::new(center.x, center.y));
            let minutes_portion =
                Radians::from(hand_rotation(self.now.minute(), 60)) / 12.0;
            let hour_hand_angle =
                Radians::from(hand_rotation(self.now.hour(), 12))
                    + minutes_portion;

            frame.with_save(|frame| {
                frame.rotate(hour_hand_angle);
                frame.stroke(&short_hand, wide_stroke());
            });

            frame.with_save(|frame| {
                frame.rotate(hand_rotation(self.now.minute(), 60));
                frame.stroke(&long_hand, wide_stroke());
            });

            frame.with_save(|frame| {
                let rotation = hand_rotation(self.now.second(), 60);

                frame.rotate(rotation);
                frame.stroke(&long_hand, thin_stroke());

                let rotate_factor = if rotation < 180.0 { 1.0 } else { -1.0 };

                frame.rotate(Degrees(-90.0 * rotate_factor));
                frame.fill_text(canvas::Text {
                    content: theme.to_string(),
                    size: (radius / 15.0).into(),
                    position: Point::new(
                        (0.78 * radius) * rotate_factor,
                        -width * 2.0,
                    ),
                    color: palette.secondary.strong.text,
                    horizontal_alignment: if rotate_factor > 0.0 {
                        alignment::Horizontal::Right
                    } else {
                        alignment::Horizontal::Left
                    },
                    vertical_alignment: alignment::Vertical::Bottom,
                    font: Font::MONOSPACE,
                    ..canvas::Text::default()
                });
            });

            // Draw clock numbers
            for hour in 1..=12 {
                let angle = Radians::from(hand_rotation(hour, 12))
                    - Radians::from(Degrees(90.0));
                let x = radius * angle.0.cos();
                let y = radius * angle.0.sin();

                frame.fill_text(canvas::Text {
                    content: format!("{}", hour),
                    size: (radius / 5.0).into(),
                    position: Point::new(x * 0.82, y * 0.82),
                    color: palette.secondary.strong.text,
                    vertical_alignment: alignment::Vertical::Center,
                    horizontal_alignment: alignment::Horizontal::Center,
                    font: Font::MONOSPACE,
                    ..canvas::Text::default()
                });
            }

            // Draw ticks
            for tick in 0..60 {
                let angle = hand_rotation(tick, 60);
                let width = if tick % 5 == 0 { 3.0 } else { 1.0 };

                frame.with_save(|frame| {
                    frame.rotate(angle);
                    frame.fill(
                        &Path::rectangle(
                            Point::new(0.0, radius - 15.0),
                            Size::new(width, 7.0),
                        ),
                        palette.secondary.strong.text,
                    );
                });
            }
        });

        vec![clock]
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
    // candles: Vec<Candle>,
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
            // candles: Vec::new(),
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

    Theme::ALL[(state.now.timestamp() as usize / 10) % Theme::ALL.len()]
        .clone()
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
        // candles: Vec::new(),
        // chart: Chart::new(&Vec::new()),
        width: 0.0,
        height: 0.0,
        now: chrono::offset::Local::now(),
        clock: Cache::new(),
    };
    (state, Task::perform(async {}, |_| Message::InitApp))
}

fn subscription(_: &State) -> Subscription<Message> {
    time::every(Duration::from_millis(500))
        .map(|_| Message::Tick(chrono::offset::Local::now()))
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
