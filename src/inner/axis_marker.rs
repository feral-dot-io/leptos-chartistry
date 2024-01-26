use super::{InnerLayout, UseInner};
use crate::{colours::Colour, debug::DebugRect, state::State};
use leptos::*;
use std::{rc::Rc, str::FromStr};

#[derive(Clone, Debug, PartialEq)]
pub struct AxisMarker {
    pub placement: RwSignal<AxisPlacement>,
    pub colour: RwSignal<Option<Colour>>,
    pub arrow: RwSignal<bool>,
    pub width: RwSignal<f64>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AxisPlacement {
    Top,
    Right,
    Bottom,
    Left,
    HorizontalZero,
    VerticalZero,
}

impl AxisMarker {
    fn layout<X: Clone, Y: Clone>(
        placement: impl Into<RwSignal<AxisPlacement>>,
    ) -> InnerLayout<X, Y> {
        InnerLayout::AxisMarker(Self {
            placement: placement.into(),
            colour: RwSignal::default(),
            arrow: true.into(),
            width: 1.0.into(),
        })
    }

    pub fn top_edge<X: Clone, Y: Clone>() -> InnerLayout<X, Y> {
        Self::layout(AxisPlacement::Top)
    }
    pub fn right_edge<X: Clone, Y: Clone>() -> InnerLayout<X, Y> {
        Self::layout(AxisPlacement::Right)
    }
    pub fn bottom_edge<X: Clone, Y: Clone>() -> InnerLayout<X, Y> {
        Self::layout(AxisPlacement::Bottom)
    }
    pub fn left_edge<X: Clone, Y: Clone>() -> InnerLayout<X, Y> {
        Self::layout(AxisPlacement::Left)
    }
    pub fn horizontal_zero<X: Clone, Y: Clone>() -> InnerLayout<X, Y> {
        Self::layout(AxisPlacement::HorizontalZero)
    }
    pub fn vertical_zero<X: Clone, Y: Clone>() -> InnerLayout<X, Y> {
        Self::layout(AxisPlacement::VerticalZero)
    }
}

impl std::fmt::Display for AxisPlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use AxisPlacement::*;
        match self {
            Top => write!(f, "top"),
            Right => write!(f, "right"),
            Bottom => write!(f, "bottom"),
            Left => write!(f, "left"),
            HorizontalZero => write!(f, "horizontal zero"),
            VerticalZero => write!(f, "vertical zero"),
        }
    }
}

impl FromStr for AxisPlacement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AxisPlacement::*;
        match s.to_lowercase().as_str() {
            "top" => Ok(Top),
            "right" => Ok(Right),
            "bottom" => Ok(Bottom),
            "left" => Ok(Left),
            "horizontal zero" => Ok(HorizontalZero),
            "vertical zero" => Ok(VerticalZero),
            _ => Err(format!("unknown axis placement: `{}`", s)),
        }
    }
}

impl<X, Y> UseInner<X, Y> for AxisMarker {
    fn render(self: Rc<Self>, state: State<X, Y>) -> View {
        view!( <AxisMarker marker=(*self).clone() state=state /> )
    }
}

#[component]
pub fn AxisMarker<X: 'static, Y: 'static>(marker: AxisMarker, state: State<X, Y>) -> impl IntoView {
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
        let (zero_x, zero_y) = zero.get();
        let coords @ (x1, y1, x2, y2) = match marker.placement.get() {
            AxisPlacement::Top => (left, top, right, top),
            AxisPlacement::Bottom => (left, bottom, right, bottom),
            AxisPlacement::Left => (left, bottom, left, top),
            AxisPlacement::Right => (right, bottom, right, top),
            AxisPlacement::HorizontalZero => (left, zero_y, right, zero_y),
            AxisPlacement::VerticalZero => (zero_x, bottom, zero_x, top),
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

    let colour = Colour::signal_option(marker.colour, super::DEFAULT_COLOUR_AXIS_MARKER);
    let colour = move || colour.get().to_string();
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
