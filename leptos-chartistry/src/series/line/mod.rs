mod marker;
pub use marker::{Marker, MarkerShape};

use super::{ApplyUseSeries, IntoUseLine, SeriesAcc, UseData};
use crate::{
    bounds::Bounds,
    colours::{Colour, LinearGradient, LIPARI},
    debug::DebugRect,
    series::GetYValue,
    state::State,
    ColourScheme,
};
use leptos::*;
use std::rc::Rc;

/// Suggested colour scheme for linear gradient. Assumes a light background with dark values for high values.
pub const LINEAR_GRADIENT: [Colour; 10] = LIPARI;

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
    /// Use a colour scheme for the line. Interpolated in SVG by the browser, overrides [Colour]. Default is `None` with fallback to the line colour.
    pub gradient: RwSignal<Option<ColourScheme>>,
    /// Width of the line.
    pub width: RwSignal<f64>,
    /// Marker at each point on the line.
    pub marker: Marker,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UseLine {
    pub id: usize,
    pub name: RwSignal<String>,
    colour: Signal<Colour>,
    gradient: RwSignal<Option<ColourScheme>>,
    width: RwSignal<f64>,
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
    pub fn with_gradient(self, scheme: impl Into<ColourScheme>) -> Self {
        self.gradient.set(Some(scheme.into()));
        self
    }

    /// Set the width of the line.
    pub fn with_width(self, width: impl Into<f64>) -> Self {
        self.width.set(width.into());
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

impl<T, Y> IntoUseLine<T, Y> for Line<T, Y> {
    fn into_use_line(self, id: usize, colour: Memo<Colour>) -> (UseLine, Rc<dyn GetYValue<T, Y>>) {
        let override_colour = self.colour;
        let colour = Signal::derive(move || override_colour.get().unwrap_or(colour.get()));
        let line = UseLine {
            id,
            name: self.name,
            colour,
            gradient: self.gradient,
            width: self.width,
            marker: self.marker.clone(),
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

    pub(crate) fn render<X: 'static, Y: 'static>(
        &self,
        data: UseData<X, Y>,
        positions: Signal<Vec<(f64, f64)>>,
    ) -> View {
        view!( <RenderLine data=data line=self.clone() positions=positions markers=positions /> )
    }
}

#[component]
pub fn RenderLine<X: 'static, Y: 'static>(
    data: UseData<X, Y>,
    line: UseLine,
    positions: Signal<Vec<(f64, f64)>>,
    markers: Signal<Vec<(f64, f64)>>,
) -> impl IntoView {
    let path = move || {
        positions.with(|positions| {
            let mut need_move = true;
            positions
                .iter()
                .map(|(x, y)| {
                    if x.is_nan() || y.is_nan() {
                        need_move = true;
                        "".to_string()
                    } else if need_move {
                        need_move = false;
                        format!("M {} {} ", x, y)
                    } else {
                        format!("L {} {} ", x, y)
                    }
                })
                .collect::<String>()
        })
    };

    // Line colour
    let gradient_id = format!("line_{}_gradient", line.id);
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
        <g class="_chartistry_line" stroke=stroke>
            <defs>
                <Show when=move || line.gradient.get().is_some()>
                    <LinearGradient
                        id=gradient_id.clone()
                        colour=gradient
                        range=data.position_range />
                </Show>
            </defs>
            <path d=path fill="none" stroke-width=line.width />
            <marker::LineMarkers line=line positions=markers />
        </g>
    }
}

#[component]
pub fn Snippet<X: 'static, Y: 'static>(series: UseLine, state: State<X, Y>) -> impl IntoView {
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
fn Taster<X: 'static, Y: 'static>(series: UseLine, state: State<X, Y>) -> impl IntoView {
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
            <RenderLine data=state.pre.data line=series positions=positions markers=markers />
        </svg>
    }
}
