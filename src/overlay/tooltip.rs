use super::OverlayLayout;
use crate::{
    layout::{snippet::SnippetTd, tick_labels::align_tick_labels},
    line::UseLine,
    projection::Projection,
    series::{Data, UseSeries},
    state::{AttrState, State},
    ticks::{GeneratedTicks, TickFormatFn},
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

#[derive(Clone)]
pub struct UseTooltip<X: 'static, Y: 'static> {
    snippet: Snippet,
    table_margin: Option<MaybeSignal<f64>>,
    x_format: TickFormatFn<X>,
    y_format: TickFormatFn<Y>,

    x_ticks: Signal<GeneratedTicks<X>>,
    y_ticks: Signal<GeneratedTicks<Y>>,
}

impl<X: Clone, Y: Clone> Tooltip<X, Y> {
    fn new(
        snippet: impl Borrow<Snippet>,
        x_ticks: impl Borrow<TickLabels<X>>,
        y_ticks: impl Borrow<TickLabels<Y>>,
    ) -> Self {
        Self {
            snippet: snippet.borrow().clone(),
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

impl<X: PartialEq, Y: PartialEq> Tooltip<X, Y> {
    fn to_use(&self, data: Signal<Data<X, Y>>, state: &State) -> UseTooltip<X, Y> {
        let proj = state.projection;
        let avail_width = Projection::derive_width(proj);
        let avail_height = Projection::derive_height(proj);

        UseTooltip {
            snippet: self.snippet.clone(),
            table_margin: self.table_margin,
            x_format: self.x_format.clone(),
            y_format: self.y_format.clone(),
            x_ticks: self.x_ticks.generate_x(&state.attr, data, avail_width),
            y_ticks: self.y_ticks.generate_y(&state.attr, data, avail_height),
        }
    }
}

impl<X: PartialEq, Y: PartialEq> OverlayLayout<X, Y> for Tooltip<X, Y> {
    fn render(self: Rc<Self>, series: UseSeries<X, Y>, state: &State) -> View {
        let tooltip = self.to_use(series.data, state);
        view! {
            <Tooltip
                tooltip=tooltip
                series=&series
                state=&state
            />
        }
    }
}

#[component]
fn Tooltip<'a, X: 'static, Y: 'static>(
    tooltip: UseTooltip<X, Y>,
    series: &'a UseSeries<X, Y>,
    state: &'a State,
) -> impl IntoView {
    let UseTooltip {
        snippet,
        table_margin,
        x_format,
        y_format,
        x_ticks,
        y_ticks,
    } = tooltip;
    let State {
        attr: AttrState { padding, font, .. },
        projection,
        mouse_page,
        mouse_chart,
        mouse_hover_inner,
        ..
    } = *state;
    let data = series.data;

    // Get nearest values
    let data_x = Signal::derive(move || {
        let (chart_x, chart_y) = mouse_chart.get();
        let (data_x, _) = projection.get().svg_to_data(chart_x, chart_y);
        data_x
    });

    let x_body = move || {
        with!(|x_ticks, data, data_x| {
            data.nearest_x(*data_x).map_or_else(
                || "no data".to_string(),
                |x_value| (x_format)(&*x_ticks.state, x_value),
            )
        })
    };
    let state = state.clone();
    let y_body = {
        let lines = series.lines.clone();
        create_memo(move |_| {
            // Sort lines by name
            let mut lines = lines.clone().into_iter().enumerate().collect::<Vec<_>>();
            lines.sort_by_key(|(_, line)| line.name.get());

            let (lines, labels): (Vec<UseLine>, Vec<String>) = lines
                .into_iter()
                .map(|(line_id, line)| {
                    let y_value = with!(|data, data_x, y_ticks| {
                        data.nearest_y(*data_x, line_id).map_or_else(
                            || "-".to_string(),
                            |y_value| (y_format)(&*y_ticks.state, y_value),
                        )
                    });
                    (line, y_value)
                })
                .unzip();
            let labels = align_tick_labels(labels);
            lines
            .into_iter()
            .zip(labels)
            .map(|(line, label)| {
                let name = line.name.clone();
                view! {
                    <tr>
                        <SnippetTd snippet=snippet.clone() line=line attr=&state.attr>{name} ":"</SnippetTd>
                        <td
                            style="text-align: left; white-space: pre; font-family: monospace;"
                            style:padding-left=move || format!("{}px", font.get().width())>
                            {label}
                        </td>
                    </tr>
                }
            })
            .collect_view()
        })
    };

    let table_margin =
        table_margin.unwrap_or_else(|| Signal::derive(move || font.get().height()).into());
    view! {
        <Show when=move || mouse_hover_inner.get()>
            <div
                style="position: absolute; z-index: 1; width: max-content; height: max-content; transform: translateY(-50%); border: 1px solid lightgrey; background-color: #fff;"
                style:top=move || format!("calc({}px)", mouse_page.get().1)
                style:right=move || format!("calc(100% - {}px + {}px)", mouse_page.get().0, table_margin.get())
                style:padding=move || padding.get().to_style_px()>
                <table
                    style="border-collapse: collapse; border-spacing: 0; margin: 0; padding: 0; text-align: right;"
                    style:font-size=move || format!("{}px", font.get().height())>
                    <thead>
                        <tr>
                            <th colspan=2 style="white-space: pre; font-family: monospace;">
                                {x_body.clone()}
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        {y_body}
                    </tbody>
                </table>
            </div>
        </Show>
    }
}
