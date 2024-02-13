use super::{ApplyUseSeries, IntoUseLine, SeriesAcc};
use crate::{bounds::Bounds, colours::Colour, debug::DebugRect, series::GetYValue, state::State};
use leptos::*;
use std::rc::Rc;

const WIDTH_TO_MARKER: f64 = 7.0;

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
    /// Marker at each point on the line.
    pub marker: RwSignal<Marker>,
    pub marker_border: RwSignal<Option<Colour>>,
    pub marker_border_width: RwSignal<f64>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum Marker {
    None,
    #[default]
    Circle,
    Triangle,
    Square,
    Diamond,
    Plus,
    Cross,
}

const ALL_MARKERS: &[Marker] = &[
    Marker::None,
    Marker::Circle,
    Marker::Triangle,
    Marker::Square,
    Marker::Diamond,
    Marker::Plus,
    Marker::Cross,
];

#[derive(Clone, Debug, PartialEq)]
pub struct UseLine {
    pub id: usize,
    pub name: RwSignal<String>,
    colour: Signal<Colour>,
    width: RwSignal<f64>,
    marker: RwSignal<Marker>,
    border: RwSignal<Option<Colour>>,
    border_width: RwSignal<f64>,
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
            marker: RwSignal::default(),
            marker_border: create_rw_signal(Some(Colour::new(255, 255, 255))),
            marker_border_width: create_rw_signal(1.0),
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
            marker: self.marker,
            marker_border: self.marker_border,
            marker_border_width: self.marker_border_width,
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
            border: self.marker_border,
            border_width: self.marker_border_width,
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

    let width = line.width;
    let colour = line.colour;
    let colour = Signal::derive(move || colour.get().to_string());
    let border = Signal::derive(move || {
        line.border
            .get()
            .map(|c| c.to_string())
            .unwrap_or("none".to_string())
    });
    let marker_url = Signal::derive(move || format!("url(#{})", line.marker.get().id(line.id)));
    view! {
        <g class="_chartistry_line" stroke=colour>
            <defs>
                <MarkerDefs line=line border=border />
            </defs>
            <path
                d=path
                fill="none"
                stroke=colour
                stroke-width=width
                marker-start=marker_url
                marker-mid=marker_url
                marker-end=marker_url
            />
        </g>
    }
}

#[component]
fn MarkerDefs(line: UseLine, border: Signal<String>) -> impl IntoView {
    let viewBox = move || {
        let border = line.border_width.get();
        format!(
            "{min} {min} {size} {size}",
            min = -1.0 - border,
            size = 2.0 + border * 2.0
        )
    };
    let width =
        Signal::derive(move || line.width.get() * WIDTH_TO_MARKER + line.border_width.get() * 4.0);
    ALL_MARKERS
        .iter()
        .map(|&shape| {
            view! {
                <marker
                    id=shape.id(line.id)
                    viewBox=viewBox
                    markerUnits="userSpaceOnUse"
                    markerWidth=width
                    markerHeight=width>
                    <circle cx=0 cy=0 r=2 fill=border stroke="none" />
                    <RenderMarkerShape shape=shape />
                </marker>
            }
        })
        .collect_view()
}

impl Marker {
    fn id(self, line_id: usize) -> String {
        format!("line_{}_marker_{}", line_id, self.label())
    }

    fn label(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Circle => "circle",
            Self::Triangle => "triangle",
            Self::Square => "square",
            Self::Diamond => "diamond",
            Self::Plus => "plus",
            Self::Cross => "cross",
        }
    }
}

impl std::str::FromStr for Marker {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Marker::None),
            "circle" => Ok(Marker::Circle),
            "triangle" => Ok(Marker::Triangle),
            "square" => Ok(Marker::Square),
            "diamond" => Ok(Marker::Diamond),
            "plus" => Ok(Marker::Plus),
            "cross" => Ok(Marker::Cross),
            _ => Err("unknown marker"),
        }
    }
}

impl std::fmt::Display for Marker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Marker::None => write!(f, "None"),
            Marker::Circle => write!(f, "Circle"),
            Marker::Triangle => write!(f, "Triangle"),
            Marker::Square => write!(f, "Square"),
            Marker::Diamond => write!(f, "Diamond"),
            Marker::Plus => write!(f, "Plus"),
            Marker::Cross => write!(f, "Cross"),
        }
    }
}

/// Renders the marker shape in -1 to 1 space.
#[component]
fn RenderMarkerShape(shape: Marker) -> impl IntoView {
    match shape {
        Marker::None => ().into_view(),

        Marker::Circle => view! {
            <circle cx=0 cy=0 r=1 fill="context-stroke" stroke="none" />
        }
        .into_view(),

        Marker::Triangle => view! {
            <polygon points="0,-1 -1,1 1,1" fill="context-stroke" stroke="none" />
        }
        .into_view(),

        Marker::Square => view! {
            <rect x="-1" y="-1" width="2" height="2" fill="context-stroke" stroke="none" />
        }
        .into_view(),

        Marker::Diamond => view! {
            <polygon points="0,-1 -1,0 0,1 1,0" fill="context-stroke" stroke="none" />
        }
        .into_view(),

        Marker::Plus => view! {
            <line x1="-1" y1="0" x2="1" y2="0" fill="none" stroke="context-stroke" stroke-width="0.5" />
            <line x1="0" y1="-1" x2="0" y2="1" fill="none" stroke="context-stroke" stroke-width="0.5" />
        }
        .into_view(),

        Marker::Cross => view! {
            <line x1="-1" y1="-1" x2="1" y2="1" fill="none" stroke="context-stroke" stroke-width="0.5" />
            <line x1="-1" y1="1" x2="1" y2="-1" fill="none" stroke="context-stroke" stroke-width="0.5" />
        }
        .into_view(),
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
