mod interpolation;
mod marker;
pub use interpolation::{Interpolation, Step};
pub use marker::{Marker, MarkerShape};

use super::{use_y::UseYDesc, ApplyUseSeries, IntoUseY, SeriesAcc, UseData, UseY};
use crate::{
    bounds::Bounds,
    colours::{Colour, DivergingGradient, LinearGradientSvg, SequentialGradient, BERLIN, LIPARI},
    debug::DebugRect,
    series::GetYValue,
    state::State,
    ColourScheme,
};
use leptos::*;
use std::rc::Rc;

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
    get_y: Rc<dyn GetYValue<T, Y>>,
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
    pub fn new(get_y: impl Fn(&T) -> Y + 'static) -> Self {
        Self {
            get_y: Rc::new(get_y),
            name: RwSignal::default(),
            colour: RwSignal::default(),
            gradient: RwSignal::default(),
            width: 1.0.into(),
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

impl<T, Y, F: Fn(&T) -> Y + 'static> From<F> for Line<T, Y> {
    fn from(f: F) -> Self {
        Self::new(f)
    }
}

impl<T, Y, U: Fn(&T) -> Y> GetYValue<T, Y> for U {
    fn value(&self, t: &T) -> Y {
        self(t)
    }

    fn cumulative_value(&self, t: &T) -> Y {
        self(t)
    }
}

impl<T, Y> ApplyUseSeries<T, Y> for Line<T, Y> {
    fn apply_use_series(self: Rc<Self>, series: &mut SeriesAcc<T, Y>) {
        let colour = series.next_colour();
        _ = series.push(colour, (*self).clone());
    }
}

impl<T, Y> IntoUseY<T, Y> for Line<T, Y> {
    fn into_use_y(self, id: usize, colour: Memo<Colour>) -> (UseY, Rc<dyn GetYValue<T, Y>>) {
        let override_colour = self.colour;
        let colour = Signal::derive(move || override_colour.get().unwrap_or(colour.get()));
        let line = UseY {
            id,
            name: self.name,
            desc: UseYDesc::Line(UseLine {
                colour,
                gradient: self.gradient,
                width: self.width,
                interpolation: self.interpolation,
                marker: self.marker.clone(),
            }),
        };
        (line, self.get_y.clone())
    }
}

impl UseLine {
    fn taster_bounds(font_height: Memo<f64>, font_width: Memo<f64>) -> Memo<Bounds> {
        create_memo(move |_| Bounds::new(font_width.get() * 2.5, font_height.get()))
    }

    pub fn snippet_width(font_height: Memo<f64>, font_width: Memo<f64>) -> Signal<f64> {
        let taster_bounds = Self::taster_bounds(font_height, font_width);
        Signal::derive(move || taster_bounds.get().width() + font_width.get())
    }
}

#[component]
pub fn RenderLine<X: 'static, Y: 'static>(
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
    let gradient = move || {
        line.gradient
            .get()
            .unwrap_or_else(|| LINEAR_GRADIENT.into())
    };

    view! {
        <g
            class="_chartistry_line"
            stroke=stroke
            stroke-linecap="round"
            stroke-linejoin="bevel"
            stroke-width=line.width>
            <defs>
                <Show when=move || line.gradient.get().is_some()>
                    <LinearGradientSvg
                        id=gradient_id.clone()
                        scheme=gradient
                        range=data.position_range />
                </Show>
            </defs>
            <path d=path fill="none" />
            <marker::LineMarkers line=line positions=markers />
        </g>
    }
}

#[component]
pub fn Snippet<X: 'static, Y: 'static>(series: UseY, state: State<X, Y>) -> impl IntoView {
    let debug = state.pre.debug;
    let name = series.name;
    view! {
        <div class="_chartistry_snippet" style="white-space: nowrap;">
            <DebugRect label="snippet" debug=debug />
            <Taster series=series state=state />
            {name}
        </div>
    }
}

#[component]
fn Taster<X: 'static, Y: 'static>(series: UseY, state: State<X, Y>) -> impl IntoView {
    const Y_OFFSET: f64 = 2.0;
    let debug = state.pre.debug;
    let font_width = state.pre.font_width;
    let right_padding = Signal::derive(move || font_width.get() / 2.0);
    let bounds = UseLine::taster_bounds(state.pre.font_height, font_width);
    // Mock positions from left to right of our bounds
    let positions = Signal::derive(move || {
        let bounds = bounds.get();
        let y = bounds.centre_y() + Y_OFFSET;
        vec![(bounds.left_x(), y), (bounds.right_x(), y)]
    });
    // One marker in the middle
    let markers = Signal::derive(move || {
        let bounds = bounds.get();
        vec![(bounds.centre_x(), bounds.centre_y() + Y_OFFSET)]
    });

    let render_desc = match &series.desc {
        UseYDesc::Line(line) => view! {
            <RenderLine
                use_y=series.clone()
                line=line.clone()
                data=state.pre.data
                positions=positions
                markers=markers />
        },
    };

    view! {
        <svg
            viewBox=move || format!("0 0 {} {}", bounds.get().width(), bounds.get().height())
            width=move || bounds.get().width() + right_padding.get()
            height=move || bounds.get().height()
            class="_chartistry_taster"
            style="box-sizing: border-box;"
            style:padding-right=move || format!("{}px", right_padding.get())
            >
            <DebugRect label="taster" debug=debug bounds=vec![bounds.into()] />
            {render_desc}
        </svg>
    }
}
