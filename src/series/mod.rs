mod line;
mod stack;
mod use_data;

pub use line::{Line, Snippet, UseLine};
pub use use_data::{Position, RenderData, UseData};

use self::stack::StackedLine;
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
    colours: Signal<ColourScheme>,
    lines: HashMap<usize, UseLine>,
    get_ys: HashMap<usize, GetY<T, Y>>,
}

trait ToUseLine<T, Y> {
    fn to_use_line(&self, id: usize, colour: Signal<Colour>) -> (UseLine, GetY<T, Y>);
}

impl<T, X, Y> Series<T, X, Y> {
    pub fn new(get_x: impl Fn(&T) -> X + 'static) -> Self {
        Self::new_with_colours(get_x, MaybeSignal::Static(colours::ARBITRARY.into()))
    }

    pub fn new_with_colours(
        get_x: impl Fn(&T) -> X + 'static,
        colours: MaybeSignal<ColourScheme>,
    ) -> Self {
        let colours = Signal::derive(move || colours.get());
        Self {
            get_x: Rc::new(get_x),
            colours,
            lines: HashMap::new(),
            get_ys: HashMap::new(),
        }
    }

    /// Pushes a new line to the series accumulator.
    fn push(&mut self, line: impl ToUseLine<T, Y>) -> GetY<T, Y> {
        // Create line
        let id = self.lines.len();
        let colours = self.colours;
        let colour = create_memo(move |_| colours.get().by_index(id));
        let (line, get_y) = line.to_use_line(id, colour.into());
        // Insert line
        self.lines.insert(id, line);
        self.get_ys.insert(id, get_y.clone());
        get_y
    }

    pub fn line(mut self, line: impl Into<Line<T, Y>>) -> Self {
        _ = self.push(line.into());
        self
    }
}

impl<T, X, Y> Series<T, X, Y> {
    pub fn stack(mut self, lines: Vec<Line<T, Y>>) -> Self
    where
        Y: std::ops::Add<Output = Y>,
    {
        let mut previous = None;
        for line in lines {
            let line = StackedLine::new(line, previous.clone());
            let get_y = self.push(line);
            // Sum next line with this one
            previous = Some(get_y.clone());
        }
        self
    }
}
