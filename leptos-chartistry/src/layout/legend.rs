use super::{rotated_label::Anchor, UseLayout, UseVerticalLayout};
use crate::{
    bounds::Bounds,
    debug::DebugRect,
    edge::Edge,
    series::{Snippet, UseY},
    state::{PreState, State},
    Padding,
};
use leptos::prelude::*;

/// Builds a legend for the chart [series](crate::Series). Orientated along the axis of its placed edge. Drawn in HTML.
#[derive(Clone, Debug)]
pub struct Legend {
    /// Anchor of the legend.
    pub anchor: RwSignal<Anchor>,
}

impl Legend {
    pub(crate) fn new(anchor: Anchor) -> Self {
        Self {
            anchor: create_rw_signal(anchor),
        }
    }

    /// Creates a new legend placed at the start of the line layout.
    pub fn start() -> Legend {
        Self::new(Anchor::Start)
    }
    /// Creates a new legend placed in the middle of the line layout.
    pub fn middle() -> Legend {
        Self::new(Anchor::Middle)
    }
    /// Creates a new legend placed at the end of the line layout.
    pub fn end() -> Legend {
        Self::new(Anchor::End)
    }

    pub(crate) fn width<X, Y>(state: &PreState<X, Y>) -> Signal<f64> {
        let font_height = state.font_height;
        let font_width = state.font_width;
        let padding = state.padding;
        let series = state.data.series;
        let snippet_bounds = UseY::snippet_width(font_height, font_width);
        Signal::derive(move || {
            let font_width = font_width.get();
            let max_chars = series
                .get()
                .into_iter()
                .map(|line| line.name.get().len() as f64 * font_width)
                .reduce(f64::max)
                .unwrap_or_default();
            snippet_bounds.get() + max_chars + padding.get().width()
        })
    }

    pub(crate) fn fixed_height<X, Y>(&self, state: &PreState<X, Y>) -> Signal<f64> {
        let font_height = state.font_height;
        let padding = state.padding;
        Signal::derive(move || font_height.get() + padding.get().height())
    }

    pub(super) fn to_horizontal_use(&self) -> UseLayout {
        UseLayout::Legend(self.clone())
    }

    pub(super) fn to_vertical_use<X, Y>(&self, state: &PreState<X, Y>) -> UseVerticalLayout {
        UseVerticalLayout {
            width: Self::width(state),
            layout: UseLayout::Legend(self.clone()),
        }
    }
}

#[component]
pub(crate) fn Legend<X: Clone + 'static, Y: Clone + 'static>(
    legend: Legend,
    #[prop(into)] edge: MaybeSignal<Edge>,
    bounds: Memo<Bounds>,
    state: State<X, Y>,
) -> impl IntoView {
    let anchor = legend.anchor;
    let debug = state.pre.debug;
    let font_height = state.pre.font_height;
    let padding = state.pre.padding;
    let series = state.pre.data.series;

    // Don't apply padding on the edges of our axis i.e., maximise the space we extend over
    let padding = create_memo(move |_| {
        let padding = padding.get();
        if edge.get().is_horizontal() {
            Padding::sides(padding.top, 0.0, padding.bottom, 0.0)
        } else {
            Padding::sides(0.0, padding.right, 0.0, padding.left)
        }
    });
    let inner = Signal::derive(move || padding.get().apply(bounds.get()));

    let html = move || {
        let edge = edge.get();
        let body = if edge.is_horizontal() {
            view!(<HorizontalBody series=series state=state.clone() />)
        } else {
            view!(<VerticalBody series=series state=state.clone() />)
        };
        view! {
            <div
                style="display: flex; height: 100%; overflow: auto;"
                style:flex-direction={if edge.is_horizontal() { "row" } else { "column" }}
                style:justify-content=move || anchor.get().css_justify_content()>
                <table
                    style="border-collapse: collapse; border-spacing: 0; margin: 0;"
                    style:font-size=move || format!("{}px", font_height.get())>
                    <tbody>
                        {body}
                    </tbody>
                </table>
            </div>
        }
    };

    view! {
        <g class="_chartistry_legend">
            <DebugRect label="Legend" debug=debug bounds=vec![bounds.into(), inner] />
            <foreignObject
                x=move || bounds.get().left_x()
                y=move || bounds.get().top_y()
                width=move || bounds.get().width()
                height=move || bounds.get().height()
                style="overflow: visible;">
                {html}
            </foreignObject>
        </g>
    }
}

#[component]
fn VerticalBody<X: Clone + 'static, Y: Clone + 'static>(
    series: Memo<Vec<UseY>>,
    state: State<X, Y>,
) -> impl IntoView {
    let padding = move || {
        let p = state.pre.padding.get();
        format!("0 {}px 0 {}px", p.right, p.left)
    };
    view! {
        <For
            each=move || series.get()
            key=|series| series.id
            let:series>
            <tr>
                <td style:padding=padding>
                    <Snippet series=series state=state.clone() />
                </td>
            </tr>
        </For>
    }
}

#[component]
fn HorizontalBody<X: Clone + 'static, Y: Clone + 'static>(
    series: Memo<Vec<UseY>>,
    state: State<X, Y>,
) -> impl IntoView {
    let padding_left = move |i| {
        (i != 0)
            .then_some(state.pre.padding.get().left)
            .map(|p| format!("{}px", p))
    };
    view! {
        <tr>
            <For
                each=move || series.get().into_iter().enumerate()
                key=|(_, series)| series.id
                let:series>
                <td style:padding-left=move || padding_left(series.0)>
                    <Snippet series=series.1 state=state.clone() />
                </td>
            </For>
        </tr>
    }
}
