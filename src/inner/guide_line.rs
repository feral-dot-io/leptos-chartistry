use super::{InnerLayout, UseInner};
use crate::{
    colours::{Colour, LIGHT_GREY},
    debug::DebugRect,
    layout::Layout,
    series::{Data, UseSeries},
    state::{AttrState, State},
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
    fn into_use(
        self: Rc<Self>,
        series: &UseSeries<X, Y>,
        _: &State<X, Y>,
    ) -> Box<dyn UseInner<X, Y>> {
        Box::new(UseGuideLine {
            axis: self.axis,
            data: series.data,
            width: self.width,
            colour: self.colour,
        })
    }
}

impl<X, Y> UseInner<X, Y> for UseGuideLine<X, Y> {
    fn render(self: Box<Self>, state: &State<X, Y>) -> View {
        view! { <GuideLine line=*self state=state /> }
    }
}

#[component]
fn GuideLine<'a, X: 'static, Y: 'static>(
    line: UseGuideLine<X, Y>,
    state: &'a State<X, Y>,
) -> impl IntoView {
    let State {
        attr: AttrState { debug, .. },
        layout: Layout { inner, .. },
        projection,
        mouse_hover_inner,
        mouse_chart,
        ..
    } = *state;

    let pos = Signal::derive(move || {
        let (mouse_x, mouse_y) = mouse_chart.get();
        let proj = projection.get();
        let inner = inner.get();
        match line.axis {
            Axis::X(AlignOver::Data) => {
                // Map mouse (SVG coord) to data
                let (data_x, _) = proj.svg_to_data(mouse_x, mouse_y);
                // Map data to nearest position
                let position_x = line.data.with(|data| data.nearest_x_position(data_x));
                // Map back to SVG
                let (svg_x, _) = proj.data_to_svg(position_x, 0.0);
                (svg_x, inner.top_y(), svg_x, inner.bottom_y())
            }
            Axis::X(AlignOver::Mouse) => (mouse_x, inner.top_y(), mouse_x, inner.bottom_y()),
            Axis::Y => (inner.left_x(), mouse_y, inner.right_x(), mouse_y),
        }
    });
    let x1 = create_memo(move |_| pos.get().0);
    let y1 = create_memo(move |_| pos.get().1);
    let x2 = create_memo(move |_| pos.get().2);
    let y2 = create_memo(move |_| pos.get().3);

    // Don't render if any of the coordinates are NaN i.e., no data
    let have_data = Signal::derive(move || {
        !(x1.get().is_nan() || y1.get().is_nan() || x2.get().is_nan() || y2.get().is_nan())
    });

    view! {
        <g class=format!("_chartistry_guide_line_{}", line.axis)>
            <Show when=move || mouse_hover_inner.get() && have_data.get() >
                <DebugRect label=format!("guide_line_{}", line.axis) debug=debug />
                <line
                    x1=x1
                    y1=y1
                    x2=x2
                    y2=y2
                    stroke=move || line.colour.get().to_string()
                    stroke-width=line.width
                />
            </Show>
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
