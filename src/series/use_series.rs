use super::line::UseLine;
use crate::{
    colours::{Colour, ColourScheme},
    debug::DebugRect,
    state::State,
};
use leptos::*;
use std::rc::Rc;

pub type GetY<T, Y> = Rc<dyn Fn(&T) -> Y>;

/// Accumulator that prepares the next series. i.e., holds lines in a legend.
#[derive(Clone)]
pub struct NextSeries<T, Y> {
    next_id: usize,
    colours: ColourScheme,
    get_ys: Vec<GetY<T, Y>>,
    get_positions: Vec<GetY<T, Y>>,
    lines: Vec<UseLine>,
}

pub trait PrepareSeries<T, X, Y> {
    fn prepare(self: Rc<Self>, acc: &mut NextSeries<T, Y>);
}

pub trait ToUseLine<T, Y> {
    fn to_use_line(&self, id: usize, colour: Colour) -> (GetY<T, Y>, GetY<T, Y>, UseLine);
}

pub(super) fn prepare<T, X, Y>(
    series: Vec<Rc<dyn PrepareSeries<T, X, Y>>>,
    colours: ColourScheme,
) -> (Vec<GetY<T, Y>>, Vec<GetY<T, Y>>, Vec<UseLine>) {
    let mut acc = NextSeries::new(colours);
    for series in series {
        series.prepare(&mut acc);
    }
    (acc.get_ys, acc.get_positions, acc.lines)
}

impl<T, Y> NextSeries<T, Y> {
    fn new(colours: ColourScheme) -> Self {
        Self {
            next_id: 0,
            colours,
            get_ys: Vec::new(),
            get_positions: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn add_line(&mut self, line: &dyn ToUseLine<T, Y>) -> (GetY<T, Y>, UseLine) {
        let id = self.next_id;
        let (get_y, get_pos, line) = line.to_use_line(id, self.colours.by_index(id));
        self.get_ys.push(get_y.clone());
        self.get_positions.push(get_pos);
        self.lines.push(line.clone());
        self.next_id += 1;
        (get_y, line)
    }
}

#[component]
pub fn Snippet<'a, X: 'static, Y: 'static>(
    series: UseLine,
    state: &'a State<X, Y>,
) -> impl IntoView {
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
pub fn Taster<'a, X: 'static, Y: 'static>(
    series: UseLine,
    state: &'a State<X, Y>,
) -> impl IntoView {
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
            {series.taster(bounds, state)}
        </svg>
    }
}
