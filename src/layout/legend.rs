use super::{
    compose::UseLayout,
    rotated_label::Anchor,
    snippet::{Snippet, SnippetTd, UseSnippet},
    HorizontalLayout, HorizontalOption, VerticalLayout, VerticalOption,
};
use crate::{
    bounds::Bounds, chart::Attr, debug::DebugRect, edge::Edge, line::UseLine, series::UseSeries,
    state::State, Padding,
};
use leptos::*;
use std::{borrow::Borrow, rc::Rc};

#[derive(Clone, Debug)]
pub struct Legend {
    snippet: Snippet,
    anchor: MaybeSignal<Anchor>,
    padding: Option<MaybeSignal<Padding>>,
    debug: Option<MaybeSignal<bool>>,
}

#[derive(Clone, Debug)]
pub struct LegendAttr {
    snippet: UseSnippet,
    anchor: MaybeSignal<Anchor>,
    padding: MaybeSignal<Padding>,
    debug: MaybeSignal<bool>,
}

#[derive(Clone, Debug)]
pub struct UseLegend {
    attr: LegendAttr,
    lines: Vec<UseLine>,
}

impl Legend {
    pub fn new(anchor: impl Into<MaybeSignal<Anchor>>, snippet: impl Borrow<Snippet>) -> Self {
        Self {
            snippet: snippet.borrow().clone(),
            anchor: anchor.into(),
            padding: None,
            debug: None,
        }
    }

    pub fn start(snippet: impl Borrow<Snippet>) -> Self {
        Self::new(Anchor::Start, snippet)
    }
    pub fn middle(snippet: impl Borrow<Snippet>) -> Self {
        Self::new(Anchor::Middle, snippet)
    }
    pub fn end(snippet: impl Borrow<Snippet>) -> Self {
        Self::new(Anchor::End, snippet)
    }

    pub fn set_padding(mut self, padding: impl Into<MaybeSignal<Padding>>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn set_debug(mut self, debug: impl Into<MaybeSignal<bool>>) -> Self {
        self.debug = Some(debug.into());
        self
    }

    pub(crate) fn apply_attr(self, attr: &Attr) -> LegendAttr {
        LegendAttr {
            snippet: self.snippet.into_use(attr),
            anchor: self.anchor,
            padding: self.padding.unwrap_or(attr.padding),
            debug: self.debug.unwrap_or(attr.debug),
        }
    }
}

impl<X: 'static, Y: 'static> HorizontalLayout<X, Y> for Legend {
    fn apply_attr(self, attr: &Attr) -> Rc<dyn HorizontalOption<X, Y>> {
        Rc::new(self.apply_attr(attr))
    }
}

impl<X: 'static, Y: 'static> VerticalLayout<X, Y> for Legend {
    fn apply_attr(self, attr: &Attr) -> Rc<dyn VerticalOption<X, Y>> {
        Rc::new(self.apply_attr(attr))
    }
}

impl LegendAttr {
    fn height(&self) -> Signal<f64> {
        let (snip_height, font, padding) = (self.snippet.height(), self.snippet.font, self.padding);
        Signal::derive(move || {
            let text_height = font.get().height() + padding.get().height();
            text_height.max(snip_height.get())
        })
    }

    pub fn into_use<X, Y>(self, series: &UseSeries<X, Y>) -> UseLegend {
        UseLegend {
            attr: self,
            lines: series.lines.clone(),
        }
    }
}

impl<X, Y> HorizontalOption<X, Y> for LegendAttr {
    fn height(&self) -> Signal<f64> {
        self.height()
    }

    fn into_use(self: Rc<Self>, series: &UseSeries<X, Y>, _: Signal<f64>) -> Rc<dyn UseLayout> {
        Rc::new((*self).clone().into_use(series))
    }
}

impl<X, Y> VerticalOption<X, Y> for LegendAttr {
    fn into_use(self: Rc<Self>, series: &UseSeries<X, Y>, _: Signal<f64>) -> Rc<dyn UseLayout> {
        Rc::new((*self).clone().into_use(series))
    }
}

impl UseLegend {
    pub fn height(&self) -> Signal<f64> {
        self.attr.height()
    }

    pub fn width(&self) -> Signal<f64> {
        let snip_width = self.attr.snippet.width();
        let (font, padding) = (self.attr.snippet.font, self.attr.padding);
        let lines = (self.lines.iter())
            .map(|line| line.name.clone())
            .collect::<Vec<_>>();
        Signal::derive(move || {
            let font_width = font.get().width();
            let max_chars = (lines.iter())
                .map(|line| line.get().len() as f64 * font_width)
                .reduce(f64::max)
                .unwrap_or_default();
            snip_width.get() + font_width + max_chars + padding.get().width()
        })
    }
}

impl UseLayout for UseLegend {
    fn width(&self) -> Signal<f64> {
        self.width()
    }

    fn render(&self, edge: Edge, bounds: Signal<Bounds>, _: &State) -> View {
        view! { <Legend legend=self.clone() edge=edge bounds=bounds /> }
    }
}

#[component]
pub fn Legend(legend: UseLegend, edge: Edge, bounds: Signal<Bounds>) -> impl IntoView {
    let LegendAttr {
        snippet,
        anchor,
        padding,
        debug,
    } = legend.attr;
    let font = snippet.font;

    let inner = Signal::derive(move || padding.get().apply(bounds.get()));
    let anchor_dir = if edge.is_horizontal() {
        "row"
    } else {
        "column"
    };

    let body = move || {
        // Sort lines by name
        let mut lines = legend.lines.clone();
        lines.sort_by_key(|line| line.name.get());

        let tds = lines.iter().map(|line| {
            let name = line.name.clone();
            view!(<SnippetTd snippet=snippet.clone() line=line.clone()>{name}</SnippetTd>)
        });

        if edge.is_horizontal() {
            view!(<tr>{ tds.collect_view() }</tr>).into_view()
        } else {
            tds.map(|td| view!(<tr>{ td }</tr>)).collect_view()
        }
    };

    view! {
        <g class="_chartistry_legend">
            <DebugRect label="Legend" debug=debug bounds=move || vec![bounds.get(), inner.get()] />
            <foreignObject
                x=move || bounds.get().left_x()
                y=move || bounds.get().top_y()
                width=move || inner.get().width()
                height=move || inner.get().height()
                style="overflow: auto;">
                <div
                    style="display: flex; height: 100%; overflow: auto;"
                    style:flex-direction=anchor_dir
                    style:justify-content=move || anchor.get().css_justify_content()>
                    <table
                        style="border-collapse: collapse; border-spacing: 0; margin: 0; padding: 0;"
                        style:font-size=move || format!("{}px", font.get().height())>
                        <tbody>
                            {body}
                        </tbody>
                    </table>
                </div>
            </foreignObject>
        </g>
    }
}
