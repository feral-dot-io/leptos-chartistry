use super::line::UseLine;
use super::use_series::{NextSeries, PrepareSeries, ToUseLine};
use super::GetYValue;
use crate::colours::Colour;
use crate::Line;
use std::ops::Add;
use std::rc::Rc;

#[derive(Clone)]
pub struct Stack<T, Y> {
    lines: Vec<Line<T, Y>>,
}

#[derive(Clone)]
struct StackLine<T, Y> {
    line: Line<T, Y>,
    previous: Option<Rc<dyn GetYValue<T, Y>>>,
}

#[derive(Clone)]
struct UseStackLine<T, Y> {
    current: Rc<dyn GetYValue<T, Y>>,
    previous: Option<Rc<dyn GetYValue<T, Y>>>,
}

impl<T, Y> Stack<T, Y> {
    pub fn new(lines: Vec<Line<T, Y>>) -> Self {
        Self { lines }
    }
}

impl<T: 'static, X, Y: Add<Output = Y> + 'static> PrepareSeries<T, X, Y> for Stack<T, Y> {
    fn prepare(self: Rc<Self>, acc: &mut NextSeries<T, Y>) {
        let mut previous = None;
        for line in self.lines.clone() {
            // Add stacked line to acc
            let line = StackLine {
                line,
                previous: previous.clone(),
            };
            let get_y = acc.add_line(&line);
            // Next line will be summed with this one
            previous = Some(get_y.clone());
        }
    }
}

impl<T: 'static, Y: Add<Output = Y> + 'static> ToUseLine<T, Y> for StackLine<T, Y> {
    fn to_use_line(&self, id: usize, colour: Colour) -> (Rc<dyn GetYValue<T, Y>>, UseLine) {
        let (get_y, line) = self.line.to_use_line(id, colour);
        let get_y = Rc::new(UseStackLine {
            current: get_y,
            previous: self.previous.clone(),
        });
        (get_y, line)
    }
}

impl<T, Y: Add<Output = Y> + 'static> GetYValue<T, Y> for UseStackLine<T, Y> {
    fn value(&self, t: &T) -> Y {
        self.current.value(t)
    }

    fn position(&self, t: &T) -> Y {
        self.previous.as_ref().map_or_else(
            || self.current.position(t),
            |prev| self.current.position(t) + prev.position(t),
        )
    }
}
