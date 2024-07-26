#![warn(missing_docs)]
//! Welcome to Chartistry! This crate provides a flexible way to build charts in [Leptos](https://github.com/leptos-rs/leptos).
//!
//! All charts are built using the [Chart] fn. If you understand this function, you understand this library.
//!
//! ## Examples
//!
//! - See the [demo for Chartistry in action](https://feral-dot-io.github.io/leptos-chartistry/).
//! - There is also an [large, assorted list of examples](https://feral-dot-io.github.io/leptos-chartistry/examples.html) available.
//!
//! Below is an example chart:
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_chartistry::*;
//!
//! # use chrono::prelude::*;
//! # struct MyData { x: DateTime<Utc>, y1: f64, y2: f64 }
//! # fn load_data() -> Signal<Vec<MyData>> { Signal::default() }
//!
//! # #[component]
//! # fn SimpleChartComponent() -> impl IntoView {
//! let data: Signal<Vec<MyData>> = load_data(/* pull data from a resource */);
//! view! {
//!     <Chart
//!         // Sets the width and height
//!         aspect_ratio=AspectRatio::from_outer_ratio(600.0, 300.0)
//!
//!         // Decorate our chart
//!         top=RotatedLabel::middle("My garden")
//!         left=TickLabels::aligned_floats()
//!         right=Legend::end()
//!         bottom=TickLabels::timestamps()
//!         inner=[
//!             AxisMarker::left_edge().into_inner(),
//!             AxisMarker::bottom_edge().into_inner(),
//!             XGridLine::default().into_inner(),
//!             YGridLine::default().into_inner(),
//!             XGuideLine::over_data().into_inner(),
//!             YGuideLine::over_mouse().into_inner(),
//!         ]
//!         tooltip=Tooltip::left_cursor()
//!
//!         // Describe the data
//!         series=Series::new(|data: &MyData| data.x)
//!             .line(Line::new(|data: &MyData| data.y1).with_name("butterflies"))
//!             .line(Line::new(|data: &MyData| data.y2).with_name("dragonflies"))
//!         data=data
//!     />
//! }
//! # }
//! ```

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
pub use colours::{Colour, ColourScheme, DivergingGradient, SequentialGradient};
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
pub use series::{
    Bar, BarPlacement, Interpolation, Line, Marker, MarkerShape, Series, Stack, Step, BAR_GAP,
    BAR_GAP_INNER, DIVERGING_GRADIENT, LINEAR_GRADIENT, SERIES_COLOUR_SCHEME, STACK_COLOUR_SCHEME,
};
pub use ticks::{AlignedFloats, Period, Tick, Timestamps};
