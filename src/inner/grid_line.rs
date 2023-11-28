use super::{InnerLayout, UseInner};
use crate::{
    colours::{Colour, LIGHTER_GREY},
    debug::DebugRect,
    projection::Projection,
    series::UseSeries,
    state::State,
    ticks::GeneratedTicks,
    TickLabels,
};
use leptos::*;
use std::{borrow::Borrow, rc::Rc};

#[derive(Clone)]
pub struct GridLine<Tick: Clone> {
    width: MaybeSignal<f64>,
    colour: MaybeSignal<Colour>,
    ticks: TickLabels<Tick>,
}

#[derive(Clone)]
pub struct HorizontalGridLine<X: Clone>(GridLine<X>);
#[derive(Clone)]
pub struct VerticalGridLine<Y: Clone>(GridLine<Y>);

#[derive(Clone)]
struct UseGridLine<Tick: 'static> {
    width: MaybeSignal<f64>,
    colour: MaybeSignal<Colour>,
    ticks: Signal<GeneratedTicks<Tick>>,
}

#[derive(Clone)]
struct UseHorizontalGridLine<X: 'static>(UseGridLine<X>);
#[derive(Clone)]
struct UseVerticalGridLine<Y: 'static>(UseGridLine<Y>);

impl<Tick: Clone> GridLine<Tick> {
    fn new(ticks: impl Borrow<TickLabels<Tick>>) -> Self {
        Self {
            width: 1.0.into(),
            colour: Into::<Colour>::into(LIGHTER_GREY).into(),
            ticks: ticks.borrow().clone(),
        }
    }

    /// Vertical grid lines running parallel to the y-axis. These run from top to bottom at each tick.
    pub fn vertical(ticks: impl Borrow<TickLabels<Tick>>) -> HorizontalGridLine<Tick> {
        HorizontalGridLine(Self::new(ticks))
    }
    /// Horizontal grid lines running parallel to the x-axis. These run from left to right at each tick.
    pub fn horizontal(ticks: impl Borrow<TickLabels<Tick>>) -> VerticalGridLine<Tick> {
        VerticalGridLine(Self::new(ticks))
    }
}

impl<X: Clone + PartialEq, Y> InnerLayout<X, Y> for HorizontalGridLine<X> {
    fn into_use(self: Rc<Self>, series: &UseSeries<X, Y>, state: &State) -> Box<dyn UseInner> {
        let avail_width = Projection::derive_width(state.projection);
        Box::new(UseHorizontalGridLine(UseGridLine {
            width: self.0.width,
            colour: self.0.colour,
            ticks: self
                .0
                .ticks
                .clone()
                .generate_x(&state.attr, series.data, avail_width),
        }))
    }
}

impl<X, Y: Clone + PartialEq> InnerLayout<X, Y> for VerticalGridLine<Y> {
    fn into_use(self: Rc<Self>, series: &UseSeries<X, Y>, state: &State) -> Box<dyn UseInner> {
        let avail_height = Projection::derive_height(state.projection);
        Box::new(UseVerticalGridLine(UseGridLine {
            width: self.0.width,
            colour: self.0.colour,
            ticks: self
                .0
                .ticks
                .clone()
                .generate_y(&state.attr, series.data, avail_height),
        }))
    }
}

impl<X> UseInner for UseHorizontalGridLine<X> {
    fn render(self: Box<Self>, state: &State) -> View {
        view! {
            <ViewHorizontalGridLine line=self.0 state=state />
        }
    }
}

impl<X> UseInner for UseVerticalGridLine<X> {
    fn render(self: Box<Self>, state: &State) -> View {
        view! {
            <ViewVerticalGridLine line=self.0 state=state />
        }
    }
}

#[component]
fn ViewHorizontalGridLine<'a, X: 'static>(line: UseGridLine<X>, state: &'a State) -> impl IntoView {
    let debug = state.attr.debug;
    let proj = state.projection;

    let ticks = Signal::derive(move || {
        let ticks = line.ticks; // Ticky ticky tick tick
        with!(|ticks, proj| {
            (ticks.ticks.iter())
                .map(|tick| {
                    let x = ticks.state.position(tick);
                    let x = proj.data_to_svg(x, 0.0).0;
                    let bounds = proj.bounds();
                    view! {
                        <line
                            x1=x
                            y1=move || bounds.top_y()
                            x2=x
                            y2=move || bounds.bottom_y()
                            stroke=move || line.colour.get().to_string()
                            stroke-width=line.width />
                    }
                })
                .collect_view()
        })
    });
    view! {
        <g class="_chartistry_grid_line_x">
            <DebugRect label="XGridLine" debug=debug />
            {ticks}
        </g>
    }
}

#[component]
fn ViewVerticalGridLine<'a, X: 'static>(line: UseGridLine<X>, state: &'a State) -> impl IntoView {
    let debug = state.attr.debug;
    let proj = state.projection;

    let ticks = Signal::derive(move || {
        let ticks = line.ticks;
        with!(|ticks, proj| {
            (ticks.ticks.iter())
                .map(|tick| {
                    let y = ticks.state.position(tick);
                    let y = proj.data_to_svg(0.0, y).1;
                    let bounds = proj.bounds();
                    view! {
                        <line
                            x1=move || bounds.left_x()
                            y1=y
                            x2=move || bounds.right_x()
                            y2=y
                            stroke=move || line.colour.get().to_string()
                            stroke-width=line.width />
                    }
                })
                .collect_view()
        })
    });
    view! {
        <g class="_chartistry_grid_line_y">
            <DebugRect label="YGridLine" debug=debug />
            {ticks}
        </g>
    }
}
