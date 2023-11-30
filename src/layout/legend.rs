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
    state::{PreState, State},
    Font, Padding,
};
use leptos::*;
use std::{borrow::Borrow, rc::Rc};

#[derive(Clone, Debug)]
pub struct Legend {
    snippet: Snippet,
    anchor: MaybeSignal<Anchor>,
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

    pub(crate) fn fixed_height<X, Y>(&self, state: &PreState<X, Y>) -> Signal<f64> {
        let font = state.font;
        let padding = state.padding;
        Signal::derive(move || font.get().height() + padding.get().height())
    }

    pub(crate) fn width<X, Y>(state: &PreState<X, Y>) -> Signal<f64> {
        let PreState {
            font,
            padding,
            lines,
            ..
        } = *state;
        let snippet_width = Snippet::width(font);
        Signal::derive(move || {
            let font_width = font.get().width();
            let max_chars = lines
                .get()
                .into_iter()
                .map(|line| line.name.get().len() as f64 * font_width)
                .reduce(f64::max)
                .unwrap_or_default();
            snippet_width.get() + max_chars + padding.get().width()
        })
    }
}

impl<X, Y> HorizontalLayout<X, Y> for Legend {
    fn fixed_height(&self, state: &PreState<X, Y>) -> Signal<f64> {
        self.fixed_height(state)
    }

    fn into_use(self: Rc<Self>, _: &PreState<X, Y>, _: Memo<f64>) -> Rc<dyn UseLayout<X, Y>> {
        self
    }
}

impl<X, Y> VerticalLayout<X, Y> for Legend {
    fn into_use(
        self: Rc<Self>,
        state: &PreState<X, Y>,
        _: Memo<f64>,
    ) -> (Signal<f64>, Rc<dyn UseLayout<X, Y>>) {
        (Self::width(state), self)
    }
}

impl<X, Y> UseLayout<X, Y> for Legend {
    fn render(&self, edge: Edge, bounds: Memo<Bounds>, state: &State<X, Y>) -> View {
        view! { <Legend legend=self.clone() edge=edge bounds=bounds state=state /> }
    }
}

#[component]
pub fn Legend<'a, X: 'static, Y: 'static>(
    legend: Legend,
    edge: Edge,
    bounds: Memo<Bounds>,
    state: &'a State<X, Y>,
) -> impl IntoView {
    let Legend { snippet, anchor } = legend;
    let PreState {
        debug,
        padding,
        font,
        lines,
        ..
    } = state.pre;

    // Don't apply padding on the edges of our axis i.e., maximise the space we extend over
    let padding = create_memo(move |_| {
        let padding = padding.get();
        if edge.is_horizontal() {
            Padding::sides(padding.top, 0.0, padding.bottom, 0.0)
        } else {
            Padding::sides(0.0, padding.right, 0.0, padding.left)
        }
    });
    let inner = Signal::derive(move || padding.get().apply(bounds.get()));

    let (body, anchor_dir) = if edge.is_horizontal() {
        (
            view!(<HorizontalBody snippet=snippet lines=lines font=font />),
            "row",
        )
    } else {
        (
            view!(<VerticalBody snippet=snippet lines=lines font=font />),
            "column",
        )
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

#[component]
fn VerticalBody(snippet: Snippet, lines: Memo<Vec<UseLine>>, font: Signal<Font>) -> impl IntoView {
    view! {
        <For
            each=move || lines.get().into_iter().enumerate()
            key=|(_, line)| line.name.get()
            let:line>
            <tr>
                <SnippetTd snippet=snippet line=line.1.clone() font=font>
                    {line.1.name.get()}
                </SnippetTd>
            </tr>
        </For>
    }
}

#[component]
fn HorizontalBody(
    snippet: Snippet,
    lines: Memo<Vec<UseLine>>,
    font: Signal<Font>,
) -> impl IntoView {
    view! {
        <tr>
            <For
                each=move || lines.get().into_iter().enumerate()
                key=|(_, line)| line.name.get()
                let:line>
                <SnippetTd snippet=snippet line=line.1.clone() font=font left_padding=line.0 != 0>
                    {line.1.name.get()}
                </SnippetTd>
            </For>
        </tr>
    }
}
