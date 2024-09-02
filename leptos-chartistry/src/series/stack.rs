use super::{ApplyUseSeries, GetYValue, IntoUseLine, SeriesAcc, UseY};
use crate::{
    colours::{Colour, ColourScheme, BATLOW},
    Line, Tick,
};
use leptos::prelude::*;
use std::ops::Add;
use std::sync::Arc;

/// Default colour scheme for stack. Assumes a light background with dark values for high values.
pub const STACK_COLOUR_SCHEME: [Colour; 10] = BATLOW;

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
#[non_exhaustive]
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
            colours: RwSignal::new(ColourScheme::from(STACK_COLOUR_SCHEME).invert()),
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

impl<T: 'static, Y: std::ops::Add<Output = Y> + Tick> ApplyUseSeries<T, Y> for Stack<T, Y> {
    fn apply_use_series(self: Arc<Self>, series: &mut SeriesAcc<T, Y>) {
        let colours = self.colours;
        let mut previous = None;
        let total_lines = self.lines.len();
        for (id, line) in self.lines.clone().into_iter().enumerate() {
            let colour = Memo::new(move |_| colours.get().interpolate(id, total_lines));
            let line = StackedLine::new(line, previous.clone());
            let get_y = series.push_line(colour, line);
            // Sum next line with this one
            previous = Some(get_y);
        }
    }
}

#[derive(Clone)]
struct StackedLine<T, Y> {
    line: Line<T, Y>,
    previous: Option<Arc<dyn GetYValue<T, Y>>>,
}

#[derive(Clone)]
struct UseStackLine<T, Y> {
    current: Arc<dyn GetYValue<T, Y>>,
    previous: Option<Arc<dyn GetYValue<T, Y>>>,
}

impl<T, Y> StackedLine<T, Y> {
    pub fn new(line: Line<T, Y>, previous: Option<Arc<dyn GetYValue<T, Y>>>) -> Self {
        Self { line, previous }
    }
}

impl<T: 'static, Y: Add<Output = Y> + Tick> IntoUseLine<T, Y> for StackedLine<T, Y> {
    fn into_use_line(self, id: usize, colour: Memo<Colour>) -> (UseY, Arc<dyn GetYValue<T, Y>>) {
        let (line, get_y) = self.line.into_use_line(id, colour);
        let get_y = Arc::new(UseStackLine {
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
