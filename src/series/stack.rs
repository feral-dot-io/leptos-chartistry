use super::line::UseLine;
use super::use_series::{NextSeries, PrepareSeries, ToUseLine};
use super::GetY;
use crate::colours::Colour;
use crate::Line;
use std::ops::Add;
use std::rc::Rc;

#[derive(Clone)]
pub struct Stack<T, Y> {
    lines: Vec<Line<T, Y>>,
}

#[derive(Clone)]
struct StackedLine<'a, T, Y> {
    line: &'a Line<T, Y>,
    previous: Option<GetY<T, Y>>,
}

impl<T, Y> Stack<T, Y> {
    pub fn new(lines: Vec<Line<T, Y>>) -> Self {
        Self { lines }
    }
}

impl<T: 'static, X, Y: Add<Output = Y> + 'static> PrepareSeries<T, X, Y> for Stack<T, Y> {
    fn prepare(self: Rc<Self>, acc: &mut NextSeries<T, Y>) {
        let mut lines = Vec::new();
        let mut previous: Option<GetY<T, Y>> = None;
        for line in &self.lines {
            // Add stacked line to acc
            let line = StackedLine {
                line,
                previous: previous.clone(),
            };
            let (get_y, line) = acc.add_line(&line);
            // Next line will be summed with this one
            previous = Some(get_y.clone());
            lines.push(line);
        }
    }
}

impl<'a, T: 'static, Y: Add<Output = Y> + 'static> ToUseLine<T, Y> for StackedLine<'a, T, Y> {
    fn to_use_line(&self, id: usize, colour: Colour) -> (GetY<T, Y>, UseLine) {
        let (get_y, line) = self.line.to_use_line(id, colour);
        let previous = self.previous.clone();
        let get_y = move |t: &T| {
            previous
                .as_ref()
                .map_or_else(|| get_y(t), |prev| get_y(t) + prev(t))
        };
        (Rc::new(get_y), line)
    }
}
