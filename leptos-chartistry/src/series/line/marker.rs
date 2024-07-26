use super::UseLine;
use crate::colours::Colour;
use leptos::prelude::*;

// Scales our marker (drawn -1 to 1) to a 1.0 line width
const WIDTH_TO_MARKER: f64 = 8.0;

/// Describes a line point marker.
#[derive(Clone, Debug, PartialEq)]
pub struct Marker {
    /// Shape of the marker. Default is no marker.
    pub shape: RwSignal<MarkerShape>,
    /// Colour of the marker. Default is line colour.
    pub colour: RwSignal<Option<Colour>>,
    /// Size relative to the line width. Default is 1.0.
    pub scale: RwSignal<f64>,
    /// Colour of the border. Set to the same as the background to separate the marker from the line. Default is the line colour.
    pub border: RwSignal<Option<Colour>>,
    /// Width of the border. Zero removes the border. Default is zero.
    pub border_width: RwSignal<f64>,
}

/// Shape of a line marker.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum MarkerShape {
    /// No marker.
    #[default]
    None,
    /// Circle marker.
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

impl Default for Marker {
    fn default() -> Self {
        Self {
            shape: RwSignal::default(),
            colour: RwSignal::default(),
            scale: create_rw_signal(1.0),
            border: RwSignal::default(),
            border_width: create_rw_signal(0.0),
        }
    }
}

impl From<MarkerShape> for Marker {
    fn from(shape: MarkerShape) -> Self {
        Self::from_shape(shape)
    }
}

impl Marker {
    /// Create a new marker with the given shape.
    pub fn from_shape(shape: impl Into<MarkerShape>) -> Self {
        Self {
            shape: create_rw_signal(shape.into()),
            ..Default::default()
        }
    }

    /// Set the colour of the marker. Default is line colour.
    pub fn with_colour(self, colour: impl Into<Option<Colour>>) -> Self {
        self.colour.set(colour.into());
        self
    }

    /// Set the size of the marker relative to the line width. Default is 1.0.
    pub fn with_scale(self, scale: impl Into<f64>) -> Self {
        self.scale.set(scale.into());
        self
    }

    /// Set the colour of the marker border. Set to the same as the background to separate the marker from the line. Default is white.
    pub fn with_border(self, border: impl Into<Option<Colour>>) -> Self {
        self.border.set(border.into());
        self
    }

    /// Set the width of the marker border. Set to zero to remove the border. Default is zero.
    pub fn with_border_width(self, border_width: impl Into<f64>) -> Self {
        self.border_width.set(border_width.into());
        self
    }
}

#[component]
pub(super) fn LineMarkers(line: UseLine, positions: Signal<Vec<(f64, f64)>>) -> impl IntoView {
    let marker = line.marker.clone();

    // Disable border if no marker
    let border_width = Signal::derive(move || {
        if marker.shape.get() == MarkerShape::None {
            0.0
        } else {
            marker.border_width.get()
        }
    });

    let markers = move || {
        // Size of our marker: proportionate to our line width
        let line_width = line.width.get();
        let diameter = line_width * WIDTH_TO_MARKER * marker.scale.get();

        positions.with(|positions| {
            positions
                .iter()
                .filter(|(x, y)| !(x.is_nan() || y.is_nan()))
                .map(|&(x, y)| {
                    view! {
                        <MarkerShape
                            shape=marker.shape.get()
                            x=x
                            y=y
                            diameter=diameter
                            line_width=line_width />
                    }
                })
                .collect_view()
        })
    };

    view! {
        <g
            fill=move || marker.colour.get().unwrap_or_else(|| line.colour.get()).to_string()
            stroke=move || marker.border.get().unwrap_or_else(|| line.colour.get()).to_string()
            stroke-width=move || border_width.get() * 2.0 // Half of the stroke is inside
            class="_chartistry_line_markers">
            {markers}
        </g>
    }
}

/// Renders the marker shape in a square. They should all be similar in size and not just extend to the edge e.g., square is a rotated diamond.
#[component]
fn MarkerShape(
    shape: MarkerShape,
    x: f64,
    y: f64,
    diameter: f64,
    line_width: f64,
) -> impl IntoView {
    let radius = diameter / 2.0;
    match shape {
        MarkerShape::None => ().into_view(),

        MarkerShape::Circle => view! {
            // Radius to fit inside our square / diamond -- not the viewbox rect
            <circle
                cx=x
                cy=y
                r=(45.0_f64).to_radians().sin() * radius
                paint-order="stroke fill"
            />
        }
        .into_view(),

        MarkerShape::Square => view! {
            <Diamond x=x y=y radius=radius rotate=45 />
        }
        .into_view(),

        MarkerShape::Diamond => view! {
            <Diamond x=x y=y radius=radius />
        }
        .into_view(),

        MarkerShape::Triangle => view! {
            <polygon
                points=format!("{},{} {},{} {},{}",
                    x, y - radius,
                    x - radius, y + radius,
                    x + radius, y + radius)
                paint-order="stroke fill"/>
        }
        .into_view(),

        MarkerShape::Plus => view! {
            <PlusPath x=x y=y diameter=diameter leg=line_width />
        }
        .into_view(),

        MarkerShape::Cross => view! {
            <PlusPath x=x y=y diameter=diameter leg=line_width rotate=45 />
        }
        .into_view(),
    }
}

#[component]
fn Diamond(x: f64, y: f64, radius: f64, #[prop(into, optional)] rotate: f64) -> impl IntoView {
    view! {
        <polygon
            transform=format!("rotate({rotate} {x} {y})")
            paint-order="stroke fill"
            points=format!("{},{} {},{} {},{} {},{}",
                x, y - radius,
                x - radius, y,
                x, y + radius,
                x + radius, y) />
    }
}

// Outline of a big plus (like the Swiss flag) up against the edge (-1 to 1)
#[component]
fn PlusPath(
    x: f64,
    y: f64,
    diameter: f64,
    leg: f64,
    #[prop(into, optional)] rotate: f64,
) -> impl IntoView {
    let radius = diameter / 2.0;
    let half_leg = leg / 2.0;
    let to_inner = radius - half_leg;
    view! {
        <path
            transform=format!("rotate({rotate} {x} {y})")
            paint-order="stroke fill"
            d=format!("M {} {} h {} v {} h {} v {} h {} v {} h {} v {} h {} v {} h {} Z",
                x - half_leg, y - radius, // Top-most left
                leg, // Top-most right
                to_inner,
                to_inner, // Right-most top
                leg, // Right-most bottom
                -to_inner,
                to_inner, // Bottom-most right
                -leg, // Bottom-most left
                -to_inner,
                -to_inner, // Left-most bottom
                -leg, // Left-most top
                to_inner) />
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
