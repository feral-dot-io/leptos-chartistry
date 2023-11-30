use super::{InnerLayout, UseInner};
use crate::{
    colours::{Colour, LIGHTISH_GREY},
    debug::DebugRect,
    edge::Edge,
    state::State,
    UseSeries,
};
use leptos::*;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct AxisMarker {
    edge: MaybeSignal<Edge>,
    placement: MaybeSignal<Placement>,
    colour: MaybeSignal<Colour>,
    arrow: MaybeSignal<bool>,
    width: MaybeSignal<f64>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Placement {
    Edge,
    Zero,
}

impl AxisMarker {
    fn new(
        edge: impl Into<MaybeSignal<Edge>>,
        placement: impl Into<MaybeSignal<Placement>>,
    ) -> Self {
        Self {
            edge: edge.into(),
            placement: placement.into(),
            colour: Into::<Colour>::into(LIGHTISH_GREY).into(),
            arrow: true.into(),
            width: 1.0.into(),
        }
    }

    pub fn top_edge() -> Self {
        Self::new(Edge::Top, Placement::Edge)
    }
    pub fn right_edge() -> Self {
        Self::new(Edge::Right, Placement::Edge)
    }
    pub fn bottom_edge() -> Self {
        Self::new(Edge::Bottom, Placement::Edge)
    }
    pub fn left_edge() -> Self {
        Self::new(Edge::Left, Placement::Edge)
    }
    pub fn horizontal_zero() -> Self {
        Self::new(Edge::Bottom, Placement::Zero)
    }
    pub fn vertical_zero() -> Self {
        Self::new(Edge::Left, Placement::Zero)
    }

    pub fn set_colour(mut self, colour: impl Into<MaybeSignal<Colour>>) -> Self {
        self.colour = colour.into();
        self
    }

    pub fn set_arrow(mut self, arrow: impl Into<MaybeSignal<bool>>) -> Self {
        self.arrow = arrow.into();
        self
    }

    pub fn set_width(mut self, width: impl Into<MaybeSignal<f64>>) -> Self {
        self.width = width.into();
        self
    }
}

impl<X, Y> InnerLayout<X, Y> for AxisMarker {
    fn into_use(self: Rc<Self>, _: &UseSeries<X, Y>, _: &State<X, Y>) -> Box<dyn UseInner<X, Y>> {
        Box::new((*self).clone())
    }
}

impl<X, Y> UseInner<X, Y> for AxisMarker {
    fn render(self: Box<Self>, state: &State<X, Y>) -> View {
        view!( <AxisMarker marker=*self state=state /> )
    }
}

#[component]
pub fn AxisMarker<'a, X: 'static, Y: 'static>(
    marker: AxisMarker,
    state: &'a State<X, Y>,
) -> impl IntoView {
    let debug = state.pre.debug;
    let zero = state.svg_zero;
    let inner = state.layout.inner;

    let pos = create_memo(move |_| {
        let inner = inner.get();
        let (top, right, bottom, left) = (
            inner.top_y(),
            inner.right_x(),
            inner.bottom_y(),
            inner.left_x(),
        );
        let coords @ (x1, y1, x2, y2) = match marker.placement.get() {
            Placement::Edge => match marker.edge.get() {
                Edge::Top => (left, top, right, top),
                Edge::Bottom => (left, bottom, right, bottom),
                Edge::Left => (left, bottom, left, top),
                Edge::Right => (right, bottom, right, top),
            },

            Placement::Zero => {
                let (zero_x, zero_y) = zero.get();
                match marker.edge.get() {
                    Edge::Top => (left, zero_y, right, zero_y),
                    Edge::Bottom => (left, zero_y, right, zero_y),
                    Edge::Left => (zero_x, bottom, zero_x, top),
                    Edge::Right => (zero_x, bottom, zero_x, top),
                }
            }
        };
        let in_bounds = inner.contains(x1, y1) && inner.contains(x2, y2);
        (in_bounds, coords)
    });
    // Check coords are within projection bounds
    let in_bounds = create_memo(move |_| pos.get().0);
    let x1 = create_memo(move |_| pos.get().1 .0);
    let y1 = create_memo(move |_| pos.get().1 .1);
    let x2 = create_memo(move |_| pos.get().1 .2);
    let y2 = create_memo(move |_| pos.get().1 .3);

    let arrow = move || {
        if marker.arrow.get() {
            "url(#marker_axis_arrow)"
        } else {
            ""
        }
    };

    let colour = move || marker.colour.get().to_string();
    view! {
        <g class="_chartistry_axis_marker">
            <defs>
                <marker
                    id="marker_axis_arrow"
                    markerUnits="strokeWidth"
                    markerWidth=7
                    markerHeight=8
                    refX=0
                    refY=4
                    orient="auto">
                    <path d="M0,0 L0,8 L7,4 z" fill=colour />
                </marker>
            </defs>
            <Show when=move || in_bounds.get() >
                <DebugRect label="axis_marker" debug=debug />
                <line
                    x1=x1
                    y1=y1
                    x2=x2
                    y2=y2
                    stroke=colour
                    stroke-width=marker.width
                    marker-end=arrow
                />
            </Show>
        </g>
    }
}
