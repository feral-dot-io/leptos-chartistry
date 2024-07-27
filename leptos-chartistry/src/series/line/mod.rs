mod interpolation;
mod marker;
pub use interpolation::{Interpolation, Step};
pub use marker::{Marker, MarkerShape};

use super::{ApplyUseSeries, IntoUseLine, SeriesAcc, UseData, UseY};
use crate::{
    colours::{Colour, DivergingGradient, LinearGradientSvg, SequentialGradient, BERLIN, LIPARI},
    series::GetYValue,
    ColourScheme, Tick,
};
use leptos::prelude::*;
use std::sync::Arc;

/// Suggested colour scheme for a linear gradient on a line. Uses darker colours for lower values and lighter colours for higher values. Assumes a light background.
pub const LINEAR_GRADIENT: SequentialGradient = LIPARI;

/// Suggested colour scheme for a diverging gradient on a line. Uses a blue for negative values, a dark central value and red for positive values. Assumes a light background.
pub const DIVERGING_GRADIENT: DivergingGradient = BERLIN;

/// Draws a line on the chart.
///
/// # Simple example
/// With no legend names, lines can be a simple closure:
/// ```rust
/// # use leptos_chartistry::*;
/// # struct MyData { x: f64, y1: f64, y2: f64 }
/// let series = Series::new(|data: &MyData| data.x)
///     .line(|data: &MyData| data.y1)
///     .line(|data: &MyData| data.y2);
/// ```
/// See this in action with the [tick labels example](https://feral-dot-io.github.io/leptos-chartistry/examples.html#tick-labels).
///
/// # Example
/// However, we can also set the name of the line which a legend can show:
/// ```rust
/// # use leptos_chartistry::*;
/// # struct MyData { x: f64, y1: f64, y2: f64 }
/// let series = Series::new(|data: &MyData| data.x)
///     .line(Line::new(|data: &MyData| data.y1).with_name("pears"))
///     .line(Line::new(|data: &MyData| data.y2).with_name("apples"));
/// ```
/// See this in action with the [legend example](https://feral-dot-io.github.io/leptos-chartistry/examples.html#legend).
pub struct Line<T, Y> {
    get_y: Arc<dyn GetYValue<T, Y>>,
    /// Name of the line. Used in the legend.
    pub name: RwSignal<String>,
    /// Colour of the line. If not set, the next colour in the series will be used.
    pub colour: RwSignal<Option<Colour>>,
    /// Use a linear gradient (colour scheme) for the line. Default is `None` with fallback to the line colour.
    pub gradient: RwSignal<Option<ColourScheme>>,
    /// Width of the line.
    pub width: RwSignal<f64>,
    /// Interpolation method of the line, aka line smoothing (or not). Describes how the line is drawn between two points. Default is [Interpolation::Monotone].
    pub interpolation: RwSignal<Interpolation>,
    /// Marker at each point on the line.
    pub marker: Marker,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UseLine {
    colour: Signal<Colour>,
    gradient: RwSignal<Option<ColourScheme>>,
    width: RwSignal<f64>,
    interpolation: RwSignal<Interpolation>,
    marker: Marker,
}

impl<T, Y> Line<T, Y> {
    /// Create a new line. The `get_y` function is used to extract the Y value from your struct.
    ///
    /// See the module documentation for examples.
    pub fn new(get_y: impl Fn(&T) -> Y + Send + Sync + 'static) -> Self
    where
        Y: Tick,
    {
        Self {
            get_y: Arc::new(get_y),
            name: RwSignal::default(),
            colour: RwSignal::default(),
            gradient: RwSignal::default(),
            width: RwSignal::new(1.0),
            interpolation: RwSignal::default(),
            marker: Marker::default(),
        }
    }

    /// Set the name of the line. Used in the legend.
    pub fn with_name(self, name: impl Into<String>) -> Self {
        self.name.set(name.into());
        self
    }

    /// Set the colour of the line. If not set, the next colour in the series will be used.
    pub fn with_colour(self, colour: impl Into<Option<Colour>>) -> Self {
        self.colour.set(colour.into());
        self
    }

    /// Use a colour scheme for the line. Interpolated in SVG by the browser, overrides [Colour]. Default is `None` with fallback to the line colour.
    ///
    /// Suggested use with [LINEAR_GRADIENT] or [DIVERGING_GRADIENT] (for data with a zero value).
    pub fn with_gradient(self, scheme: impl Into<ColourScheme>) -> Self {
        self.gradient.set(Some(scheme.into()));
        self
    }

    /// Set the width of the line.
    pub fn with_width(self, width: impl Into<f64>) -> Self {
        self.width.set(width.into());
        self
    }

    /// Set the interpolation method of the line.
    pub fn with_interpolation(self, interpolation: impl Into<Interpolation>) -> Self {
        self.interpolation.set(interpolation.into());
        self
    }

    /// Set the marker at each point on the line.
    pub fn with_marker(mut self, marker: impl Into<Marker>) -> Self {
        self.marker = marker.into();
        self
    }
}

impl<T, Y> Clone for Line<T, Y> {
    fn clone(&self) -> Self {
        Self {
            get_y: self.get_y.clone(),
            name: self.name,
            colour: self.colour,
            gradient: self.gradient,
            width: self.width,
            interpolation: self.interpolation,
            marker: self.marker.clone(),
        }
    }
}

impl<T, Y: Tick, F: Fn(&T) -> Y + Send + Sync + 'static> From<F> for Line<T, Y> {
    fn from(f: F) -> Self {
        Self::new(f)
    }
}

impl<T, Y: Tick, U: Fn(&T) -> Y + Send + Sync> GetYValue<T, Y> for U {
    fn value(&self, t: &T) -> Y {
        self(t)
    }

