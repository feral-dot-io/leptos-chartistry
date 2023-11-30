use crate::{line::UseLine, Font};
use leptos::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Style {
    HorizontalTaster,
    VerticalTaster,
}

#[derive(Copy, Clone, Debug)]
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

    fn taster_height(font: Signal<Font>) -> Signal<f64> {
        Signal::derive(move || font.get().height())
    }

    fn taster_width(font: Signal<Font>) -> Signal<f64> {
        Signal::derive(move || font.get().width() * 2.0)
    }

    pub(crate) fn width(font: Signal<Font>) -> Signal<f64> {
        let taster_width = Self::taster_width(font);
        Signal::derive(move || taster_width.get() + font.get().width())
    }
}

#[component]
pub(crate) fn SnippetTd(
    snippet: Snippet,
    line: UseLine,
    font: Signal<Font>,
    #[prop(optional)] left_padding: bool,
    children: Children,
) -> impl IntoView {
    let taster = move || match snippet.style.get() {
        Style::VerticalTaster => view! {
            <SnippetVerticalTaster line=&line font=font />
        },
        Style::HorizontalTaster => view! {
            <SnippetHorizontalTaster line=&line font=font />
        },
    };
    view! {
        <td
            class="_chartistry_snippet"
            style="white-space: nowrap;"
            style:padding-left=move || left_padding.then(move || format!("{}px", font.get().width() * 2.0))
        >
            {taster}
            {children()}
        </td>
    }
}

#[component]
fn SnippetHorizontalTaster<'a>(line: &'a UseLine, font: Signal<Font>) -> impl IntoView {
    let width = Snippet::taster_width(font);
    let height = Snippet::taster_height(font);
    let colour = line.colour;
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
fn SnippetVerticalTaster<'a>(line: &'a UseLine, font: Signal<Font>) -> impl IntoView {
    let width = Snippet::taster_width(font);
    let height = Snippet::taster_height(font);
    let colour = line.colour;
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
