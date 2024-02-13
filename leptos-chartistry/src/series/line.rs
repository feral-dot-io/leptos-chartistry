use super::{ApplyUseSeries, IntoUseLine, SeriesAcc};
use crate::{
    bounds::Bounds,
    colours::{self, Colour},
    debug::DebugRect,
    series::GetYValue,
    state::State,
};
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
    pub marker: Marker,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Marker {
    pub shape: RwSignal<MarkerShape>,
    pub scale: RwSignal<f64>,
    pub border: RwSignal<Colour>,
    pub border_width: RwSignal<f64>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum MarkerShape {
    None,
    #[default]
    Circle,
    Square,
    Triangle,
    Diamond,
    Plus,
    Cross,
}

const ALL_MARKER_SHAPES: &[MarkerShape] = &[
    MarkerShape::None,
    MarkerShape::Circle,
    MarkerShape::Square,
    MarkerShape::Triangle,
    MarkerShape::Diamond,
    MarkerShape::Plus,
    MarkerShape::Cross,
];

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

impl Default for Marker {
    fn default() -> Self {
        Self {
            shape: RwSignal::default(),
            scale: create_rw_signal(1.0),
            border: create_rw_signal(colours::WHITE),
            border_width: create_rw_signal(1.0),
        }
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
            marker: self.marker.clone(),
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
    let marker_url =
        Signal::derive(move || format!("url(#{})", line.marker.shape.get().id(line.id)));
    view! {
        <g class="_chartistry_line" stroke=colour>
            <defs>
                <MarkerDefs line=line />
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
fn MarkerDefs(line: UseLine) -> impl IntoView {
    let marker = line.marker;
    let border_colour = Signal::derive(move || marker.border.get().to_string());
    // Do we have a border? Yes if >0 and not Marker::None.
    let border_width = create_memo(move |_| {
        if marker.shape.get() == MarkerShape::None {
            0.0
        } else {
            marker.border_width.get()
        }
    });

    // Our view box is around -1 to 1. Add a border around that.
    let viewBox = move || {
        let border = border_width.get();
        format!(
            "{min} {min} {size} {size}",
            min = -1.0 - border,
            size = 2.0 + border * 2.0
        )
    };
    // Calculate width as line + border
    let width =
        Signal::derive(move || line.width.get() * WIDTH_TO_MARKER + border_width.get() * 4.0);

    ALL_MARKER_SHAPES
        .iter()
        .map(|&shape| {
            view! {
                <marker
                    id=shape.id(line.id)
                    viewBox=viewBox
                    markerUnits="userSpaceOnUse"
                    markerWidth=width
                    markerHeight=width>
                    <circle cx=0 cy=0 r=border_width fill=border_colour stroke="none" />
                    <RenderMarkerShape shape=shape />
                </marker>
            }
        })
        .collect_view()
}

impl MarkerShape {
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

impl std::str::FromStr for MarkerShape {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(MarkerShape::None),
            "circle" => Ok(MarkerShape::Circle),
            "triangle" => Ok(MarkerShape::Triangle),
            "square" => Ok(MarkerShape::Square),
            "diamond" => Ok(MarkerShape::Diamond),
            "plus" => Ok(MarkerShape::Plus),
            "cross" => Ok(MarkerShape::Cross),
            _ => Err("unknown marker"),
        }
    }
}

impl std::fmt::Display for MarkerShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarkerShape::None => write!(f, "None"),
            MarkerShape::Circle => write!(f, "Circle"),
            MarkerShape::Triangle => write!(f, "Triangle"),
            MarkerShape::Square => write!(f, "Square"),
            MarkerShape::Diamond => write!(f, "Diamond"),
            MarkerShape::Plus => write!(f, "Plus"),
            MarkerShape::Cross => write!(f, "Cross"),
        }
    }
}

/// Renders the marker shape in -1 to 1 space. They should all be similar in size and not just extend to the edge e.g., square is a rotated diamond.
#[component]
fn RenderMarkerShape(shape: MarkerShape) -> impl IntoView {
    match shape {
        MarkerShape::None => ().into_view(),

        MarkerShape::Circle => view! {
            <circle cx=0 cy=0 r=1 fill="context-stroke" stroke="none" />
        }
        .into_view(),

        MarkerShape::Triangle => view! {
            <polygon points="0,-1 -1,1 1,1" fill="context-stroke" stroke="none" />
        }
        .into_view(),

        MarkerShape::Square => view! {
            <polygon points="0,-1 -1,0 0,1 1,0" transform="rotate(45)" fill="context-stroke" stroke="none" />
        }
        .into_view(),

        MarkerShape::Diamond => view! {
            <polygon points="0,-1 -1,0 0,1 1,0" fill="context-stroke" stroke="none" />
        }
        .into_view(),

        MarkerShape::Plus => view! {
            <line x1="-1" y1="0" x2="1" y2="0" fill="none" stroke="context-stroke" stroke-width="0.5" />
            <line x1="0" y1="-1" x2="0" y2="1" fill="none" stroke="context-stroke" stroke-width="0.5" />
        }
        .into_view(),

        MarkerShape::Cross => view! {
            <line
                x1="-1" y1="0" x2="1" y2="0"
                transform="rotate(45)"
                stroke-width="0.5"
                fill="none"
                stroke="context-stroke" />
            <line
                x1="0" y1="-1" x2="0" y2="1"
                transform="rotate(45)"
                stroke-width="0.5"
                fill="none"
                stroke="context-stroke" />
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
