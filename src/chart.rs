use crate::{Font, RotatedLabel};
use leptos::*;

#[derive(Clone, Debug)]
pub struct Chart {
    attr: Attr,
    layout: Vec<Layout>,
}

#[derive(Clone, Debug)]
pub struct Attr {
    font: MaybeSignal<Font>,
}

#[derive(Clone, Debug)]
pub enum Layout {
    TextLabel(RotatedLabel),
}

impl Chart {
    pub fn new(font: impl Into<MaybeSignal<Font>>) -> Self {
        Self {
            attr: Attr::new(font),
            layout: vec![],
        }
    }

    pub fn add_layout(mut self, opt: impl Into<Layout>) -> Self {
        self.layout.push(opt.into());
        self
    }
}

impl Attr {
    pub fn new(font: impl Into<MaybeSignal<Font>>) -> Self {
        Self { font: font.into() }
    }

    pub fn font(&self, priority: MaybeSignal<Option<Font>>) -> MaybeSignal<Font> {
        let fallback = self.font;
        MaybeSignal::derive(move || priority.with(|f| f.unwrap_or_else(|| fallback.get())))
    }
}

#[component]
pub fn Chart(chart: Chart) -> impl IntoView {
    let Chart { attr, layout } = chart;

    let layout = (layout.into_iter())
        .map(|layout| view!(<Layout layout=layout attr=&attr />))
        .collect_view();

    view! {
        <div style="margin: 0 auto;">
            <svg style="overflow: visible;">
                {layout}
            </svg>
        </div>
    }
}

impl From<RotatedLabel> for Layout {
    fn from(label: RotatedLabel) -> Self {
        Layout::TextLabel(label)
    }
}

#[component]
pub fn Layout<'a>(layout: Layout, attr: &'a Attr) -> impl IntoView {
    match layout {
        Layout::TextLabel(config) => view! { <RotatedLabel config=config attr=attr /> }.into_view(),
    }
}
