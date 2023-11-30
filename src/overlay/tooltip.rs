use super::OverlayLayout;
use crate::{
    debug::DebugRect,
    layout::{snippet::SnippetTd, Layout},
    state::{PreState, State},
    ticks::TickFormatFn,
    Snippet, TickLabels, TickState,
};
use leptos::*;
use std::{borrow::Borrow, rc::Rc};

#[derive(Clone)]
pub struct Tooltip<X, Y> {
    snippet: Snippet,
    table_margin: Option<MaybeSignal<f64>>,
    x_format: TickFormatFn<X>,
    y_format: TickFormatFn<Y>,

    x_ticks: TickLabels<X>,
    y_ticks: TickLabels<Y>,
}

impl<X: Clone, Y: Clone> Tooltip<X, Y> {
    fn new(
        snippet: impl Borrow<Snippet>,
        x_ticks: impl Borrow<TickLabels<X>>,
        y_ticks: impl Borrow<TickLabels<Y>>,
    ) -> Self {
        Self {
            snippet: *snippet.borrow(),
            table_margin: None,
            x_format: Rc::new(|s, t| s.long_format(t)),
            y_format: Rc::new(|s, t| s.long_format(t)),
            x_ticks: x_ticks.borrow().clone(),
            y_ticks: y_ticks.borrow().clone(),
        }
    }

    pub fn left_cursor(
        snippet: impl Borrow<Snippet>,
        x_ticks: impl Borrow<TickLabels<X>>,
        y_ticks: impl Borrow<TickLabels<Y>>,
    ) -> Self {
        Self::new(snippet, x_ticks, y_ticks)
    }

    pub fn set_table_margin(mut self, table_margin: impl Into<MaybeSignal<f64>>) -> Self {
        self.table_margin = Some(table_margin.into());
        self
    }

    pub fn set_x_format(
        mut self,
        format: impl Fn(&dyn TickState<Tick = X>, &X) -> String + 'static,
    ) -> Self {
        self.x_format = Rc::new(format);
        self
    }

    pub fn set_y_format(
        mut self,
        format: impl Fn(&dyn TickState<Tick = Y>, &Y) -> String + 'static,
    ) -> Self {
        self.y_format = Rc::new(format);
        self
    }
}

impl<X: Clone + PartialEq, Y: Clone + PartialEq> OverlayLayout<X, Y> for Tooltip<X, Y> {
    fn render(self: Rc<Self>, state: &State<X, Y>) -> View {
        view!( <Tooltip tooltip=(*self).clone() state=&state /> )
    }
}

#[component]
fn Tooltip<'a, X: PartialEq + 'static, Y: Clone + PartialEq + 'static>(
    tooltip: Tooltip<X, Y>,
    state: &'a State<X, Y>,
) -> impl IntoView {
    let Tooltip {
        snippet,
        x_format,
        y_format,
        x_ticks,
        y_ticks,
        ..
    } = tooltip;
    let PreState {
        debug,
        font,
        padding,
        ..
    } = state.pre;
    let State {
        layout: Layout { inner, .. },
        mouse_page,
        hover_inner,
        nearest_data_x,
        nearest_data_y,
        ..
    } = *state;

    let avail_width = Signal::derive(move || with!(|inner| inner.width()));
    let avail_height = Signal::derive(move || with!(|inner| inner.height()));
    let x_ticks = x_ticks.generate_x(&state.pre, avail_width);
    let y_ticks = y_ticks.generate_y(&state.pre, avail_height);

    let x_body = move || {
        with!(|nearest_data_x, x_ticks| {
            nearest_data_x.as_ref().map_or_else(
                || "no data".to_string(),
                |x_value| (x_format)(&*x_ticks.state, x_value),
            )
        })
    };

    let format_y_value = move |y_value: Option<Y>| {
        y_ticks.with(|y_ticks| {
            y_value.as_ref().map_or_else(
                || "-".to_string(),
                |y_value| (y_format)(&*y_ticks.state, y_value),
            )
        })
    };

    let nearest_data_y = move || {
        nearest_data_y
            .get()
            .into_iter()
            .map(|(line, y_value)| {
                let y_value = format_y_value(y_value);
                (line, y_value)
            })
            .collect::<Vec<_>>()
    };

    let table_margin = tooltip
        .table_margin
        .unwrap_or_else(|| Signal::derive(move || font.get().height()).into());
    view! {
        <Show when=move || hover_inner.get()>
            <DebugRect label="tooltip" debug=debug />
            <div
                style="position: absolute; z-index: 1; width: max-content; height: max-content; transform: translateY(-50%); border: 1px solid lightgrey; background-color: #fff;"
                style:top=move || format!("calc({}px)", mouse_page.get().1)
                style:right=move || format!("calc(100% - {}px + {}px)", mouse_page.get().0, table_margin.get())
                style:padding=move || padding.get().to_style_px()
            >
                <table
                    style="border-collapse: collapse; border-spacing: 0; margin: 0; padding: 0; text-align: right;"
                    style:font-size=move || format!("{}px", font.get().height())
                >
                    <thead>
                        <tr>
                            <th colspan=2 style="white-space: pre; font-family: monospace;">
                                {x_body.clone()}
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <For
                            each=nearest_data_y.clone()
                            key=|(_, y_value)| y_value.to_owned()
                            let:line
                        >
                            <tr>
                                <SnippetTd snippet=snippet line=line.0.clone() font=font>{line.0.name} ":"</SnippetTd>
                                <td
                                    style="text-align: left; white-space: pre; font-family: monospace;"
                                    style:padding-left=move || format!("{}px", font.get().width())
                                >
                                    {line.1}
                                </td>
                            </tr>
                        </For>
                    </tbody>
                </table>
            </div>
        </Show>
    }
}
