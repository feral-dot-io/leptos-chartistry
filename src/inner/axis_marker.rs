use super::UseInner;
use crate::{colours::Colour, debug::DebugRect, state::State};
use leptos::*;
use std::{rc::Rc, str::FromStr};

pub const AXIS_MARKER_COLOUR: Colour = Colour::new(0xD2, 0xD2, 0xD2);

#[derive(Clone, Debug, PartialEq)]
pub struct AxisMarker {
    pub placement: RwSignal<AxisPlacement>,
    pub colour: RwSignal<Colour>,
    pub arrow: RwSignal<bool>,
    pub width: RwSignal<f64>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum AxisPlacement {
    Top,
    Right,
    Bottom,
    Left,
    HorizontalZero,
    VerticalZero,
}

impl AxisMarker {
    pub fn new(placement: impl Into<RwSignal<AxisPlacement>>) -> Self {
        Self {
            placement: placement.into(),
            colour: create_rw_signal(AXIS_MARKER_COLOUR),
            arrow: true.into(),
            width: 1.0.into(),
        }
    }

    pub fn top_edge() -> Self {
        Self::new(AxisPlacement::Top)
    }
    pub fn right_edge() -> Self {
        Self::new(AxisPlacement::Right)
    }
    pub fn bottom_edge() -> Self {
        Self::new(AxisPlacement::Bottom)
    }
    pub fn left_edge() -> Self {
        Self::new(AxisPlacement::Left)
    }
    pub fn horizontal_zero() -> Self {
        Self::new(AxisPlacement::HorizontalZero)
    }
    pub fn vertical_zero() -> Self {
        Self::new(AxisPlacement::VerticalZero)
    }
}

impl std::fmt::Display for AxisPlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use AxisPlacement as P;
        match self {
            P::Top => write!(f, "top"),
            P::Right => write!(f, "right"),
            P::Bottom => write!(f, "bottom"),
            P::Left => write!(f, "left"),
            P::HorizontalZero => write!(f, "horizontal zero"),
            P::VerticalZero => write!(f, "vertical zero"),
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
fn AxisMarker<X: 'static, Y: 'static>(marker: AxisMarker, state: State<X, Y>) -> impl IntoView {
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

    let colour = marker.colour;
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
