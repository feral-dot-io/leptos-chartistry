use super::{line::UseLine, GetYValue, ToUseLine};
use crate::{colours::Colour, Line};
use leptos::signal_prelude::*;
use std::ops::Add;
use std::rc::Rc;

#[derive(Clone)]
pub(super) struct StackedLine<T, Y> {
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

impl<T: 'static, Y: Add<Output = Y> + 'static> ToUseLine<T, Y> for StackedLine<T, Y> {
    fn to_use_line(&self, id: usize, colour: Signal<Colour>) -> (UseLine, Rc<dyn GetYValue<T, Y>>) {
        let (line, get_y) = self.line.to_use_line(id, colour);
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

    fn position(&self, t: &T) -> Y {
        self.previous.as_ref().map_or_else(
            || self.current.position(t),
            |prev| self.current.position(t) + prev.position(t),
        )
    }
}
