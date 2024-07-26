use super::UseInner;
use crate::{
    colours::Colour, debug::DebugRect, projection::Projection, state::State, ticks::GeneratedTicks,
    Tick, TickLabels,
};
use leptos::prelude::*;
use std::rc::Rc;

/// Default colour for grid lines.
pub const GRID_LINE_COLOUR: Colour = Colour::from_rgb(0xEF, 0xF2, 0xFA);

macro_rules! impl_grid_line {
    ($name:ident) => {
        /// Builds a tick-aligned grid line across the inner chart area.
        #[derive(Clone)]
        pub struct $name<Tick: 'static> {
            /// Width of the grid line.
            pub width: RwSignal<f64>,
            /// Colour of the grid line.
            pub colour: RwSignal<Colour>,
            /// Ticks to align the grid line to.
            pub ticks: TickLabels<Tick>,
        }

        impl<Tick: crate::Tick> $name<Tick> {
            /// Creates a new grid line from a set of ticks.
            pub fn from_ticks(ticks: impl Into<TickLabels<Tick>>) -> Self {
                Self {
                    ticks: ticks.into(),
                    ..Default::default()
                }
            }

            /// Sets the colour of the grid line.
            pub fn with_colour(self, colour: impl Into<Colour>) -> Self {
                self.colour.set(colour.into());
                self
            }
        }

        impl<Tick: crate::Tick> Default for $name<Tick> {
            fn default() -> Self {
                Self {
                    width: 1.0.into(),
                    colour: create_rw_signal(GRID_LINE_COLOUR),
                    ticks: TickLabels::default(),
                }
            }
        }
    };
}

impl_grid_line!(XGridLine);
impl_grid_line!(YGridLine);

macro_rules! impl_use_grid_line {
    ($name:ident) => {
        struct $name<Tick: 'static> {
            width: RwSignal<f64>,
            colour: RwSignal<Colour>,
            ticks: Signal<GeneratedTicks<Tick>>,
        }

        impl<Tick> Clone for $name<Tick> {
            fn clone(&self) -> Self {
                Self {
                    width: self.width,
                    colour: self.colour,
                    ticks: self.ticks,
                }
            }
        }
    };
}

impl_use_grid_line!(UseXGridLine);
impl_use_grid_line!(UseYGridLine);

impl<X: Tick> XGridLine<X> {
    pub(crate) fn use_horizontal<Y>(self, state: &State<X, Y>) -> Rc<dyn UseInner<X, Y>> {
        let inner = state.layout.inner;
        let avail_width = Signal::derive(move || with!(|inner| inner.width()));
        Rc::new(UseXGridLine {
            width: self.width,
            colour: self.colour,
            ticks: self.ticks.generate_x(&state.pre, avail_width),
        })
    }
}

impl<Y: Tick> YGridLine<Y> {
    pub(crate) fn use_vertical<X>(self, state: &State<X, Y>) -> Rc<dyn UseInner<X, Y>> {
        let inner = state.layout.inner;
        let avail_height = Signal::derive(move || with!(|inner| inner.height()));
        Rc::new(UseYGridLine {
            width: self.width,
            colour: self.colour,
            ticks: self.ticks.generate_y(&state.pre, avail_height),
        })
    }
}

impl<X: Tick, Y> UseInner<X, Y> for UseXGridLine<X> {
    fn render(self: Rc<Self>, state: State<X, Y>) -> View {
        view!( <ViewXGridLine line=(*self).clone() state=state /> )
    }
}

impl<X, Y: Tick> UseInner<X, Y> for UseYGridLine<Y> {
    fn render(self: Rc<Self>, state: State<X, Y>) -> View {
        view!( <ViewYGridLine line=(*self).clone() state=state /> )
    }
}

#[component]
fn ViewXGridLine<X: Tick, Y: 'static>(line: UseXGridLine<X>, state: State<X, Y>) -> impl IntoView {
    let debug = state.pre.debug;
    let inner = state.layout.inner;
    let proj = state.projection;
    let colour = line.colour;

    let lines = move || {
        for_ticks(line.ticks, proj, true)
            .into_iter()
            .map(|(x, label)| {
                view! {
                    <DebugRect label=format!("grid_line_x/{}", label) debug=debug />
                    <line
                        x1=x
                        y1=move || inner.get().top_y()
                        x2=x
                        y2=move || inner.get().bottom_y() />
                }
            })
            .collect_view()
    };

    view! {
        <g
            class="_chartistry_grid_line_x"
            stroke=move || colour.get().to_string()
            stroke-width=line.width>
            <DebugRect label="grid_line_x" debug=debug />
            {lines}
        </g>
    }
}

#[component]
fn ViewYGridLine<X: 'static, Y: Tick>(line: UseYGridLine<Y>, state: State<X, Y>) -> impl IntoView {
    let debug = state.pre.debug;
    let inner = state.layout.inner;
    let proj = state.projection;
    let colour = line.colour;

    let lines = move || {
        for_ticks(line.ticks, proj, false)
            .into_iter()
            .map(|(y, label)| {
                view! {
                    <DebugRect label=format!("grid_line_y/{}", label) debug=debug />
                    <line
                        x1=move || inner.get().left_x()
                        y1=y
                        x2=move || inner.get().right_x()
                        y2=y />
                }
            })
            .collect_view()
    };

    view! {
        <g
            class="_chartistry_grid_line_y"
            stroke=move || colour.get().to_string()
            stroke-width=line.width>
            <DebugRect label="grid_line_y" debug=debug />
            {lines}
        </g>
    }
}

fn for_ticks<Tick: crate::Tick>(
    ticks: Signal<GeneratedTicks<Tick>>,
    proj: Signal<Projection>,
    is_x: bool,
) -> Vec<(f64, String)> {
    ticks.with(move |ticks| {
        let proj = proj.get();
        ticks
            .ticks
            .iter()
            .map(|tick| {
                let label = ticks.state.format(tick);
                let tick = tick.position();
                let tick = if is_x {
                    proj.position_to_svg(tick, 0.0).0
                } else {
                    proj.position_to_svg(0.0, tick).1
                };
                (tick, label)
            })
            .collect::<Vec<_>>()
    })
}
