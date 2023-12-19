use super::{compose::UseLayout, HorizontalLayout, VerticalLayout};
use crate::{
    bounds::Bounds,
    debug::DebugRect,
    edge::Edge,
    state::{PreState, State},
};
use leptos::*;
use std::rc::Rc;

#[derive(Copy, Clone, Debug)]
pub enum Anchor {
    Start,
    Middle,
    End,
}

#[derive(Clone, Debug)]
pub struct RotatedLabel {
    text: MaybeSignal<String>,
    anchor: MaybeSignal<Anchor>,
}

impl RotatedLabel {
    pub fn new(
        anchor: impl Into<MaybeSignal<Anchor>>,
        text: impl Into<MaybeSignal<String>>,
    ) -> Self {
        Self {
            text: text.into(),
            anchor: anchor.into(),
        }
    }

    pub fn start(text: impl Into<MaybeSignal<String>>) -> Self {
        Self::new(Anchor::Start, text)
    }
    pub fn middle(text: impl Into<MaybeSignal<String>>) -> Self {
        Self::new(Anchor::Middle, text)
    }
    pub fn end(text: impl Into<MaybeSignal<String>>) -> Self {
        Self::new(Anchor::End, text)
    }

    fn size<X, Y>(&self, state: &PreState<X, Y>) -> Signal<f64> {
        let text = self.text.clone();
        let PreState { font, padding, .. } = *state;
        Signal::derive(move || {
            if text.with(|t| t.is_empty()) {
                0.0
            } else {
                font.get().height() + padding.get().height()
            }
        })
    }
}

impl<X, Y> HorizontalLayout<X, Y> for RotatedLabel {
    fn fixed_height(&self, state: &PreState<X, Y>) -> Signal<f64> {
        self.size(state)
    }

    fn into_use(self: Rc<Self>, _: &PreState<X, Y>, _: Memo<f64>) -> Rc<dyn UseLayout<X, Y>> {
        self
    }
}

impl<X, Y> VerticalLayout<X, Y> for RotatedLabel {
    fn into_use(
        self: Rc<Self>,
        state: &PreState<X, Y>,
        _: Memo<f64>,
    ) -> (Signal<f64>, Rc<dyn UseLayout<X, Y>>) {
        // Note: width is height because it's rotated
        (self.size(state), self)
    }
}

impl Anchor {
    fn as_svg_attr(&self) -> &'static str {
        match self {
            Anchor::Start => "start",
            Anchor::Middle => "middle",
            Anchor::End => "end",
        }
    }

    fn map_points(&self, left: f64, middle: f64, right: f64) -> f64 {
        match self {
            Anchor::Start => left,
            Anchor::Middle => middle,
            Anchor::End => right,
        }
    }

    pub fn css_justify_content(&self) -> &'static str {
        match self {
            Anchor::Start => "flex-start",
            Anchor::Middle => "center",
            Anchor::End => "flex-end",
        }
    }
}

impl<X, Y> UseLayout<X, Y> for RotatedLabel {
    fn render(&self, edge: Edge, bounds: Memo<Bounds>, state: State<X, Y>) -> View {
        view! { <RotatedLabel label=self.clone() edge=edge bounds=bounds state=state /> }
    }
}

#[component]
fn RotatedLabel<X: 'static, Y: 'static>(
    label: RotatedLabel,
    edge: Edge,
    bounds: Memo<Bounds>,
    state: State<X, Y>,
) -> impl IntoView {
    let RotatedLabel { text, anchor } = label;
    let PreState {
        font,
        padding,
        debug,
        ..
    } = state.pre;

    let content = Signal::derive(move || padding.get().apply(bounds.get()));
    let position = create_memo(move |_| {
        let c = content.get();
        let (top, right, bottom, left) = (c.top_y(), c.right_x(), c.bottom_y(), c.left_x());
        let (centre_x, centre_y) = (c.centre_x(), c.centre_y());

        let anchor = anchor.get();
        match edge {
            Edge::Top | Edge::Bottom => (0, anchor.map_points(left, centre_x, right), centre_y),
            Edge::Left => (270, centre_x, anchor.map_points(bottom, centre_y, top)),
            // Right rotates the opposite way to Left inverting the anchor points
            Edge::Right => (90, centre_x, anchor.map_points(top, centre_y, bottom)),
        }
    });

    view! {
        <g class="_chartistry_rotated_label">
            <DebugRect label="RotatedLabel" debug=debug bounds=vec![bounds.into(), content] />
            <text
                x=move || position.with(|(_, x, _)| x.to_string())
                y=move || position.with(|(_, _, y)| y.to_string())
                transform=move || position.with(|(rotate, x, y)| format!("rotate({rotate}, {x}, {y})"))
                dominant-baseline="middle"
                text-anchor=move || anchor.get().as_svg_attr()
                font-family=move || font.get().svg_family()
                font-size=move || font.get().svg_size()>
                {text}
            </text>
        </g>
    }
}
