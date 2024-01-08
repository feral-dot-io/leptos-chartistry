mod line;
mod stack;
mod use_data;

pub use line::{Line, Snippet, UseLine};
pub use stack::Stack;
pub use use_data::{Position, RenderData, SeriesVec, UseData};

use crate::colours::{Colour, ColourScheme};
use std::{collections::HashMap, rc::Rc};

type GetY<T, Y> = Rc<dyn GetYValue<T, Y>>;
trait GetYValue<T, Y> {
    fn value(&self, t: &T) -> Y;
    fn position(&self, t: &T) -> Y;
}

/// Accumulator that prepares the next series. i.e., holds lines in a legend.
#[derive(Clone)]
pub struct SeriesAcc<T, Y> {
    next_id: usize,
    colours: ColourScheme,
    lines: HashMap<usize, UseLine>,
    get_ys: HashMap<usize, GetY<T, Y>>,
}

// TODO rename to Series
pub trait Series<T, Y> {
    fn prepare(self: Rc<Self>, acc: &mut SeriesAcc<T, Y>);
}

trait ToUseLine<T, Y> {
    fn to_use_line(&self, id: usize, colour: Colour) -> (GetY<T, Y>, UseLine);
}

fn prepare_series<T, Y>(
    series: Vec<Rc<dyn Series<T, Y>>>,
    colours: ColourScheme,
) -> (HashMap<usize, UseLine>, HashMap<usize, GetY<T, Y>>) {
    let mut acc = SeriesAcc::new(colours);
    for series in series {
        series.prepare(&mut acc);
    }
    (acc.lines, acc.get_ys)
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
        let (get_y, line) = line.to_use_line(id, self.colours.by_index(id));
        self.next_id += 1;
        self.lines.insert(id, line);
        self.get_ys.insert(id, get_y.clone());
        get_y
    }
}
