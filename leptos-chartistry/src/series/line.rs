use super::{ApplyUseSeries, IntoUseLine, SeriesAcc};
use crate::{bounds::Bounds, colours::Colour, debug::DebugRect, series::GetYValue, state::State};
use leptos::*;
use std::rc::Rc;

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
    /// Width of the line.
    pub width: RwSignal<f64>,
    /// Line point marker.
    pub marker: Marker,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Marker {
    pub shape: RwSignal<MarkerShape>,
    pub size: RwSignal<Option<f64>>,
    pub spacing: RwSignal<f64>,
    pub colour: RwSignal<Option<Colour>>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum MarkerShape {
    None,
    Circle,
    Triangle,
    Square,
    Diamond,
    #[default]
    Plus,
    //Cross,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UseLine {
    pub id: usize,
    pub name: RwSignal<String>,
    colour: Signal<Colour>,
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

    /// Set the width of the line.
    pub fn with_width(self, width: impl Into<f64>) -> Self {
        self.width.set(width.into());
        self
    }
}

impl<T, Y> Clone for Line<T, Y> {
    fn clone(&self) -> Self {
        Self {
            get_y: self.get_y.clone(),
            name: self.name,
            colour: self.colour,
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
            width: self.width,
            marker: self.marker,
        };
        (line, self.get_y.clone())
    }
}

impl UseLine {
    pub fn taster_bounds(font_height: Memo<f64>, font_width: Memo<f64>) -> Memo<Bounds> {
        create_memo(move |_| Bounds::new(font_width.get() * 2.0, font_height.get()))
    }

    pub fn snippet_width(font_height: Memo<f64>, font_width: Memo<f64>) -> Signal<f64> {
        let taster_bounds = Self::taster_bounds(font_height, font_width);
        Signal::derive(move || taster_bounds.get().width() + font_width.get())
    }

    pub fn taster(&self, bounds: Memo<Bounds>) -> View {
        view!( <LineTaster line=self.clone() bounds=bounds /> )
    }

    pub(crate) fn render(&self, positions: Signal<Vec<(f64, f64)>>) -> View {
        view!( <RenderLine line=self.clone() positions=positions /> )
    }
}

#[component]
fn LineTaster(line: UseLine, bounds: Memo<Bounds>) -> impl IntoView {
    let colour = line.colour;
    view! {
        <line
            x1=move || bounds.get().left_x()
            x2=move || bounds.get().right_x()
            y1=move || bounds.get().centre_y() + 1.0
            y2=move || bounds.get().centre_y() + 1.0
            stroke=move || colour.get().to_string()
            stroke-width=line.width
        />
    }
}

#[component]
pub fn RenderLine(line: UseLine, positions: Signal<Vec<(f64, f64)>>) -> impl IntoView {
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

    // Derive colours
    let colour = line.colour;
    let marker_colour = {
        let marker_colour = line.marker.colour;
        Signal::derive(move || marker_colour.get().unwrap_or(colour.get()).to_string())
    };
    let line_colour = Signal::derive(move || colour.get().to_string());

    let markers = move || {
        let shape = line.marker.shape.get();
        let diameter = line.marker.size.get().unwrap_or(line.width.get() * 7.0);
        let radius = diameter / 2.0;

        positions.with(|positions| {
            positions
                .iter()
                .filter(|(x, y)| !x.is_nan() && !y.is_nan())
                // Draw shape around centre (x, y)
                .map(|&(x, y)| match shape {
                    MarkerShape::None => ().into_view(),
                    MarkerShape::Circle => view! {
                    <circle
                        cx=x
                        cy=y
                        r=move || diameter / 2.0 />
                    }
                    .into_view(),
                    MarkerShape::Triangle => view! {
                        <polygon
                            points=format!("{},{} {},{} {},{}",
                                x, y - radius,
                                x - radius, y + radius,
                                x + radius, y + radius) />
                    }
                    .into_view(),
                    MarkerShape::Square => view! {
                    <rect
                        x=x - radius
                        y=y - radius
                        width=diameter
                        height=diameter />
                    }
                    .into_view(),
                    MarkerShape::Diamond => view! {
                        <polygon
                            points=format!("{},{} {},{} {},{} {},{}",
                                x, y - radius,
                                x - radius, y,
                                x, y + radius,
                                x + radius, y) />
                    }
                    .into_view(),
                    MarkerShape::Plus => {
                        let third = diameter / 3.0;
                        view! {
                            // Outline of a big plus (like the Swiss flag) up against the edge
                            <path
                                d=format!("M {} {} h {} v {} h {} v {} h {} v {} h {} v {} h {} v {} h {} Z",
                                    x - radius + third, y - radius, // Top-most left
                                    third, // Top-most right
                                    third,
                                    third, // Right-most top
                                    third, // Right-most bottom
                                    -third,
                                    third, // Bottom-most right
                                    -third, // Bottom-most left
                                    -third,
                                    -third, // Left-most bottom
                                    -third, // Left-most top
                                    third) />
                        }
                        .into_view()
                    }
                })
                .collect_view()
        })
    };

    view! {
        <g class="_chartistry_line">
            <path
                d=path
                fill="none"
                stroke=line_colour
                stroke-width=line.width
            />
            <g class="_chartistry_line_markers" fill=marker_colour>
                {markers}
            </g>
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
pub fn Taster<X: 'static, Y: 'static>(series: UseLine, state: State<X, Y>) -> impl IntoView {
    let debug = state.pre.debug;
    let font_width = state.pre.font_width;
    let bounds = UseLine::taster_bounds(state.pre.font_height, font_width);
    view! {
        <svg
            class="_chartistry_taster"
            width=move || bounds.get().width() + font_width.get()
            height=move || bounds.get().height()
            viewBox=move || format!("0 0 {} {}", bounds.get().width(), bounds.get().height())
            style="box-sizing: border-box;"
            style:padding-right=move || format!("{}px", font_width.get())>
            <DebugRect label="taster" debug=debug bounds=vec![bounds.into()] />
            {series.taster(bounds)}
        </svg>
    }
}
