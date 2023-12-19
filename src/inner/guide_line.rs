use super::{InnerLayout, UseInner};
use crate::{
    colours::{Colour, LIGHT_GREY},
    debug::DebugRect,
    layout::Layout,
    state::State,
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
    fn into_use(self: Rc<Self>, _: &State<X, Y>) -> Rc<dyn UseInner<X, Y>> {
        self
    }
}

impl<X, Y> UseInner<X, Y> for GuideLine {
    fn render(self: Rc<Self>, state: State<X, Y>) -> View {
        view!( <GuideLine line=(*self).clone() state=state /> )
    }
}

#[component]
fn GuideLine<X: 'static, Y: 'static>(line: GuideLine, state: State<X, Y>) -> impl IntoView {
    let debug = state.pre.debug;
    let State {
        layout: Layout { inner, .. },
        hover_inner,
        mouse_chart,
        nearest_svg_x,
        ..
    } = state;

    let pos = Signal::derive(move || {
        let (mouse_x, mouse_y) = mouse_chart.get();
        let inner = inner.get();
        match line.axis {
            Axis::X(AlignOver::Data) => {
                let svg_x = nearest_svg_x.get();
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
            <Show when=move || hover_inner.get() && have_data.get() >
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
