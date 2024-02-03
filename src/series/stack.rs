use super::{line::UseLine, ApplyUseSeries, GetYValue, IntoUseLine, SeriesAcc};
use crate::{
    colours::{self, Colour, ColourScheme},
    Line,
};
use leptos::signal_prelude::*;
use std::ops::Add;
use std::rc::Rc;

pub const STACK_COLOUR_SCHEME: [Colour; 10] = colours::BATLOW;

#[derive(Clone)]
pub struct Stack<T, Y> {
    lines: Vec<Line<T, Y>>,
    pub colours: RwSignal<ColourScheme>,
}

impl<T, Y> Stack<T, Y> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn line(mut self, line: impl Into<Line<T, Y>>) -> Self {
        self.lines.push(line.into());
        self
    }

    pub fn with_colours<Opt>(self, colours: impl Into<ColourScheme>) -> Self {
        self.colours.set(colours.into());
        self
    }
}

impl<T, Y> Default for Stack<T, Y> {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            colours: create_rw_signal(STACK_COLOUR_SCHEME.into()),
        }
    }
}

impl<T, Y, I: IntoIterator<Item = Line<T, Y>>> From<I> for Stack<T, Y> {
    fn from(lines: I) -> Self {
        let mut stack = Self::default();
        for line in lines {
            stack = stack.line(line);
        }
        stack
    }
}

impl<T: 'static, Y: std::ops::Add<Output = Y> + 'static> ApplyUseSeries<T, Y> for Stack<T, Y> {
    fn apply_use_series(self: Rc<Self>, series: &mut SeriesAcc<T, Y>) {
        let colours = self.colours;
        let mut previous = None;
        for (id, line) in self.lines.clone().into_iter().enumerate() {
            let colour = create_memo(move |_| colours.get().by_index(id));
            let line = StackedLine::new(line, previous.clone());
            let get_y = series.push(colour, line);
            // Sum next line with this one
            previous = Some(get_y);
        }
    }
}

#[derive(Clone)]
struct StackedLine<T, Y> {
    line: Line<T, Y>,
    previous: Option<Rc<dyn GetYValue<T, Y>>>,
}

#[derive(Clone)]
struct UseStackLine<T, Y> {
    current: Rc<dyn GetYValue<T, Y>>,
    previous: Option<Rc<dyn GetYValue<T, Y>>>,
}

impl<T, Y> StackedLine<T, Y> {
    pub fn new(line: Line<T, Y>, previous: Option<Rc<dyn GetYValue<T, Y>>>) -> Self {
        Self { line, previous }
    }
}

impl<T: 'static, Y: Add<Output = Y> + 'static> IntoUseLine<T, Y> for StackedLine<T, Y> {
    fn into_use_line(self, id: usize, colour: Memo<Colour>) -> (UseLine, Rc<dyn GetYValue<T, Y>>) {
        let (line, get_y) = self.line.into_use_line(id, colour);
        let get_y = Rc::new(UseStackLine {
            current: get_y,
            previous: self.previous.clone(),
        });
        (line, get_y)
    }
}

impl<T, Y: Add<Output = Y>> GetYValue<T, Y> for UseStackLine<T, Y> {
    fn value(&self, t: &T) -> Y {
        self.current.value(t)
    }

    fn cumulative_value(&self, t: &T) -> Y {
        self.previous.as_ref().map_or_else(
            || self.current.cumulative_value(t),
            |prev| self.current.cumulative_value(t) + prev.cumulative_value(t),
        )
    }
}
