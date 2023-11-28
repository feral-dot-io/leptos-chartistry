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

    fn taster_height(&self, attr: &AttrState) -> Signal<f64> {
        let font = attr.font;
        Signal::derive(move || font.get().height())
    }

    pub(crate) fn taster_width(&self, attr: &AttrState) -> Signal<f64> {
        let font = attr.font;
        Signal::derive(move || font.get().width() * 2.0)
    }
}

#[component]
pub(crate) fn SnippetTd<'a>(
    snippet: Snippet,
    line: UseLine,
    attr: &'a AttrState,
    #[prop(optional)] left_padding: bool,
    children: Children,
) -> impl IntoView {
    let font = attr.font;
    let attr = attr.clone();
    let snippet = move || match snippet.style.get() {
        Style::VerticalTaster => view! {
            <SnippetVerticalTaster snippet=&snippet line=&line attr=&attr />
        },
        Style::HorizontalTaster => view! {
            <SnippetHorizontalTaster snippet=&snippet line=&line attr=&attr />
        },
    };
    view! {
        <td
            class="_chartistry_snippet"
            style="white-space: nowrap;"
            style:padding-left=move || left_padding.then(move || format!("{}px", font.get().width() * 2.0))
        >
            {snippet}
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
    let width = snippet.taster_width(attr);
    let height = snippet.taster_height(attr);
    view! {
        <svg
            class="_chartistry_snippet_horizontal"
            width=width
            height=height
            viewBox=move || format!("0 0 {} {}", width.get(), height.get())
            style="overflow: visible;"
            style:padding-right=move || format!("{}px", font.get().width())
        >
            <line
                x1=0
                x2=width
                y1=move || font.get().width()
                y2=move || font.get().width()
                stroke=move || colour.get().to_string()
                stroke-width=line.width
            />
        </svg>
    }
}

#[component]
fn SnippetVerticalTaster<'a>(
    snippet: &'a Snippet,
    line: &'a UseLine,
    attr: &'a AttrState,
) -> impl IntoView {
    let colour = line.colour;
    let width = snippet.taster_width(attr);
    let height = snippet.taster_height(attr);
    let x = Signal::derive(move || width.get() / 2.0);
    view! {
        <div
            class="_chartistry_snippet_vertical"
            style="float: left;"
            style:width=move || format!("{}px", width.get())
            style:height=move || format!("{}px", height.get())
        >
            <svg
                width=width
                // Extend the height slightly to cover below the text baseline
                height=move || height.get() * 1.2
                preserve_aspect_ratio="none"
                viewBox=move || format!("0 0 {} {}", width.get(), height.get())
                style="position: absolute;"
            >
                <line
                    x1=x
                    x2=x
                    y1=0
                    y2=height
                    stroke=move || colour.get().to_string()
                    stroke-width=line.width
                />
            </svg>
        </div>
    }
}
