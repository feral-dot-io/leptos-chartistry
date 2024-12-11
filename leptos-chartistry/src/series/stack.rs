use super::{ApplyUseSeries, GetYValue, IntoUseLine, SeriesAcc, UseY};
use crate::{
    colours::{Colour, ColourScheme, BATLOW},
    Line,
};
use leptos::prelude::*;
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

impl<T: 'static> ApplyUseSeries<T, f64> for Stack<T, f64> {
    fn apply_use_series(self: Arc<Self>, series: &mut SeriesAcc<T, f64>) {
        let colours = self.colours;
        let total_lines = self.lines.len();
        let mut previous = Vec::with_capacity(total_lines);
        for (id, line) in self.lines.clone().into_iter().enumerate() {
            let colour = Memo::new(move |_| colours.get().interpolate(id, total_lines));
            let line = StackedLine {
                line,
                previous: previous.clone(),
            };
            // Add line
            let get_y = series.push_line(colour, line);
            // Sum next line with this one
            previous.push(get_y);
        }
    }
}

#[derive(Clone)]
struct StackedLine<T, Y> {
    line: Line<T, Y>,
    previous: Vec<Arc<dyn GetYValue<T, Y>>>,
}

#[derive(Clone)]
struct UseStackLine<T, Y> {
    line: Arc<dyn GetYValue<T, Y>>,
    previous: Vec<Arc<dyn GetYValue<T, Y>>>,
}

impl<T: 'static> IntoUseLine<T, f64> for StackedLine<T, f64> {
    fn into_use_line(self, id: usize, colour: Memo<Colour>) -> (UseY, Arc<dyn GetYValue<T, f64>>) {
        let (line, get_y) = self.line.into_use_line(id, colour);
        let get_y = Arc::new(UseStackLine {
            line: get_y,
            previous: self.previous.clone(),
        });
        (line, get_y)
    }
}

impl<T> GetYValue<T, f64> for UseStackLine<T, f64> {
    fn value(&self, t: &T) -> f64 {
        self.line.value(t)
    }

    fn stacked_value(&self, t: &T) -> f64 {
        self.previous
            .iter()
            .chain(std::iter::once(&self.line))
            .map(|get_y| get_y.value(t))
            .filter(|v| v.is_normal())
            .sum()
    }
}
