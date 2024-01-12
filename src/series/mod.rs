mod line;
mod stack;
mod use_data;

pub use line::{Line, Snippet, UseLine};
pub use stack::Stack;
pub use use_data::{Position, RenderData, UseData};

use crate::colours::{self, Colour, ColourScheme};
use leptos::signal_prelude::*;
use std::{collections::HashMap, rc::Rc};

type GetX<T, X> = Rc<dyn Fn(&T) -> X>;
type GetY<T, Y> = Rc<dyn GetYValue<T, Y>>;

trait GetYValue<T, Y> {
    fn value(&self, t: &T) -> Y;
    fn position(&self, t: &T) -> Y;
}

#[derive(Clone)]
pub struct Series<T: 'static, X: 'static, Y: 'static> {
    get_x: GetX<T, X>,
    colours: MaybeSignal<Option<ColourScheme>>,
    lines: Vec<Rc<dyn ApplyUseSeries<T, X, Y>>>,
}

trait ApplyUseSeries<T, X, Y> {
    fn apply_use_series(self: Rc<Self>, _: &mut UseSeries<T, X, Y>);
}

trait IntoUseLine<T, Y> {
    fn into_use_line(self, id: usize, colour: Signal<Colour>) -> (UseLine, GetY<T, Y>);
}

struct UseSeries<T, X, Y> {
    get_x: GetX<T, X>,
    colours: Memo<ColourScheme>,
    lines: HashMap<usize, UseLine>,
    get_ys: HashMap<usize, GetY<T, Y>>,
}

impl<T, X, Y> Series<T, X, Y> {
    pub fn new(get_x: impl Fn(&T) -> X + 'static) -> Self {
        Self {
            get_x: Rc::new(get_x),
            colours: MaybeSignal::default(),
            lines: Vec::new(),
        }
    }

    pub fn set_colours(mut self, colours: impl Into<MaybeSignal<Option<ColourScheme>>>) -> Self {
        self.colours = colours.into();
        self
    }

    pub fn line(mut self, line: impl Into<Line<T, Y>>) -> Self {
        self.lines.push(Rc::new(line.into()));
        self
    }

    fn into_use(self) -> UseSeries<T, X, Y> {
        let mut series = UseSeries::new(self.get_x, self.colours);
        for line in self.lines {
            line.apply_use_series(&mut series);
        }
        series
    }
}

impl<T, X, Y: std::ops::Add<Output = Y>> Series<T, X, Y> {
    pub fn stack(mut self, stack: impl Into<Stack<T, Y>>) -> Self {
        self.lines.push(Rc::new(stack.into()));
        self
    }
}

impl<T, X, Y> UseSeries<T, X, Y> {
    fn new(get_x: GetX<T, X>, colours: MaybeSignal<Option<ColourScheme>>) -> Self {
        let colours = create_memo(move |_| colours.get().unwrap_or(colours::ARBITRARY.into()));
        Self {
            get_x,
            colours,
            lines: HashMap::new(),
            get_ys: HashMap::new(),
        }
    }

    fn push(&mut self, line: impl IntoUseLine<T, Y>) -> GetY<T, Y> {
        // Create line
        let id = self.lines.len();
        let colours = self.colours;
        let colour = create_memo(move |_| colours.get().by_index(id));
        let (line, get_y) = line.into_use_line(id, colour.into());
        // Insert line
        self.lines.insert(id, line);
        self.get_ys.insert(id, get_y.clone());
        get_y
    }
}
