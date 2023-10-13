use super::{options::Axis, InnerLayout, InnerOption, UseInner};
use crate::{chart::Attr, debug::DebugRect, projection::Projection};
use leptos::*;

#[derive(Clone, Debug)]
pub struct GuideLine {
    axis: Axis,
    width: MaybeSignal<f64>,
    debug: Option<MaybeSignal<bool>>,
}

#[derive(Clone, Debug)]
pub struct UseGuideLine {
    axis: Axis,
    width: MaybeSignal<f64>,
    debug: MaybeSignal<bool>,
}

impl GuideLine {
    fn new(axis: Axis) -> Self {
        Self {
            axis,
            width: 1.0.into(),
            debug: None,
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

    pub fn set_debug(mut self, debug: impl Into<MaybeSignal<bool>>) -> Self {
        self.debug = Some(debug.into());
        self
    }
}

impl<X, Y> InnerLayout<X, Y> for GuideLine {
    fn apply_attr(self, attr: &Attr) -> Box<dyn InnerOption<X, Y>> {
        Box::new(UseGuideLine {
            axis: self.axis,
            width: self.width,
            debug: self.debug.unwrap_or(attr.debug),
        })
    }
}

impl UseInner for UseGuideLine {
    fn render(
        self: Box<Self>,
        proj: Signal<Projection>,
        mouse: Signal<Option<(f64, f64)>>,
    ) -> View {
        view!( <GuideLine line=*self projection=proj mouse=mouse /> )
    }
}

#[component]
fn GuideLine(
    line: UseGuideLine,
    projection: Signal<Projection>,
    mouse: Signal<Option<(f64, f64)>>,
) -> impl IntoView {
    let mouse = Signal::derive(move || {
        let bounds = projection.get().bounds();
        (mouse.get()).filter(|(x, y)| bounds.contains(*x, *y))
    });
    let render = Signal::derive(move || {
        let bounds = projection.get().bounds();
        mouse.get().map(|(x, y)| {
            let (x1, y1, x2, y2) = match line.axis {
                Axis::Horizontal => (x, bounds.top_y(), x, bounds.bottom_y()),
                Axis::Vertical => (bounds.left_x(), y, bounds.right_x(), y),
            };
            view! {
                <DebugRect label=format!("GuideLine-{}", line.axis) debug=line.debug />
                <line
                    x1=x1
                    y1=y1
                    x2=x2
                    y2=y2
                    stroke="lightslategrey"
                    stroke-width=move || line.width.get() />
            }
        })
    });
    view! {
        <g class=format!("_chartistry_guide_line_{}", line.axis)>
            {render}
        </g>
    }
}
