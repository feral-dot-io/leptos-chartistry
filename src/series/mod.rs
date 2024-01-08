mod line;
mod stack;
mod use_data;

pub use line::{Line, Snippet, UseLine};
pub use stack::Stack;
pub use use_data::{Position, RenderData, UseData};

use crate::colours::{Colour, ColourScheme};
use std::{collections::HashMap, rc::Rc};

type GetX<T, X> = Rc<dyn Fn(&T) -> X>;
type GetY<T, Y> = Rc<dyn GetYValue<T, Y>>;

trait GetYValue<T, Y> {
    fn value(&self, t: &T) -> Y;
    fn position(&self, t: &T) -> Y;
}

#[derive(Clone)]
pub struct SeriesVec<T: 'static, X: 'static, Y: 'static> {
    get_x: GetX<T, X>,
    series: Vec<Rc<dyn Series<T, Y>>>,
}

pub trait Series<T, Y> {
    fn prepare(self: Rc<Self>, acc: &mut SeriesAcc<T, Y>);
}

trait ToUseLine<T, Y> {
    fn to_use_line(&self, id: usize, colour: Colour) -> (GetY<T, Y>, UseLine);
}

/// Accumulator that prepares the next series. i.e., holds lines in a legend.
#[derive(Clone)]
pub struct SeriesAcc<T, Y> {
    next_id: usize,
    colours: ColourScheme,
    lines: HashMap<usize, UseLine>,
    get_ys: HashMap<usize, GetY<T, Y>>,
}

struct PreparedSeries<T, X, Y> {
    get_x: GetX<T, X>,
    lines: HashMap<usize, UseLine>,
    get_ys: HashMap<usize, GetY<T, Y>>,
}

impl<T: 'static, X: Clone + PartialEq + 'static, Y: Clone + PartialEq + 'static>
    SeriesVec<T, X, Y>
{
    pub fn new(get_x: impl Fn(&T) -> X + 'static) -> Self {
        Self {
            get_x: Rc::new(get_x),
            series: Vec::new(),
        }
    }

    pub fn push(mut self, series: impl Series<T, Y> + 'static) -> Self {
        self.series.push(Rc::new(series));
        self
    }

    fn prepare(self, colours: ColourScheme) -> PreparedSeries<T, X, Y> {
        let mut acc = SeriesAcc::new(colours);
        for series in self.series {
            series.prepare(&mut acc);
        }
        PreparedSeries {
            get_x: self.get_x,
            lines: acc.lines,
            get_ys: acc.get_ys,
        }
    }
}

impl<T, Y> SeriesAcc<T, Y> {
    fn new(colours: ColourScheme) -> Self {
        Self {
            next_id: 0,
            colours,
            lines: HashMap::new(),
            get_ys: HashMap::new(),
        }
    }

    fn add_line(&mut self, line: &dyn ToUseLine<T, Y>) -> GetY<T, Y> {
        let id = self.next_id;
        let colour = self.colours.by_index(id);
        let (get_y, line) = line.to_use_line(id, colour);
        self.next_id += 1;
        self.lines.insert(id, line);
        self.get_ys.insert(id, get_y.clone());
        get_y
    }
}
