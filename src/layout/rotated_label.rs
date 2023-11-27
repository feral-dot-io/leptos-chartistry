use super::{
    compose::{UseLayout, VerticalLayout},
    HorizontalLayout, HorizontalOption, VerticalOption,
};
use crate::{
    bounds::Bounds,
    debug::DebugRect,
    edge::Edge,
    series::UseSeries,
    state::{AttrState, State},
    Font, Padding,
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

#[derive(Clone, Debug)]
pub struct UseRotatedLabel {
    text: MaybeSignal<String>,
    anchor: MaybeSignal<Anchor>,
    font: Signal<Font>,
    padding: Signal<Padding>,
    debug: Signal<bool>,
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

    fn apply_attr(self, attr: &AttrState) -> UseRotatedLabel {
        UseRotatedLabel {
            text: self.text,
            anchor: self.anchor,
            font: attr.font,
            padding: attr.padding,
            debug: attr.debug,
        }
    }
}

impl<X: 'static, Y: 'static> HorizontalLayout<X, Y> for RotatedLabel {
    fn apply_attr(self, attr: &AttrState) -> Rc<dyn HorizontalOption<X, Y>> {
        Rc::new(self.apply_attr(attr))
    }
}

impl<X: 'static, Y: 'static> VerticalLayout<X, Y> for RotatedLabel {
    fn apply_attr(self, attr: &AttrState) -> Rc<dyn VerticalOption<X, Y>> {
        Rc::new(self.apply_attr(attr))
    }
}

impl<X, Y> HorizontalOption<X, Y> for UseRotatedLabel {
    fn fixed_height(&self) -> Signal<f64> {
        self.size()
    }

    fn into_use(self: Rc<Self>, _: &UseSeries<X, Y>, _: Signal<f64>) -> Rc<dyn UseLayout> {
        self
    }
}

impl<X, Y> VerticalOption<X, Y> for UseRotatedLabel {
    fn into_use(
        self: Rc<Self>,
        _: &UseSeries<X, Y>,
        _: Signal<f64>,
    ) -> (Signal<f64>, Rc<dyn UseLayout>) {
        // Note: width is height because it's rotated
        (self.size(), self)
    }
}

impl UseRotatedLabel {
    pub fn size(&self) -> Signal<f64> {
        let text = self.text.clone();
        let font = self.font;
        let padding = self.padding;
        Signal::derive(move || {
            if text.with(|t| t.is_empty()) {
                0.0
            } else {
                font.get().height() + padding.get().height()
            }
        })
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

impl UseLayout for UseRotatedLabel {
    fn render(&self, edge: Edge, bounds: Signal<Bounds>, state: &State) -> View {
        view! { <RotatedLabel label=self.clone() edge=edge bounds=bounds state=state /> }
    }
}

#[component]
fn RotatedLabel<'a>(
    label: UseRotatedLabel,
    edge: Edge,
    bounds: Signal<Bounds>,
    state: &'a State,
) -> impl IntoView {
    let UseRotatedLabel { text, anchor, .. } = label;
    let AttrState {
        font,
        padding,
        debug,
        ..
    } = state.attr;

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
            <DebugRect label="RotatedLabel" debug=debug bounds=vec![bounds, content] />
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
