use super::chart::Attr;
use super::Font;
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
}

impl RotatedLabel {
    fn new(anchor: Anchor, text: impl Into<MaybeSignal<String>>) -> Self {
        Self {
            text: text.into(),
            anchor: RwSignal::new(anchor),
            font: MaybeSignal::default(),
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
}

impl Anchor {
    fn svg_text_anchor(&self) -> &'static str {
        match self {
            Anchor::Start => "start",
            Anchor::Middle => "middle",
            Anchor::End => "end",
        }
    }
}

#[component]
pub fn RotatedLabel<'a>(config: RotatedLabel, attr: &'a Attr) -> impl IntoView {
    let font = attr.font(config.font);
    view! {
        <text
            dominant-baseline="middle"
            text-anchor=move || config.anchor.get().svg_text_anchor()
            font-family=move || font.get().svg_family()
            font-size=move || font.get().svg_size()>
            { config.text }
        </text>
    }
}
