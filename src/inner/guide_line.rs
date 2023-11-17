use super::{InnerLayout, InnerOption, UseInner};
use crate::{
    chart::Attr,
    colours::{Colour, LIGHT_GREY},
    projection::Projection,
    series::{Data, UseSeries},
    use_watched_node::UseWatchedNode,
};
use leptos::*;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct GuideLine {
    axis: Axis,
    width: MaybeSignal<f64>,
    colour: MaybeSignal<Colour>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Axis {
    X(AlignOver),
    Y,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
enum AlignOver {
    #[default]
    Data,
    Mouse,
}

#[derive(Clone, Debug)]
pub struct UseGuideLine<X: 'static, Y: 'static> {
    axis: Axis,
    data: Signal<Data<X, Y>>,
    width: MaybeSignal<f64>,
    colour: MaybeSignal<Colour>,
}

impl GuideLine {
    fn new(axis: Axis) -> Self {
        Self {
            axis,
            width: 1.0.into(),
            colour: Into::<Colour>::into(LIGHT_GREY).into(),
        }
    }

    pub fn x_axis() -> Self {
        Self::new(Axis::X(AlignOver::default()))
    }

    pub fn x_axis_over_data() -> Self {
        Self::new(Axis::X(AlignOver::Data))
    }

    pub fn x_axis_over_mouse() -> Self {
        Self::new(Axis::X(AlignOver::Mouse))
    }

    pub fn y_axis() -> Self {
        Self::new(Axis::Y)
    }

    pub fn set_width(mut self, width: impl Into<MaybeSignal<f64>>) -> Self {
        self.width = width.into();
        self
    }

    pub fn set_colour(mut self, colour: impl Into<MaybeSignal<Colour>>) -> Self {
        self.colour = colour.into();
        self
    }
}

impl<X, Y> InnerLayout<X, Y> for GuideLine {
    fn apply_attr(self, _: &Attr) -> Rc<dyn InnerOption<X, Y>> {
        Rc::new(self)
    }
}

impl<X, Y> InnerOption<X, Y> for GuideLine {
    fn into_use(
        self: Rc<Self>,
        series: &UseSeries<X, Y>,
        _: Signal<Projection>,
    ) -> Box<dyn UseInner> {
        Box::new(UseGuideLine {
            axis: self.axis,
            data: series.data,
            width: self.width,
            colour: self.colour,
        })
    }
}

impl<X, Y> UseInner for UseGuideLine<X, Y> {
    fn render(self: Box<Self>, proj: Signal<Projection>, watch: &UseWatchedNode) -> View {
        let mouse_hover = watch.mouse_hover_inner(proj);
        view!( <GuideLine line=*self projection=proj mouse_hover=mouse_hover mouse=watch.mouse_rel /> )
    }
}

#[component]
fn GuideLine<X: 'static, Y: 'static>(
    line: UseGuideLine<X, Y>,
    projection: Signal<Projection>,
    mouse_hover: Signal<bool>,
    mouse: Signal<(f64, f64)>,
) -> impl IntoView {
    let render = create_memo(move |_| {
        // Mouse over chart?
        if !mouse_hover.get() {
            return view!().into_view();
        }

        let (mouse_x, mouse_y) = mouse.get();
        let projection = projection.get();
        let b = projection.bounds();
        let (x1, y1, x2, y2) = match line.axis {
            Axis::X(AlignOver::Data) => {
                // Map mouse (SVG coord) to data
                let (data_x, _) = projection.svg_to_data(mouse_x, mouse_y);
                // Map data to nearest position
                let position_x = line.data.with(|data| data.nearest_x_position(data_x));
                // Map back to SVG
                let (svg_x, _) = projection.data_to_svg(position_x, 0.0);
                (svg_x, b.top_y(), svg_x, b.bottom_y())
            }
            Axis::X(AlignOver::Mouse) => (mouse_x, b.top_y(), mouse_x, b.bottom_y()),
            Axis::Y => (b.left_x(), mouse_y, b.right_x(), mouse_y),
        };

        // Don't render if any of the coordinates are NaN i.e., no data
        if x1.is_nan() || y1.is_nan() || x2.is_nan() || y2.is_nan() {
            return view!().into_view();
        }

        view! {
            <line
                x1=x1
                y1=y1
                x2=x2
                y2=y2
                stroke=line.colour.get().to_string()
                stroke-width=line.width />
        }
        .into_view()
    });
    view! {
        <g class=format!("_chartistry_guide_line_{}", line.axis)>
            {render}
        </g>
    }
}

impl std::fmt::Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Axis::X(_) => write!(f, "x"),
            Axis::Y => write!(f, "y"),
        }
    }
}
