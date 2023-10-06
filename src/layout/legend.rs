use super::{
    rotated_label::Anchor,
    snippet::{Snippet, SnippetTd, UseSnippet},
    LayoutOption,
};
use crate::{
    bounds::Bounds, chart::Attr, debug::DebugRect, edge::Edge, series::UseSeries, Line, Padding,
};
use leptos::*;

#[derive(Clone, Debug)]
pub struct Legend {
    snippet: Snippet,
    anchor: MaybeSignal<Anchor>,
    padding: MaybeSignal<Option<Padding>>,
    debug: MaybeSignal<Option<bool>>,
}

#[derive(Clone, Debug)]
pub struct UseLegend {
    snippet: UseSnippet,
    anchor: MaybeSignal<Anchor>,
    padding: MaybeSignal<Padding>,
    debug: MaybeSignal<bool>,
    lines: Vec<Line>,
}

impl Legend {
    pub fn new(anchor: impl Into<MaybeSignal<Anchor>>, snippet: Snippet) -> Self {
        Self {
            snippet: snippet.into(),
            anchor: anchor.into(),
            padding: MaybeSignal::default(),
            debug: MaybeSignal::default(),
        }
    }

    pub fn start(snippet: Snippet) -> Self {
        Self::new(Anchor::Start, snippet)
    }
    pub fn middle(snippet: Snippet) -> Self {
        Self::new(Anchor::Middle, snippet)
    }
    pub fn end(snippet: Snippet) -> Self {
        Self::new(Anchor::End, snippet)
    }

    pub fn set_padding(mut self, padding: impl Into<MaybeSignal<Option<Padding>>>) -> Self {
        self.padding = padding.into();
        self
    }

    pub(super) fn to_use<X, Y>(self, attr: &Attr, series: &UseSeries<X, Y>) -> UseLegend {
        UseLegend {
            snippet: self.snippet.to_use(attr),
            anchor: self.anchor,
            padding: attr.padding(self.padding),
            debug: attr.debug(self.debug),
            lines: series.lines.clone(),
        }
    }
}

impl UseLegend {
    pub fn height(&self) -> Signal<f64> {
        let (snip_height, padding) = (self.snippet.height(), self.padding);
        Signal::derive(move || padding.get().height() + snip_height.get())
    }

    pub fn width(&self) -> Signal<f64> {
        let snip_width = self.snippet.width();
        let (font, padding) = (self.snippet.font, self.padding);
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

impl From<Legend> for LayoutOption {
    fn from(config: Legend) -> Self {
        LayoutOption::Legend(config)
    }
}

#[component]
pub fn Legend(legend: UseLegend, edge: Edge, bounds: Bounds) -> impl IntoView {
    let UseLegend {
        snippet,
        anchor,
        padding,
        debug,
        lines,
    } = legend;
    let font = snippet.font;

    let inner = Signal::derive(move || padding.get().apply(bounds));
    let anchor_dir = if edge.is_horizontal() {
        "row"
    } else {
        "column"
    };

    let body = move || {
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
            <DebugRect label="Legend" debug=debug bounds=move || vec![bounds, inner.get()] />
            <foreignObject
                x=move || inner.get().left_x()
                y=move || inner.get().top_y()
                width=move || inner.get().width()
                height=move || inner.get().height()
                style="overflow: auto;">
                <div
                    style="display: flex; height: 100%;"
                    style:flex-direction=anchor_dir
                    style:justify-content=move || anchor.get().css_justify_content()>
                    <table
                        style="border-collapse: collapse; border-spacing: 0; margin: 0; padding: 0; overflow: auto;"
                        style:font-size=move || format!("{}px;", font.get().height())>
                        <tbody>
                            {body}
                        </tbody>
                    </table>
                </div>
            </foreignObject>
        </g>
    }
}
