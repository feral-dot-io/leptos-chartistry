use super::{line::UseLine, ApplyUseSeries, GetYValue, IntoUseLine, SeriesAcc};
use crate::{
    colours::{Colour, ColourScheme},
    Line,
};
use leptos::signal_prelude::*;
use std::ops::Add;
use std::rc::Rc;

/// Default colour scheme for stack. Assumes a light background with dark values for high values.
pub const STACK_COLOUR_SCHEME: [Colour; 10] = BATLOW;

const BATLOW: [Colour; 10] = [
    Colour::from_rgb(0x01, 0x19, 0x59),
    Colour::from_rgb(0x10, 0x3F, 0x60),
    Colour::from_rgb(0x1C, 0x5A, 0x62),
    Colour::from_rgb(0x3C, 0x6D, 0x56),
    Colour::from_rgb(0x68, 0x7B, 0x3E),
    Colour::from_rgb(0x9D, 0x89, 0x2B),
    Colour::from_rgb(0xD2, 0x93, 0x43),
    Colour::from_rgb(0xF8, 0xA1, 0x7B),
    Colour::from_rgb(0xFD, 0xB7, 0xBC),
    Colour::from_rgb(0xFA, 0xCC, 0xFA),
];

/// Draws a stack of lines on top of each other.
///
/// # Example
/// ```rust
/// # use leptos_chartistry::*;
/// # struct MyData { x: f64, y1: f64, y2: f64 }
/// let stack = Stack::new()
///     .line(Line::new(|data: &MyData| data.y1).with_name("fairies"))
///     .line(Line::new(|data: &MyData| data.y2).with_name("pixies"));
/// ```
/// See this in action with the [stacked line chart example](https://feral-dot-io.github.io/leptos-chartistry/examples.html#stacked-line-chart).
#[derive(Clone)]
pub struct Stack<T, Y> {
    lines: Vec<Line<T, Y>>,
    /// Colour scheme for the stack. Interpolates colours across the whole scheme.
    pub colours: RwSignal<ColourScheme>,
}

impl<T, Y> Stack<T, Y> {
    /// Create a new empty stack.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a line to the stack.
    pub fn line(mut self, line: impl Into<Line<T, Y>>) -> Self {
        self.lines.push(line.into());
        self
    }

    /// Gets the current number of lines in the stack.
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    /// Returns true if there are no lines in the stack.
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Sets the colour scheme for the stack.
    pub fn with_colours<Opt>(self, colours: impl Into<ColourScheme>) -> Self {
        self.colours.set(colours.into());
        self
    }
}

impl<T, Y> Default for Stack<T, Y> {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            colours: ColourScheme::from(STACK_COLOUR_SCHEME).invert().into(),
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
        let total_lines = self.lines.len();
        for (id, line) in self.lines.clone().into_iter().enumerate() {
            let colour = create_memo(move |_| colours.get().interpolate(id, total_lines));
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
