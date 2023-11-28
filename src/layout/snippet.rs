use crate::{line::UseLine, state::AttrState};
use leptos::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Style {
    HorizontalTaster,
    VerticalTaster,
}

#[derive(Clone, Debug)]
pub struct Snippet {
    style: MaybeSignal<Style>,
}

impl Snippet {
    pub fn new(style: impl Into<MaybeSignal<Style>>) -> Self {
        Self {
            style: style.into(),
        }
    }

    pub fn horizontal() -> Self {
        Self::new(Style::HorizontalTaster)
    }
    pub fn vertical() -> Self {
        Self::new(Style::VerticalTaster)
    }

    pub(crate) fn fixed_height(attr: &AttrState) -> Signal<f64> {
        let font = attr.font;
        let padding = attr.padding;
        Signal::derive(move || font.get().height() + padding.get().height())
    }

    fn taster_width(&self, attr: &AttrState) -> Signal<f64> {
        let style = self.style;
        let font = attr.font;
        Signal::derive(move || match style.get() {
            Style::HorizontalTaster => font.get().width() * 2.0,
            Style::VerticalTaster => font.get().width() / 3.5,
        })
    }

    fn taster_height(&self, attr: &AttrState) -> Signal<f64> {
        let font = attr.font;
        Signal::derive(move || font.get().height())
    }

    pub(crate) fn width(&self, attr: &AttrState) -> Signal<f64> {
        let padding = attr.padding;
        let taster_width = self.taster_width(attr);
        Signal::derive(move || taster_width.get() + padding.get().width())
    }
}

#[component]
pub(crate) fn SnippetTd<'a>(
    snippet: Snippet,
    line: UseLine,
    attr: &'a AttrState,
    children: Children,
) -> impl IntoView {
    let attr = attr.clone();
    let padding = attr.padding;
    view! {
        <td
            class="_chartistry_snippet"
            style="white-space: nowrap;"
            style:padding=move || padding.get().to_style_px()>
            {move || match snippet.style.get() {
                Style::VerticalTaster => view!(<SnippetVerticalTaster snippet=&snippet line=&line attr=&attr />),
                Style::HorizontalTaster => view!(<SnippetHorizontalTaster snippet=&snippet line=&line attr=&attr />),
            }}
            {children()}
        </td>
    }
}

#[component]
fn SnippetHorizontalTaster<'a>(
    snippet: &'a Snippet,
    line: &'a UseLine,
    attr: &'a AttrState,
) -> impl IntoView {
    let font = attr.font;
    let colour = line.colour;
    let taster_width = snippet.taster_width(attr);
    let taster_height = snippet.taster_height(attr);
    view! {
        <svg
            class="_chartistry_snippet_horizontal"
            width=taster_width
            height=taster_height
            viewBox=move || format!("0 0 {} {}", taster_width.get(), taster_height.get())
            style="overflow: visible;"
            style:padding-right=move || format!("{}px", font.get().width())>
            <line
                x1=0
                x2=taster_width
                y1=move || font.get().width()
                y2=move || font.get().width()
                stroke=move || colour.get().to_string()
                stroke-width=line.width />
        </svg>
    }
}

#[component]
fn SnippetVerticalTaster<'a>(
    snippet: &'a Snippet,
    line: &'a UseLine,
    attr: &'a AttrState,
) -> impl IntoView {
    let font = attr.font;
    let colour = line.colour;
    let taster_width = snippet.taster_width(attr);
    let taster_height = snippet.taster_height(attr);
    let x = Signal::derive(move || taster_width.get() / 2.0);
    view! {
        <div
            class="_chartistry_snippet_vertical"
            style="float: left;"
            style:width=move || format!("{}px", taster_width.get())
            style:height=move || format!("{}px", taster_height.get())
            style:margin-right=move || format!("{}px", font.get().width())>
            <svg
                width=taster_width
                // Extend the height slightly to cover below the text baseline
                height=move || taster_height.get() * 1.2
                preserve_aspect_ratio="none"
                viewBox=move || format!("0 0 {} {}", taster_width.get(), taster_height.get())
                style="position: absolute;">
                <line
                    x1=x
                    x2=x
                    y1=0
                    y2=taster_height
                    stroke=move || colour.get().to_string()
                    stroke-width=line.width />
            </svg>
        </div>
    }
}
