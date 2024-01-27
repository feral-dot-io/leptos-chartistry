use super::UseInner;
use crate::{colours::Colour, debug::DebugRect, state::State};
use leptos::*;
use std::rc::Rc;

macro_rules! impl_guide_line {
    ($name:ident) => {
        #[derive(Clone, Debug)]
        pub struct $name {
            pub align: RwSignal<AlignOver>,
            pub width: RwSignal<f64>,
            pub colour: RwSignal<Option<Colour>>,
        }

        impl $name {
            pub fn new(align: impl Into<RwSignal<AlignOver>>) -> Self {
                Self {
                    align: align.into(),
                    ..Default::default()
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    align: AlignOver::default().into(),
                    width: 1.0.into(),
                    colour: RwSignal::default(),
                }
            }
        }
    };
}

impl_guide_line!(XGuideLine);
impl_guide_line!(YGuideLine);

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum AlignOver {
    #[default]
    Mouse,
    Data,
}

#[derive(Clone)]
struct UseXGuideLine(XGuideLine);
#[derive(Clone)]
struct UseYGuideLine(YGuideLine);

impl XGuideLine {
    pub(crate) fn use_horizontal<X, Y>(self) -> Rc<dyn UseInner<X, Y>> {
        Rc::new(UseXGuideLine(self))
    }
}

impl YGuideLine {
    pub(crate) fn use_vertical<X, Y>(self) -> Rc<dyn UseInner<X, Y>> {
        Rc::new(UseYGuideLine(self))
    }
}

impl<X, Y> UseInner<X, Y> for UseXGuideLine {
    fn render(self: Rc<Self>, state: State<X, Y>) -> View {
        view!( <XGuideLine line=self.0.clone() state=state /> )
    }
}

impl<X, Y> UseInner<X, Y> for UseYGuideLine {
    fn render(self: Rc<Self>, state: State<X, Y>) -> View {
        view!( <YGuideLine line=self.0.clone() state=state /> )
    }
}

#[component]
fn XGuideLine<X: 'static, Y: 'static>(line: XGuideLine, state: State<X, Y>) -> impl IntoView {
    let inner = state.layout.inner;
    let mouse_chart = state.mouse_chart;
    let nearest_svg_x = state.nearest_svg_x;
    let pos = Signal::derive(move || {
        let (mouse_x, _) = mouse_chart.get();
        let inner = inner.get();
        match line.align.get() {
            AlignOver::Data => {
                let svg_x = nearest_svg_x.get();
                (svg_x, inner.top_y(), svg_x, inner.bottom_y())
            }
            AlignOver::Mouse => (mouse_x, inner.top_y(), mouse_x, inner.bottom_y()),
        }
    });
    view! {
        <GuideLine id="x" width=line.width colour=line.colour state=state pos=pos />
    }
}

#[component]
fn YGuideLine<X: 'static, Y: 'static>(line: YGuideLine, state: State<X, Y>) -> impl IntoView {
    let inner = state.layout.inner;
    let mouse_chart = state.mouse_chart;
    // TODO align over
    let pos = Signal::derive(move || {
        let (_, mouse_y) = mouse_chart.get();
        let inner = inner.get();
        (inner.left_x(), mouse_y, inner.right_x(), mouse_y)
    });
    view! {
        <GuideLine id="y" width=line.width colour=line.colour state=state pos=pos />
    }
}

#[component]
fn GuideLine<X: 'static, Y: 'static>(
    id: &'static str,
    width: RwSignal<f64>,
    colour: RwSignal<Option<Colour>>,
    state: State<X, Y>,
    pos: Signal<(f64, f64, f64, f64)>,
) -> impl IntoView {
    let debug = state.pre.debug;
    let hover_inner = state.hover_inner;

    let x1 = create_memo(move |_| pos.get().0);
    let y1 = create_memo(move |_| pos.get().1);
    let x2 = create_memo(move |_| pos.get().2);
    let y2 = create_memo(move |_| pos.get().3);

    // Don't render if any of the coordinates are NaN i.e., no data
    let have_data = Signal::derive(move || {
        !(x1.get().is_nan() || y1.get().is_nan() || x2.get().is_nan() || y2.get().is_nan())
    });

    let colour = Colour::signal_option(colour, super::DEFAULT_COLOUR_GUIDE_LINE);
    view! {
        <g class=format!("_chartistry_{}_guide_line", id)>
            <Show when=move || hover_inner.get() && have_data.get() >
                <DebugRect label=format!("{}_guide_line", id) debug=debug />
                <line
                    x1=x1
                    y1=y1
                    x2=x2
                    y2=y2
                    stroke=move || colour.get().to_string()
                    stroke-width=width
                />
            </Show>
        </g>
    }
}
