use super::{
    compose::UseLayout,
    rotated_label::Anchor,
    snippet::{Snippet, SnippetTd},
    HorizontalLayout, VerticalLayout,
};
use crate::{
    bounds::Bounds,
    debug::DebugRect,
    edge::Edge,
    line::UseLine,
    series::UseSeries,
    state::{AttrState, State},
};
use leptos::*;
use std::{borrow::Borrow, rc::Rc};

#[derive(Clone, Debug)]
pub struct Legend {
    snippet: Snippet,
    anchor: MaybeSignal<Anchor>,
}

#[derive(Clone, Debug)]
pub struct UseLegend {
    pub(crate) snippet: Snippet,
    anchor: MaybeSignal<Anchor>,
    lines: Vec<UseLine>,
    pub(crate) width: Signal<f64>,
    pub(crate) height: Signal<f64>,
}

impl Legend {
    pub fn new(anchor: impl Into<MaybeSignal<Anchor>>, snippet: impl Borrow<Snippet>) -> Self {
        Self {
            snippet: snippet.borrow().clone(),
            anchor: anchor.into(),
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

    pub(crate) fn into_use<X, Y>(self, attr: &AttrState, series: &UseSeries<X, Y>) -> UseLegend {
        let width = mk_width(&self.snippet, attr, series);
        UseLegend {
            snippet: self.snippet,
            anchor: self.anchor,
            lines: series.lines.clone(),
            width,
            height: Snippet::fixed_height(attr),
        }
    }
}

fn mk_width<X, Y>(snippet: &Snippet, attr: &AttrState, series: &UseSeries<X, Y>) -> Signal<f64> {
    let snip_width = snippet.width(attr);
    let font = attr.font;
    let padding = attr.padding;
    let lines = series
        .lines
        .iter()
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

impl<X, Y> HorizontalLayout<X, Y> for Legend {
    fn fixed_height(&self, attr: &AttrState) -> Signal<f64> {
        Snippet::fixed_height(attr)
    }

    fn into_use(
        self: Rc<Self>,
        attr: &AttrState,
        series: &UseSeries<X, Y>,
        _: Signal<f64>,
    ) -> Rc<dyn UseLayout> {
        Rc::new((*self).clone().into_use(attr, series))
    }
}

impl<X, Y> VerticalLayout<X, Y> for Legend {
    fn into_use(
        self: Rc<Self>,
        attr: &AttrState,
        series: &UseSeries<X, Y>,
        _: Signal<f64>,
    ) -> (Signal<f64>, Rc<dyn UseLayout>) {
        let legend = Rc::new((*self).clone().into_use(attr, series));
        (legend.width, legend)
    }
}

impl UseLayout for UseLegend {
    fn render(&self, edge: Edge, bounds: Signal<Bounds>, state: &State) -> View {
        view! { <Legend legend=self.clone() edge=edge bounds=bounds state=state /> }
    }
}

#[component]
pub fn Legend<'a>(
    legend: UseLegend,
    edge: Edge,
    bounds: Signal<Bounds>,
    state: &'a State,
) -> impl IntoView {
    let UseLegend {
        snippet, anchor, ..
    } = legend;
    let AttrState {
        debug,
        padding,
        font,
        ..
    } = state.attr;

    let inner = Signal::derive(move || padding.get().apply(bounds.get()));
    let anchor_dir = if edge.is_horizontal() {
        "row"
    } else {
        "column"
    };

    let state = state.clone();
    let body = move || {
        // Sort lines by name
        let mut lines = legend.lines.clone();
        lines.sort_by_key(|line| line.name.get());

        let tds = lines.iter().map(|line| {
            let name = line.name.clone();
            view!(<SnippetTd snippet=snippet.clone() line=line.clone() attr=&state.attr>{name}</SnippetTd>)
        });

        if edge.is_horizontal() {
            view!(<tr>{ tds.collect_view() }</tr>).into_view()
        } else {
            tds.map(|td| view!(<tr>{ td }</tr>)).collect_view()
        }
    };

    view! {
        <g class="_chartistry_legend">
            <DebugRect label="Legend" debug=debug bounds=vec![bounds, inner] />
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
