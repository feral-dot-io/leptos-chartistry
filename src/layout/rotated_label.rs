use crate::{
    bounds::Bounds, chart::Attr, debug::DebugRect, edge::Edge, layout::LayoutOption, Font, Padding,
};
use leptos::*;

#[derive(Copy, Clone, Debug)]
pub enum Anchor {
    Start,
    Middle,
    End,
}

#[derive(Clone, Debug)]
pub struct RotatedLabel {
    text: MaybeSignal<String>,
    anchor: RwSignal<Anchor>,
    font: MaybeSignal<Option<Font>>,
    padding: MaybeSignal<Option<Padding>>,
    debug: MaybeSignal<Option<bool>>,
}

impl RotatedLabel {
    fn new(anchor: Anchor, text: impl Into<MaybeSignal<String>>) -> Self {
        Self {
            text: text.into(),
            anchor: RwSignal::new(anchor),
            font: MaybeSignal::default(),
            padding: MaybeSignal::default(),
            debug: MaybeSignal::default(),
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

    pub fn set_anchor(&self) -> WriteSignal<Anchor> {
        self.anchor.write_only()
    }

    pub fn set_font(mut self, font: impl Into<MaybeSignal<Option<Font>>>) -> Self {
        self.font = font.into();
        self
    }

    pub fn set_padding(mut self, padding: impl Into<MaybeSignal<Option<Padding>>>) -> Self {
        self.padding = padding.into();
        self
    }

    pub(super) fn size(&self, attr: &Attr) -> Signal<f64> {
        let text = self.text.clone();
        let padding = attr.padding(self.padding);
        let font = attr.font(self.font);
        Signal::derive(move || {
            if text.with(|t| t.is_empty()) {
                0.0
            } else {
                with!(|font, padding| font.height() + padding.height())
            }
        })
    }
}

impl From<RotatedLabel> for LayoutOption {
    fn from(label: RotatedLabel) -> Self {
        LayoutOption::RotatedLabel(label)
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
}

#[component]
pub fn RotatedLabel<'a>(
    label: RotatedLabel,
    attr: &'a Attr,
    edge: Edge,
    bounds: Bounds,
) -> impl IntoView {
    let font = attr.font(label.font);
    let padding = attr.padding(label.padding);
    let debug = attr.debug(label.debug);
    let content = Signal::derive(move || padding.get().apply(bounds));

    let position = Signal::derive(move || {
        let content = content.get();
        let (top, right, bottom, left) = content.as_css_tuple();
        let (centre_x, centre_y) = (content.centre_x(), content.centre_y());

        let anchor = label.anchor.get();
        match edge {
            Edge::Top | Edge::Bottom => (0, anchor.map_points(left, centre_x, right), centre_y),
            Edge::Left => (270, centre_x, anchor.map_points(bottom, centre_y, top)),
            // Right rotates the opposite way to Left inverting the anchor points
            Edge::Right => (90, centre_x, anchor.map_points(top, centre_y, bottom)),
        }
    });

    view! {
        <g class="_chartistry_rotated_label">
            <DebugRect label="RotatedLabel" debug=debug bounds=move || vec![bounds, content.get()] />
            <text
                x=move || position.with(|(_, x, _)| x.to_string())
                y=move || position.with(|(_, _, y)| y.to_string())
                transform=move || position.with(|(rotate, x, y)| format!("rotate({rotate}, {x}, {y})"))
                dominant-baseline="middle"
                text-anchor=move || label.anchor.get().as_svg_attr()
                font-family=move || font.get().svg_family()
                font-size=move || font.get().svg_size()>
                { label.text }
            </text>
        </g>
    }
}
