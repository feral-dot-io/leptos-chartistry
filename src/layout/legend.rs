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
    state::{AttrState, PreState, State},
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
            snippet: *snippet.borrow(),
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

    fn fixed_height(&self, attr: &AttrState) -> Signal<f64> {
        let font = attr.font;
        let padding = attr.padding;
        Signal::derive(move || font.get().height() + padding.get().height())
    }

    pub(crate) fn into_use<X, Y>(self, attr: &AttrState, series: &UseSeries<X, Y>) -> UseLegend {
        let height = self.fixed_height(attr);
        let width = mk_width(attr, series);
        UseLegend {
            snippet: self.snippet,
            anchor: self.anchor,
            lines: series.lines.clone(),
            width,
            height,
        }
    }
}

fn mk_width<X, Y>(attr: &AttrState, series: &UseSeries<X, Y>) -> Signal<f64> {
    let AttrState { font, padding, .. } = *attr;
    let snippet_width = Snippet::taster_width(font);
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
        snippet_width.get() + font_width + max_chars + padding.get().width()
    })
}

impl<X, Y> HorizontalLayout<X, Y> for Legend {
    fn fixed_height(&self, attr: &AttrState) -> Signal<f64> {
        self.fixed_height(attr)
    }

    fn into_use(
        self: Rc<Self>,
        state: &PreState<X, Y>,
        series: &UseSeries<X, Y>,
        _: Memo<f64>,
    ) -> Box<dyn UseLayout<X, Y>> {
        Box::new((*self).clone().into_use(&state.attr, series))
    }
}

impl<X, Y> VerticalLayout<X, Y> for Legend {
    fn into_use(
        self: Rc<Self>,
        state: &PreState<X, Y>,
        series: &UseSeries<X, Y>,
        _: Memo<f64>,
    ) -> (Signal<f64>, Box<dyn UseLayout<X, Y>>) {
        let legend = Box::new((*self).clone().into_use(&state.attr, series));
        (legend.width, legend)
    }
}

impl<X, Y> UseLayout<X, Y> for UseLegend {
    fn render(&self, edge: Edge, bounds: Memo<Bounds>, state: &State<X, Y>) -> View {
        view! { <Legend legend=self.clone() edge=edge bounds=bounds state=state /> }
    }
}

#[component]
pub fn Legend<'a, X: 'static, Y: 'static>(
    legend: UseLegend,
    edge: Edge,
    bounds: Memo<Bounds>,
    state: &'a State<X, Y>,
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

    let body = move || {
        // Sort lines by name
        let mut lines = legend.lines.clone();
        lines.sort_by_key(|line| line.name.get());

        let tds = lines.into_iter().enumerate().map(|(i, line)| {
            let name = line.name.clone();
            view! {
                <SnippetTd snippet=snippet line=line font=font left_padding=edge.is_horizontal() && i != 0>
                    {name}
                </SnippetTd>
            }
        });

        if edge.is_horizontal() {
            view!(<tr>{ tds.collect_view() }</tr>).into_view()
        } else {
            tds.map(|td| view!(<tr>{ td }</tr>)).collect_view()
        }
    };

    view! {
        <g class="_chartistry_legend">
            <DebugRect label="Legend" debug=debug bounds=vec![bounds.into(), inner] />
            <foreignObject
                x=move || inner.get().left_x()
                y=move || inner.get().top_y()
                width=move || inner.get().width()
                height=move || inner.get().height()
                style="overflow: visible;">
                <div
                    style="display: flex; height: 100%; overflow: visible;"
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
