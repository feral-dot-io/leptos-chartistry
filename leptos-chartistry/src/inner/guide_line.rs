use super::UseInner;
use crate::{bounds::Bounds, colours::Colour, debug::DebugRect, state::State, Tick};
use leptos::prelude::*;
use std::{rc::Rc, str::FromStr};

/// Default colour for guide lines.
pub const GUIDE_LINE_COLOUR: Colour = Colour::from_rgb(0x9A, 0x9A, 0x9A);

macro_rules! impl_guide_line {
    ($name:ident) => {
        /// Builds a mouse guide line. Aligned over the mouse position or nearest data.
        #[derive(Clone, Debug)]
        pub struct $name {
            /// Alignment of the guide line.
            pub align: RwSignal<AlignOver>,
            /// Width of the guide line.
            pub width: RwSignal<f64>,
            /// Colour of the guide line.
            pub colour: RwSignal<Colour>,
        }

        impl $name {
            fn new(align: AlignOver) -> Self {
                Self {
                    align: create_rw_signal(align.into()),
                    width: create_rw_signal(1.0),
                    colour: create_rw_signal(GUIDE_LINE_COLOUR),
                }
            }

            /// Creates a new guide line aligned over the mouse position.
            pub fn over_mouse() -> Self {
                Self::new(AlignOver::Mouse)
            }

            /// Creates a new guide line aligned over the nearest data.
            pub fn over_data() -> Self {
                Self::new(AlignOver::Data)
            }

            /// Sets the colour of the guide line.
            pub fn with_colour(self, colour: impl Into<Colour>) -> Self {
                self.colour.set(colour.into());
                self
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new(AlignOver::default())
            }
        }
    };
}

impl_guide_line!(XGuideLine);
impl_guide_line!(YGuideLine);

/// Align over mouse or nearest data.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum AlignOver {
    /// Align over the mouse position.
    #[default]
    Mouse,
    /// Align over the nearest data. Creates a "snap to data" effect.
    Data,
}

#[derive(Clone)]
struct UseXGuideLine(XGuideLine);
#[derive(Clone)]
struct UseYGuideLine(YGuideLine);

impl XGuideLine {
    pub(crate) fn use_horizontal<X: Tick, Y: Tick>(self) -> Rc<dyn UseInner<X, Y>> {
        Rc::new(UseXGuideLine(self))
    }
}

impl YGuideLine {
    pub(crate) fn use_vertical<X, Y>(self) -> Rc<dyn UseInner<X, Y>> {
        Rc::new(UseYGuideLine(self))
    }
}

impl std::fmt::Display for AlignOver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlignOver::Mouse => write!(f, "mouse"),
            AlignOver::Data => write!(f, "data"),
        }
    }
}

impl FromStr for AlignOver {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mouse" => Ok(AlignOver::Mouse),
            "data" => Ok(AlignOver::Data),
            _ => Err(format!("invalid align over: `{}`", s)),
        }
    }
}

impl<X: Tick, Y: Tick> UseInner<X, Y> for UseXGuideLine {
    fn render(self: Rc<Self>, state: State<X, Y>) -> View {
        view!( <XGuideLine line=self.0.clone() state=state /> )
    }
}

impl<X, Y> UseInner<X, Y> for UseYGuideLine {
    fn render(self: Rc<Self>, state: State<X, Y>) -> View {
        view!( <YGuideLine line=self.0.clone() state=state /> )
    }
}

#[component]
fn XGuideLine<X: Tick, Y: Tick>(line: XGuideLine, state: State<X, Y>) -> impl IntoView {
    let inner = state.layout.inner;
    let mouse_chart = state.mouse_chart;

    // Data alignment
    let nearest_pos_x = state.pre.data.nearest_position_x(state.hover_position_x);
    let nearest_svg_x = create_memo(move |_| {
        nearest_pos_x
            .get()
            .map(|pos_x| state.projection.get().position_to_svg(pos_x, 0.0).0)
    });

    let pos = Signal::derive(move || {
        let (mouse_x, _) = mouse_chart.get();
        let x = match line.align.get() {
            AlignOver::Data => nearest_svg_x.get().unwrap_or(mouse_x),
            AlignOver::Mouse => mouse_x,
        };
        let inner = inner.get();
        Bounds::from_points(x, inner.top_y(), x, inner.bottom_y())
    });

    view! {
        <GuideLine id="x" width=line.width colour=line.colour state=state pos=pos />
    }
}

#[component]
fn YGuideLine<X: 'static, Y: 'static>(line: YGuideLine, state: State<X, Y>) -> impl IntoView {
    let inner = state.layout.inner;
    let mouse_chart = state.mouse_chart;
    // TODO align over
    let pos = Signal::derive(move || {
        let (_, mouse_y) = mouse_chart.get();
        let inner = inner.get();
        Bounds::from_points(inner.left_x(), mouse_y, inner.right_x(), mouse_y)
    });
    view! {
        <GuideLine id="y" width=line.width colour=line.colour state=state pos=pos />
    }
}

#[component]
fn GuideLine<X: 'static, Y: 'static>(
    id: &'static str,
    width: RwSignal<f64>,
    colour: RwSignal<Colour>,
    state: State<X, Y>,
    pos: Signal<Bounds>,
) -> impl IntoView {
    let debug = state.pre.debug;
    let hover_inner = state.hover_inner;

    let x1 = create_memo(move |_| pos.get().left_x());
    let y1 = create_memo(move |_| pos.get().top_y());
    let x2 = create_memo(move |_| pos.get().right_x());
    let y2 = create_memo(move |_| pos.get().bottom_y());

    // Don't render if any of the coordinates are NaN i.e., no data
    let have_data = Signal::derive(move || {
        !(x1.get().is_nan() || y1.get().is_nan() || x2.get().is_nan() || y2.get().is_nan())
    });

    view! {
        <g
            class=format!("_chartistry_{}_guide_line", id)
            stroke=move || colour.get().to_string()
            stroke-width=width>
            <Show when=move || hover_inner.get() && have_data.get() >
                <DebugRect label=format!("{}_guide_line", id) debug=debug />
                <line
                    x1=x1
                    y1=y1
                    x2=x2
                    y2=y2
                />
            </Show>
        </g>
    }
}
