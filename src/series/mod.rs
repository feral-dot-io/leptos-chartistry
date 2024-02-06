mod line;
mod stack;
mod use_data;

pub use line::{Line, Snippet, UseLine};
pub use stack::{Stack, STACK_COLOUR_SCHEME};
pub use use_data::{RenderData, UseData};

use crate::colours::{Colour, ColourScheme};
use leptos::signal_prelude::*;
use std::rc::Rc;

/// Arbitrary colours for a brighter palette than BATLOW
const SERIES_COLOUR_SCHEME: [Colour; 10] = [
    Colour::new(0x12, 0xA5, 0xED), // Blue
    Colour::new(0xF5, 0x32, 0x5B), // Red
    Colour::new(0x71, 0xc6, 0x14), // Green
    Colour::new(0xFF, 0x84, 0x00), // Orange
    Colour::new(0x7b, 0x4d, 0xff), // Purple
    Colour::new(0xdb, 0x4c, 0xb2), // Magenta
    Colour::new(0x92, 0xb4, 0x2c), // Darker green
    Colour::new(0xFF, 0xCA, 0x00), // Yellow
    Colour::new(0x22, 0xd2, 0xba), // Turquoise
    Colour::new(0xea, 0x60, 0xdf), // Pink
];

type GetX<T, X> = Rc<dyn Fn(&T) -> X>;
type GetY<T, Y> = Rc<dyn GetYValue<T, Y>>;

trait GetYValue<T, Y> {
    fn value(&self, t: &T) -> Y;
    fn cumulative_value(&self, t: &T) -> Y;
}

/// Describes the lines, bars, etc. that make up a series. Maps `T` (your struct) to `X` and `[Y]`.
///
/// TODO Each `T` will yield an `X` and `[Y]`. Each `Y` represents something like a line or bar.
#[derive(Clone)]
pub struct Series<T: 'static, X: 'static, Y: 'static> {
    get_x: GetX<T, X>,
    lines: Vec<Rc<dyn ApplyUseSeries<T, Y>>>,
    pub min_x: RwSignal<Option<X>>,
    pub max_x: RwSignal<Option<X>>,
    pub min_y: RwSignal<Option<Y>>,
    pub max_y: RwSignal<Option<Y>>,
    pub colours: RwSignal<ColourScheme>,
}

trait ApplyUseSeries<T, Y> {
    fn apply_use_series(self: Rc<Self>, _: &mut SeriesAcc<T, Y>);
}

trait IntoUseLine<T, Y> {
    fn into_use_line(self, id: usize, colour: Memo<Colour>) -> (UseLine, GetY<T, Y>);
}

struct SeriesAcc<T, Y> {
    colour_id: usize,
    colours: RwSignal<ColourScheme>,
    lines: Vec<(UseLine, GetY<T, Y>)>,
}

impl<T, X, Y> Series<T, X, Y> {
    pub fn new(get_x: impl Fn(&T) -> X + 'static) -> Self {
        Self {
            get_x: Rc::new(get_x),
            min_x: RwSignal::default(),
            max_x: RwSignal::default(),
            min_y: RwSignal::default(),
            max_y: RwSignal::default(),
            colours: create_rw_signal(SERIES_COLOUR_SCHEME.into()),
            lines: Vec::new(),
        }
    }

    pub fn with_colours<Opt>(self, colours: impl Into<ColourScheme>) -> Self {
        self.colours.set(colours.into());
        self
    }

    pub fn with_min_x(self, max_x: impl Into<Option<X>>) -> Self {
        self.min_x.set(max_x.into());
        self
    }

    pub fn with_max_x(self, max_x: impl Into<Option<X>>) -> Self {
        self.max_x.set(max_x.into());
        self
    }

    pub fn with_min_y(self, min_y: impl Into<Option<Y>>) -> Self {
        self.min_y.set(min_y.into());
        self
    }

    pub fn with_max_y(self, max_y: impl Into<Option<Y>>) -> Self {
        self.max_y.set(max_y.into());
        self
    }

    pub fn with_x_range(self, min_x: impl Into<Option<X>>, max_x: impl Into<Option<X>>) -> Self {
        self.with_min_x(min_x).with_max_x(max_x)
    }

    pub fn with_y_range(self, min_y: impl Into<Option<Y>>, max_y: impl Into<Option<Y>>) -> Self {
        self.with_min_y(min_y).with_max_y(max_y)
    }

    pub fn line(mut self, line: impl Into<Line<T, Y>>) -> Self {
        self.lines.push(Rc::new(line.into()));
        self
    }

    pub fn lines(mut self, lines: impl IntoIterator<Item = impl Into<Line<T, Y>>>) -> Self {
        for line in lines {
            self = self.line(line.into());
        }
        self
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    fn to_use_lines(&self) -> Vec<(UseLine, GetY<T, Y>)> {
        let mut series = SeriesAcc::new(self.colours);
        for line in self.lines.clone() {
            line.apply_use_series(&mut series);
        }
        series.lines
    }
}

impl<T, X, Y: std::ops::Add<Output = Y>> Series<T, X, Y> {
    pub fn stack(mut self, stack: impl Into<Stack<T, Y>>) -> Self {
        self.lines.push(Rc::new(stack.into()));
        self
    }
}

impl<T, Y> SeriesAcc<T, Y> {
    fn new(colours: RwSignal<ColourScheme>) -> Self {
        Self {
            colour_id: 0,
            colours,
            lines: Vec::new(),
        }
    }

    fn next_colour(&mut self) -> Memo<Colour> {
        let id = self.colour_id;
        self.colour_id += 1;
        let colours = self.colours;
        create_memo(move |_| colours.get().by_index(id))
    }

    fn push(&mut self, colour: Memo<Colour>, line: impl IntoUseLine<T, Y>) -> GetY<T, Y> {
        // Create line
        let id = self.lines.len();
        let (line, get_y) = line.into_use_line(id, colour);
        // Insert line
        self.lines.push((line, get_y.clone()));
        get_y
    }
}
