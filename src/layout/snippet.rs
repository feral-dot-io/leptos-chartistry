use crate::{chart::Attr, Font, Line, Padding};
use leptos::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Style {
    HorizontalTaster,
    VerticalTaster,
}

#[derive(Clone, Debug)]
pub struct Snippet {
    style: MaybeSignal<Style>,
    font: Option<MaybeSignal<Font>>,
    padding: Option<MaybeSignal<Padding>>,
}

#[derive(Clone, Debug)]
pub(crate) struct UseSnippet {
    pub style: MaybeSignal<Style>,
    pub font: MaybeSignal<Font>,
    pub padding: MaybeSignal<Padding>,
}

impl Snippet {
    pub fn new(style: impl Into<MaybeSignal<Style>>) -> Self {
        Self {
            style: style.into(),
            font: None,
            padding: None,
        }
    }

    pub fn horizontal() -> Self {
        Self::new(Style::HorizontalTaster)
    }
    pub fn vertical() -> Self {
        Self::new(Style::VerticalTaster)
    }

    pub fn set_font(mut self, font: impl Into<MaybeSignal<Font>>) -> Self {
        self.font = Some(font.into());
        self
    }

    pub fn set_padding(mut self, padding: impl Into<MaybeSignal<Padding>>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    pub(super) fn to_use(self, attr: &Attr) -> UseSnippet {
        UseSnippet {
            style: self.style,
            font: self.font.unwrap_or(attr.font),
            padding: self.padding.unwrap_or(attr.padding),
        }
    }
}

impl UseSnippet {
    fn taster_width(&self) -> Signal<f64> {
        let (style, font) = (self.style, self.font);
        Signal::derive(move || match style.get() {
            Style::HorizontalTaster => font.get().width() * 2.0,
            Style::VerticalTaster => font.get().width() / 3.5,
        })
    }

    fn taster_height(&self) -> Signal<f64> {
        let font = self.font;
        Signal::derive(move || font.get().height())
    }

    pub fn width(&self) -> Signal<f64> {
        let padding = self.padding;
        let taster_width = self.taster_width();
        Signal::derive(move || taster_width.get() + padding.get().width())
    }

    pub fn height(&self) -> Signal<f64> {
        let padding = self.padding;
        let taster_height = self.taster_height();
        Signal::derive(move || taster_height.get() + padding.get().height())
    }
}

#[component]
pub(crate) fn SnippetTd(snippet: UseSnippet, line: Line, children: Children) -> impl IntoView {
    let padding = snippet.padding;
    view! {
        <td
            class="_chartistry_snippet"
            style:padding=move || padding.get().to_style_px()>
            {move || match snippet.style.get() {
                Style::VerticalTaster => view!(<SnippetVerticalTaster snippet=&snippet line=&line />),
                Style::HorizontalTaster => view!(<SnippetHorizontalTaster snippet=&snippet line=&line />),
            }}
            {children()}
        </td>
    }
}

#[component]
fn SnippetHorizontalTaster<'a>(snippet: &'a UseSnippet, line: &'a Line) -> impl IntoView {
    let font = snippet.font;
    let (taster_width, taster_height) = (snippet.taster_width(), snippet.taster_height());
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
                stroke="red"
                stroke-width=line.width />
        </svg>
    }
}

#[component]
fn SnippetVerticalTaster<'a>(snippet: &'a UseSnippet, line: &'a Line) -> impl IntoView {
    let font = snippet.font;
    let (taster_width, taster_height) = (snippet.taster_width(), snippet.taster_height());
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
                    y2=snippet.taster_height()
                    stroke="red"
                    stroke-width=line.width />
            </svg>
        </div>
    }
}
