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

/// Describes a line point marker.
#[derive(Clone, Debug, PartialEq)]
pub struct Marker {
    /// Shape of the marker. Default is no marker.
    pub shape: RwSignal<MarkerShape>,
    /// Size of the marker relative to the line width. Default is 1.0.
    pub scale: RwSignal<f64>,
    /// Colour of the marker border. Set to the same as the background to separate the marker from the line. Default is white.
    pub border: RwSignal<Colour>,
    /// Width of the marker border. Set to zero to remove the border. Default is zero.
    pub border_width: RwSignal<f64>,
}

/// Shape of a line marker.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum MarkerShape {
    /// No marker.
    None,
    /// Circle marker.
    #[default] // TODO
    Circle,
    /// Square marker.
    Square,
    /// Diamond marker.
    Diamond,
    /// Triangle marker.
    Triangle,
    /// Plus marker.
    Plus,
    /// Cross marker.
    Cross,
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

impl Default for Marker {
    fn default() -> Self {
        Self {
            shape: RwSignal::default(),
            scale: create_rw_signal(1.0),
            border: create_rw_signal(Colour::new(0, 0, 255)), // TODO
            border_width: create_rw_signal(0.0),              // TODO
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
    view! {
        <g class="_chartistry_line" stroke=colour>
            <path
                d=path
                fill="none"
                stroke=colour
                stroke-width=width
            />
            <LineMarkers line=line positions=positions />
        </g>
    }
}

#[component]
fn LineMarkers(line: UseLine, positions: Signal<Vec<(f64, f64)>>) -> impl IntoView {
    let marker = line.marker.clone();
    let marker_id = format!("line_{}_marker", line.id);

    // Disable border if no marker
    let border_width = Signal::derive(move || {
        if marker.shape.get() == MarkerShape::None {
            0.0
        } else {
            marker.border_width.get()
        }
    });
    // Our view box is around -1 to 1. Add a border around that.
    let viewBox = move || {
        let border = border_width.get();
        // -1 -1 2 2 with border
        format!(
            "{min} {min} {size} {size}",
            min = -1.0 - border,
            size = 2.0 + border * 2.0
        )
    };
    // Size of our marker: proportionate to our line width plus border
    let size = create_memo(move |_| {
        line.width.get() * WIDTH_TO_MARKER * marker.scale.get() + border_width.get() * 2.0
    });

    let markers = {
        let marker_id = marker_id.clone();
        move || {
            positions.with(|positions| {
                positions
                    .iter()
                    .filter(|(x, y)| !(x.is_nan() || y.is_nan()))
                    .map(|&(x, y)| {
                        view! {
                            <use_
                                href=format!("#{marker_id}")
                                x=move || x - size.get() / 2.0
                                y=move || y - size.get() / 2.0
                                width=size
                                height=size />
                        }
                    })
                    .collect_view()
            })
        }
    };

    view! {
        <g
            fill=move || line.colour.get().to_string()
            stroke=move || marker.border.get().to_string()
            stroke-width=move || border_width.get()
            class="_chartistry_line_markers">
            <symbol id=marker_id viewBox=viewBox>
                {move || view!(<RenderMarkerShape shape=marker.shape.get() />)}
            </symbol>
            {markers}
        </g>
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
            <circle cx=0 cy=0 r=1 />
        }
        .into_view(),

        MarkerShape::Triangle => view! {
            <polygon points="0,-1 -1,1 1,1" />
        }
        .into_view(),

        MarkerShape::Square => view! {
            <polygon points="0,-1 -1,0 0,1 1,0" transform="rotate(45)" />
        }
        .into_view(),

        MarkerShape::Diamond => view! {
            <polygon points="0,-1 -1,0 0,1 1,0" />
        }
        .into_view(),

        MarkerShape::Plus => view! {
            <PlusPath />
        }
        .into_view(),

        MarkerShape::Cross => view! {
            <PlusPath rotate=45 />
        }
        .into_view(),
    }
}

// Outline of a big plus (like the Swiss flag) up against the edge (-1 to 1)
#[component]
fn PlusPath(#[prop(into, optional)] rotate: f64) -> impl IntoView {
    const OFFSET: f64 = 2.0 / 3.0;
    const WIDTH: f64 = 0.4;
    view! {
        <path
            transform=format!("rotate({})", rotate)
            d=format!("M {} {} h {} v {} h {} v {} h {} v {} h {} v {} h {} v {} h {} Z",
                -WIDTH / 2.0, -1, // Top-most left
                WIDTH, // Top-most right
                OFFSET,
                OFFSET, // Right-most top
                WIDTH, // Right-most bottom
                -OFFSET,
                OFFSET, // Bottom-most right
                -WIDTH, // Bottom-most left
                -OFFSET,
                -OFFSET, // Left-most bottom
                -WIDTH, // Left-most top
                OFFSET) />
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
