mod line;
mod stack;
mod use_data;

pub use line::{Line, Snippet, UseLine};
pub use stack::Stack;
pub use use_data::{RenderData, UseData};

use crate::colours::{self, Colour, ColourScheme};
use leptos::signal_prelude::*;
use std::rc::Rc;

const DEFAULT_COLOUR_SCHEME: [Colour; 10] = colours::ARBITRARY;

type GetX<T, X> = Rc<dyn Fn(&T) -> X>;
type GetY<T, Y> = Rc<dyn GetYValue<T, Y>>;

trait GetYValue<T, Y> {
    fn value(&self, t: &T) -> Y;
    fn cumulative_value(&self, t: &T) -> Y;
}

#[derive(Clone)]
pub struct Series<T: 'static, X: 'static, Y: 'static> {
    get_x: GetX<T, X>,
    lines: Vec<Rc<dyn ApplyUseSeries<T, Y>>>,
    pub min_x: RwSignal<Option<X>>,
    pub max_x: RwSignal<Option<X>>,
    pub min_y: RwSignal<Option<Y>>,
    pub max_y: RwSignal<Option<Y>>,
    colours: Signal<Option<ColourScheme>>,
}

trait ApplyUseSeries<T, Y> {
    fn apply_use_series(self: Rc<Self>, _: &mut SeriesAcc<T, Y>);
}

trait IntoUseLine<T, Y> {
    fn into_use_line(self, id: usize, colour: Memo<Colour>) -> (UseLine, GetY<T, Y>);
}

struct SeriesAcc<T, Y> {
    colour_id: usize,
    colours: Memo<ColourScheme>,
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
            colours: Signal::default(),
            lines: Vec::new(),
        }
    }

    pub fn with_colours<Opt>(mut self, colours: impl Into<MaybeSignal<Opt>>) -> Self
    where
        Opt: Clone + Into<Option<ColourScheme>> + 'static,
    {
        let colours = colours.into();
        self.colours = Signal::derive(move || colours.get().into());
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

    fn to_use_lines(&self) -> Vec<(UseLine, GetY<T, Y>)> {
        let colours = ColourScheme::signal_default(self.colours, DEFAULT_COLOUR_SCHEME.into());
        let mut series = SeriesAcc::new(colours);
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
    fn new(colours: Memo<ColourScheme>) -> Self {
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