    fn cumulative_value(&self, t: &T) -> Y {
        self(t)
    }
}

impl<T, Y> ApplyUseSeries<T, Y> for Line<T, Y> {
    fn apply_use_series(self: Arc<Self>, series: &mut SeriesAcc<T, Y>) {
        let colour = series.next_colour();
        _ = series.push_line(colour, (*self).clone());
    }
}

impl<T, Y> IntoUseLine<T, Y> for Line<T, Y> {
    fn into_use_line(self, id: usize, colour: Memo<Colour>) -> (UseY, Arc<dyn GetYValue<T, Y>>) {
        let override_colour = self.colour;
        let colour = Signal::derive(move || override_colour.get().unwrap_or(colour.get()));
        let line = UseY::new_line(
            id,
            self.name,
            UseLine {
                colour,
                gradient: self.gradient,
                width: self.width,
                interpolation: self.interpolation,
                marker: self.marker.clone(),
            },
        );
        (line, self.get_y.clone())
    }
}

#[component]
pub fn RenderLine<X: 'static, Y: Send + Sync + 'static>(
    use_y: UseY,
    line: UseLine,
    data: UseData<X, Y>,
    positions: Signal<Vec<(f64, f64)>>,
    markers: Signal<Vec<(f64, f64)>>,
) -> impl IntoView {
    let path = move || positions.with(|positions| line.interpolation.get().path(positions));

    // Line colour
    let gradient_id = format!("line_{}_gradient", use_y.id);
    let stroke = {
        let colour = line.colour;
        let gradient_id = gradient_id.clone();
        Signal::derive(move || {
            // Gradient takes precedence
            if line.gradient.get().is_some() {
                format!("url(#{gradient_id})")
            } else {
                colour.get().to_string()
            }
        })
    };
    let gradient = Signal::derive(move || {
        line.gradient
            .get()
            .unwrap_or_else(|| LINEAR_GRADIENT.into())
    });
    let range_y = Signal::derive(move || data.range_y.read().positions());

    let width = line.width;
    view! {
        <g
            class="_chartistry_line"
            stroke=stroke
            stroke-linecap="round"
            stroke-linejoin="bevel"
            stroke-width=width>
            <defs>
                <Show when=move || line.gradient.get().is_some()>
                    <LinearGradientSvg
                        id=gradient_id.clone()
                        scheme=gradient
                        range_y=range_y />
                </Show>
            </defs>
            <path d=path fill="none" />
            <marker::LineMarkers line=line positions=markers />
        </g>
    }
}
