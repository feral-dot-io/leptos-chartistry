#![warn(missing_docs)]
//! Welcome to Chartistry! This crate provides a flexible way to build charts in [Leptos](https://github.com/leptos-rs/leptos).
//!
//! All charts are built using the [Chart] fn. If you understand this function, you understand this library.
//!
//! ## Examples
//! If you skimmed the prop list you'd get a good idea of how the charts are built up from their individual layout components e.g., top, left, and tooltip. These components are picked from a grab bag of options that you'd probably expect in a chart e.g., a label, legend, and ticks. This also makes it flexible and very easy to pick and choose what you need from existing examples. So make sure to take a look at the [list of examples](https://feral-dot-io.github.io/leptos-chartistry/examples/).
//!
//! ```rust
//! // TODO
//! ```
//!
//!
//!

mod aspect_ratio;
mod bounds;
mod chart;
mod colours;
mod debug;
mod edge;
mod inner;
mod layout;
mod overlay;
mod padding;
mod projection;
mod series;
mod state;
mod ticks;
mod use_watched_node;

pub use aspect_ratio::AspectRatio;
pub use chart::Chart;
pub use colours::{Colour, ColourScheme};
pub use edge::Edge;
pub use inner::{
    axis_marker::{AxisMarker, AxisPlacement, AXIS_MARKER_COLOUR},
    grid_line::{XGridLine, YGridLine, GRID_LINE_COLOUR},
    guide_line::{AlignOver, XGuideLine, YGuideLine, GUIDE_LINE_COLOUR},
    legend::InsetLegend,
    InnerLayout, IntoInner,
};
pub use layout::{
    legend::Legend,
    rotated_label::{Anchor, RotatedLabel},
    tick_labels::TickLabels,
    EdgeLayout, IntoEdge,
};
pub use overlay::tooltip::{Tooltip, TooltipPlacement, TooltipSortBy, TOOLTIP_CURSOR_DISTANCE};
pub use padding::Padding;
pub use series::{Line, Series, Stack, STACK_COLOUR_SCHEME};
pub use ticks::{AlignedFloats, Period, Tick, Timestamps};
