use super::line::UseLine;
use crate::{
    colours::{Colour, ColourScheme},
    debug::DebugRect,
    state::State,
};
use leptos::*;
use std::{collections::HashMap, rc::Rc};

pub type GetY<T, Y> = Rc<dyn GetYValue<T, Y>>;
pub trait GetYValue<T, Y> {
    fn value(&self, t: &T) -> Y;
    fn position(&self, t: &T) -> Y;
}

/// Accumulator that prepares the next series. i.e., holds lines in a legend.
#[derive(Clone)]
pub struct NextSeries<T, Y> {
    next_id: usize,
    colours: ColourScheme,
    lines: HashMap<usize, UseLine>,
    get_ys: HashMap<usize, GetY<T, Y>>,
}

pub trait PrepareSeries<T, X, Y> {
    fn prepare(self: Rc<Self>, acc: &mut NextSeries<T, Y>);
}

pub trait ToUseLine<T, Y> {
    fn to_use_line(&self, id: usize, colour: Colour) -> (GetY<T, Y>, UseLine);
}

pub(super) fn prepare<T, X, Y>(
    series: Vec<Rc<dyn PrepareSeries<T, X, Y>>>,
    colours: ColourScheme,
) -> (HashMap<usize, UseLine>, HashMap<usize, GetY<T, Y>>) {
    let mut acc = NextSeries::new(colours);
    for series in series {
        series.prepare(&mut acc);
    }
    (acc.lines, acc.get_ys)
}

impl<T, Y> NextSeries<T, Y> {
    fn new(colours: ColourScheme) -> Self {
        Self {
            next_id: 0,
            colours,
            lines: HashMap::new(),
            get_ys: HashMap::new(),
        }
    }

    pub fn add_line(&mut self, line: &dyn ToUseLine<T, Y>) -> GetY<T, Y> {
        let id = self.next_id;
        let (get_y, line) = line.to_use_line(id, self.colours.by_index(id));
        self.next_id += 1;
        self.lines.insert(id, line);
        self.get_ys.insert(id, get_y.clone());
        get_y
    }
}

#[component]
pub fn Snippet<X: 'static, Y: 'static>(series: UseLine, state: State<X, Y>) -> impl IntoView {
    let debug = state.pre.debug;
    let name = series.name.clone();
    view! {
        <div class="_chartistry_snippet" style="white-space: nowrap;">
            <DebugRect label="snippet" debug=debug />
            <Taster series=series state=state />
            {name}
        </div>
    }
}

#[component]
pub fn Taster<X: 'static, Y: 'static>(series: UseLine, state: State<X, Y>) -> impl IntoView {
    let debug = state.pre.debug;
    let font = state.pre.font;
    let bounds = UseLine::taster_bounds(font);
    view! {
        <svg
            class="_chartistry_taster"
            width=move || bounds.get().width()
            height=move || bounds.get().height()
            viewBox=move || format!("0 0 {} {}", bounds.get().width(), bounds.get().height())
            style:padding-right=move || format!("{}px", font.get().width())>
            <DebugRect label="taster" debug=debug bounds=vec![bounds.into()] />
            {series.taster(bounds)}
        </svg>
    }
}
