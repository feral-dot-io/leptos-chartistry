use super::{OverlayLayout, UseOverlay};
use crate::{
    layout::{
        snippet::{SnippetTd, UseSnippet},
        tick_labels::{align_tick_labels, TickLabelsAttr},
    },
    line::UseLine,
    projection::Projection,
    series::{Data, UseSeries},
    state::{AttrState, State},
    ticks::{long_format_fn, GeneratedTicks, TickFormatFn},
    Snippet, TickLabels,
};
use leptos::*;
use std::{borrow::Borrow, rc::Rc};

#[derive(Clone)]
pub struct Tooltip<X: Clone, Y: Clone> {
    snippet: Snippet,
    table_margin: Option<MaybeSignal<f64>>,

    x_ticks: TickLabels<X>,
    y_ticks: TickLabels<Y>,
}

#[derive(Clone)]
pub struct TooltipAttr<X: 'static, Y: 'static> {
    snippet: UseSnippet,
    table_margin: MaybeSignal<f64>,

    x_ticks: TickLabelsAttr<X>,
    y_ticks: TickLabelsAttr<Y>,
}

#[derive(Clone)]
pub struct UseTooltip<X: 'static, Y: 'static> {
    snippet: UseSnippet,
    table_margin: MaybeSignal<f64>,

    x_format: TickFormatFn<X>,
    x_ticks: Signal<GeneratedTicks<X>>,
    y_format: TickFormatFn<Y>,
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
}

impl<X: Clone + PartialEq + 'static, Y: Clone + PartialEq + 'static> OverlayLayout<X, Y>
    for Tooltip<X, Y>
{
    fn apply_attr(self, attr: &AttrState) -> Rc<dyn UseOverlay<X, Y>> {
        let font = attr.font;
        Rc::new(TooltipAttr {
            snippet: self.snippet.into_use(attr),
            table_margin: self
                .table_margin
                .unwrap_or_else(|| Signal::derive(move || font.get().height()).into()),

            x_ticks: self.x_ticks.apply_attr(attr, long_format_fn()),
            y_ticks: self.y_ticks.apply_attr(attr, long_format_fn()),
        })
    }
}

impl<X: PartialEq, Y: PartialEq> TooltipAttr<X, Y> {
    fn into_use(self, data: Signal<Data<X, Y>>, proj: Signal<Projection>) -> UseTooltip<X, Y> {
        let avail_width = Projection::derive_width(proj);
        let avail_height = Projection::derive_height(proj);
        UseTooltip {
            snippet: self.snippet,
            table_margin: self.table_margin,
            x_format: self.x_ticks.format.clone(),
            x_ticks: self.x_ticks.generate_x(data, avail_width),
            y_format: self.y_ticks.format.clone(),
            y_ticks: self.y_ticks.generate_y(data, avail_height),
        }
    }
}

impl<X: Clone + PartialEq, Y: Clone + PartialEq> UseOverlay<X, Y> for TooltipAttr<X, Y> {
    fn render(self: Rc<Self>, series: UseSeries<X, Y>, state: &State) -> View {
        let State {
            projection,
            mouse_page,
            mouse_chart,
            mouse_hover_inner,
            ..
        } = *state;

        let tooltip = (*self).clone().into_use(series.data, projection);
        let state = state.clone();
        create_memo(move |_| {
            if !mouse_hover_inner.get() {
                return view!().into_view();
            }

            view! {
                <Tooltip
                    tooltip=tooltip.clone()
                    series=series.clone()
                    state=&state
                    mouse_page=mouse_page
                    mouse_chart=mouse_chart
                />
            }
            .into_view()
        })
        .into_view()
    }
}

#[component]
fn Tooltip<'a, X: 'static, Y: 'static>(
    tooltip: UseTooltip<X, Y>,
    series: UseSeries<X, Y>,
    state: &'a State,
    mouse_page: Signal<(f64, f64)>,
    mouse_chart: Signal<(f64, f64)>,
) -> impl IntoView {
    let proj = state.projection;
    let padding = state.attr.padding;
    let UseTooltip {
        snippet,
        table_margin,
        x_format,
        x_ticks,
        y_format,
        y_ticks,
    } = tooltip;
    let data = series.data;
    let font = snippet.font;

    // Get nearest values
    let data_x = Signal::derive(move || {
        let (chart_x, chart_y) = mouse_chart.get();
        let (data_x, _) = proj.get().svg_to_data(chart_x, chart_y);
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
    let y_body = create_memo(move |_| {
        // Sort lines by name
        let mut lines = series
            .lines
            .clone()
            .into_iter()
            .enumerate()
            .collect::<Vec<_>>();
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
                        <SnippetTd snippet=snippet.clone() line=line>{name} ":"</SnippetTd>
                        <td
                            style="text-align: left; white-space: pre; font-family: monospace;"
                            style:padding-left=move || format!("{}px", font.get().width())>
                            {label}
                        </td>
                    </tr>
                }
            })
            .collect_view()
    });

    view! {
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
                            {x_body}
                        </th>
                    </tr>
                </thead>
                <tbody>
                    {y_body}
                </tbody>
            </table>
        </div>
    }
}
