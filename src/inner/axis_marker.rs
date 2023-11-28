use super::{InnerLayout, UseInner};
use crate::{
    colours::{Colour, LIGHTISH_GREY},
    edge::Edge,
    projection::Projection,
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
    fn into_use(self: Rc<Self>, _: &UseSeries<X, Y>, _: &State) -> Box<dyn UseInner> {
        Box::new((*self).clone())
    }
}

impl UseInner for AxisMarker {
    fn render(self: Box<Self>, state: &State) -> View {
        view! {
            <AxisMarker marker=*self projection=state.projection />
        }
    }
}

#[component]
pub fn AxisMarker(marker: AxisMarker, projection: Signal<Projection>) -> impl IntoView {
    let pos = Signal::derive(move || {
        let b = projection.get().bounds();
        let (top, right, bottom, left) = (b.top_y(), b.right_x(), b.bottom_y(), b.left_x());
        let coords @ (x1, y1, x2, y2) = match marker.placement.get() {
            Placement::Edge => match marker.edge.get() {
                Edge::Top => (left, top, right, top),
                Edge::Bottom => (left, bottom, right, bottom),
                Edge::Left => (left, bottom, left, top),
                Edge::Right => (right, bottom, right, top),
            },

            Placement::Zero => {
                let (zero_x, zero_y) = projection.with(|proj| proj.data_to_svg(0.0, 0.0));
                match marker.edge.get() {
                    Edge::Top => (left, zero_y, right, zero_y),
                    Edge::Bottom => (left, zero_y, right, zero_y),
                    Edge::Left => (zero_x, bottom, zero_x, top),
                    Edge::Right => (zero_x, bottom, zero_x, top),
                }
            }
        };
        // Check coords are within projection bounds
        if b.contains(x1, y1) && b.contains(x2, y2) {
            Some(coords)
        } else {
            None
        }
    });

    let arrow = move || {
        if marker.arrow.get() {
            "url(#marker_axis_arrow)"
        } else {
            ""
        }
    };

    let colour = move || marker.colour.get().to_string();
    let render = move || {
        if let Some((x1, y1, x2, y2)) = pos.get() {
            view! {
                <line
                    x1=x1
                    y1=y1
                    x2=x2
                    y2=y2
                    stroke=colour
                    stroke-width=marker.width
                    marker-end=arrow
                />
            }
            .into_view()
        } else {
            ().into_view()
        }
    };
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
            {render}
        </g>
    }
}
