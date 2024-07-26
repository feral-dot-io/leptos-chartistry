use super::UseInner;
use crate::{colours::Colour, debug::DebugRect, state::State};
use leptos::prelude::*;
use std::{rc::Rc, str::FromStr};

/// Default colour for axis markers.
pub const AXIS_MARKER_COLOUR: Colour = Colour::from_rgb(0xD2, 0xD2, 0xD2);

/// Builds an axis marker. This marks a boundary (e.g., zero or the chart edge) around the inner chart area.
#[derive(Clone, Debug, PartialEq)]
pub struct AxisMarker {
    /// Placement of the marker.
    pub placement: RwSignal<AxisPlacement>,
    /// Colour of the marker.
    pub colour: RwSignal<Colour>,
    /// Whether to show a small arrow at the end of the marker pointing outwards from zero.
    pub arrow: RwSignal<bool>,
    /// Width of the marker and arrow line.
    pub width: RwSignal<f64>,
}

/// Placement of an axis marker around the inner chart area.
#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum AxisPlacement {
    /// Top edge of the inner chart area.
    Top,
    /// Right edge of the inner chart area.
    Right,
    /// Bottom edge of the inner chart area.
    Bottom,
    /// Left edge of the inner chart area.
    Left,
    /// Horizontal zero line (if present).
    HorizontalZero,
    /// Vertical zero line (if present).
    VerticalZero,
}

impl AxisMarker {
    fn new(placement: AxisPlacement) -> Self {
        Self {
            placement: create_rw_signal(placement),
            colour: create_rw_signal(AXIS_MARKER_COLOUR),
            arrow: create_rw_signal(true),
            width: create_rw_signal(1.0),
        }
    }

    /// New axis marker on the top edge.
    pub fn top_edge() -> Self {
        Self::new(AxisPlacement::Top)
    }
    /// New axis marker on the right edge.
    pub fn right_edge() -> Self {
        Self::new(AxisPlacement::Right)
    }
    /// New axis marker on the bottom edge.
    pub fn bottom_edge() -> Self {
        Self::new(AxisPlacement::Bottom)
    }
    /// New axis marker on the left edge.
    pub fn left_edge() -> Self {
        Self::new(AxisPlacement::Left)
    }
    /// New axis marker on the horizontal zero line (if present).
    pub fn horizontal_zero() -> Self {
        Self::new(AxisPlacement::HorizontalZero)
    }
    /// New axis marker on the vertical zero line (if present).
    pub fn vertical_zero() -> Self {
        Self::new(AxisPlacement::VerticalZero)
    }

    /// Sets the arrow visibility.
    pub fn with_arrow(self, arrow: impl Into<bool>) -> Self {
        self.arrow.set(arrow.into());
        self
    }

    /// Sets the marker colour.
    pub fn with_colour(self, colour: impl Into<Colour>) -> Self {
        self.colour.set(colour.into());
        self
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
        <g
            class="_chartistry_axis_marker"
            stroke=colour
            stroke-width=marker.width>
            <Show when=move || in_bounds.get() >
                <DebugRect label="axis_marker" debug=debug />
                <line
                    x1=x1
                    y1=y1
                    x2=x2
                    y2=y2
                    marker-end=arrow
                />
            </Show>
        </g>
    }
}
