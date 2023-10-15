use super::{options::Axis, InnerLayout, InnerOption, UseInner};
use crate::{chart::Attr, projection::Projection, use_watched_node::UseWatchedNode};
use leptos::*;

#[derive(Clone, Debug)]
pub struct GuideLine {
    axis: Axis,
    width: MaybeSignal<f64>,
}

#[derive(Clone, Debug)]
pub struct UseGuideLine {
    axis: Axis,
    width: MaybeSignal<f64>,
}

impl GuideLine {
    fn new(axis: Axis) -> Self {
        Self {
            axis,
            width: 1.0.into(),
        }
    }

    pub fn horizontal() -> Self {
        Self::new(Axis::Horizontal)
    }

    pub fn vertical() -> Self {
        Self::new(Axis::Vertical)
    }

    pub fn set_width(mut self, width: impl Into<MaybeSignal<f64>>) -> Self {
        self.width = width.into();
        self
    }
}

impl<X, Y> InnerLayout<X, Y> for GuideLine {
    fn apply_attr(self, _: &Attr) -> Box<dyn InnerOption<X, Y>> {
        Box::new(UseGuideLine {
            axis: self.axis,
            width: self.width,
        })
    }
}

impl UseInner for UseGuideLine {
    fn render(self: Box<Self>, proj: Signal<Projection>, watch: &UseWatchedNode) -> View {
        view!( <GuideLine line=*self projection=proj mouse_over=watch.over_inner mouse=watch.mouse_rel /> )
    }
}

#[component]
fn GuideLine(
    line: UseGuideLine,
    projection: Signal<Projection>,
    mouse_over: Signal<bool>,
    mouse: Signal<(f64, f64)>,
) -> impl IntoView {
    let render = create_memo(move |_| {
        if !mouse_over.get() {
            return view!().into_view();
        }

        let (x1, y1, x2, y2) = match line.axis {
            Axis::Horizontal => (
                Signal::derive(move || with!(|mouse| mouse.0)),
                Signal::derive(move || with!(|projection| projection.bounds().top_y())),
                Signal::derive(move || with!(|mouse| mouse.0)),
                Signal::derive(move || with!(|projection| projection.bounds().bottom_y())),
            ),
            Axis::Vertical => (
                Signal::derive(move || with!(|projection| projection.bounds().left_x())),
                Signal::derive(move || with!(|mouse| mouse.1)),
                Signal::derive(move || with!(|projection| projection.bounds().right_x())),
                Signal::derive(move || with!(|mouse| mouse.1)),
            ),
        };

        view! {
            <line
                x1=x1
                y1=y1
                x2=x2
                y2=y2
                stroke="lightslategrey"
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
