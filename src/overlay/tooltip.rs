use super::{OverlayLayout, UseOverlay};
use crate::{
    chart::Attr,
    layout::snippet::{SnippetTd, UseSnippet},
    projection::Projection,
    series::{Data, UseSeries},
    ticks::{GeneratedTicks, Ticks},
    use_watched_node::UseWatchedNode,
    Padding, Snippet, TickLabels,
};
use leptos::*;
use std::borrow::Borrow;

#[derive(Clone)]
pub struct Tooltip<X: Clone, Y: Clone> {
    snippet: Snippet,
    table_spacing: MaybeSignal<f64>,
    padding: Option<MaybeSignal<Padding>>,

    x_ticks: TickLabels<X>,
    y_ticks: TickLabels<Y>,
}

#[derive(Clone)]
pub struct TooltipAttr<X: 'static, Y: 'static> {
    snippet: UseSnippet,
    table_spacing: MaybeSignal<f64>,
    padding: MaybeSignal<Padding>,

    x_ticks: Ticks<X>,
    y_ticks: Ticks<Y>,
}

#[derive(Clone)]
pub struct UseTooltip<X: 'static, Y: 'static> {
    snippet: UseSnippet,
    table_spacing: MaybeSignal<f64>,
    padding: MaybeSignal<Padding>,

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
            table_spacing: 0.0.into(),
            padding: None,
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
}

impl<X: Clone + PartialEq + 'static, Y: Clone + PartialEq + 'static> OverlayLayout<X, Y>
    for Tooltip<X, Y>
{
    fn apply_attr(self, attr: &Attr) -> Box<dyn UseOverlay<X, Y>> {
        Box::new(TooltipAttr {
            snippet: self.snippet.to_use(attr),
            table_spacing: self.table_spacing,
            padding: self.padding.unwrap_or(attr.padding),

            x_ticks: self.x_ticks.apply_attr(attr),
            y_ticks: self.y_ticks.apply_attr(attr),
        })
    }
}

impl<X: PartialEq, Y: PartialEq> TooltipAttr<X, Y> {
    fn to_use(self, data: Signal<Data<X, Y>>, proj: Signal<Projection>) -> UseTooltip<X, Y> {
        let avail_width = Projection::derive_width(proj);
        let avail_height = Projection::derive_height(proj);
        UseTooltip {
            snippet: self.snippet,
            table_spacing: self.table_spacing,
            padding: self.padding,
            x_ticks: self.x_ticks.generate_x(data, avail_width).ticks,
            y_ticks: self.y_ticks.generate_y(data, avail_height).ticks,
        }
    }
}

impl<X: Clone + PartialEq, Y: Clone + PartialEq> UseOverlay<X, Y> for TooltipAttr<X, Y> {
    fn render(
        self: Box<Self>,
        series: UseSeries<X, Y>,
        proj: Signal<Projection>,
        watch: &UseWatchedNode,
    ) -> View {
        let tooltip = self.to_use(series.data, proj);
        let (mouse_abs, mouse_rel, over_inner) =
            (watch.mouse_abs, watch.mouse_rel, watch.over_inner);
        create_memo(move |_| {
            if !over_inner.get() {
                return view!().into_view();
            }

            view! {
                <Tooltip
                    tooltip=tooltip.clone()
                    series=series.clone()
                    projection=proj
                    mouse_abs=mouse_abs
                    mouse_rel=mouse_rel
                />
            }
            .into_view()
        })
        .into_view()
    }
}

#[component]
fn Tooltip<X: 'static, Y: 'static>(
    tooltip: UseTooltip<X, Y>,
    series: UseSeries<X, Y>,
    projection: Signal<Projection>,
    mouse_abs: Signal<(f64, f64)>,
    mouse_rel: Signal<(f64, f64)>,
) -> impl IntoView {
    let UseTooltip {
        snippet,
        table_spacing,
        padding,
        x_ticks,
        y_ticks,
    } = tooltip;
    let data = series.data;
    let font = snippet.font;

    // Get nearest values
    let data_x = Signal::derive(move || {
        with!(|mouse_rel, projection| {
            let (data_x, _) = projection.svg_to_data(mouse_rel.0, mouse_rel.1);
            data_x
        })
    });

    // Y-values
    let y_body = create_memo(move |_| {
        (series.lines.clone().into_iter())
            .enumerate()
            .map(|(line_id, line)| {
                let name = line.name.clone();
                let y_value = move || {
                    with!(|data, data_x, y_ticks| {
                        let y_value = data.nearest_y(*data_x, line_id);
                        y_ticks.state.long_format(y_value)
                    })
                };
                view! {
                    <tr>
                        <SnippetTd snippet=snippet.clone() line=line>{name} ":"</SnippetTd>
                        <td
                            style="text-align: left; white-space: pre; font-family: monospace;"
                            style:padding-left=move || format!("{}px", font.get().width())>
                            {y_value}
                        </td>
                    </tr>
                }
            })
            .collect_view()
    });

    view! {
        <div
            style="position: absolute; z-index: 1; width: max-content; height: max-content; transform: translateY(-50%); border: 1px solid lightgrey; background-color: #fff;"
            style:top=move || format!("calc({}px)", mouse_abs.get().1)
            style:right=move || format!("calc(100% - {}px + {}px)", mouse_abs.get().0, table_spacing.get())
            style:padding=move || padding.get().to_style_px()>
            <table
                style="border-collapse: collapse; border-spacing: 0; margin: 0; padding: 0; text-align: right;"
                style:font-size=move || format!("{}px", font.get().height())>
                <thead>
                    <tr>
                        <th colspan=2 style="white-space: pre; font-family: monospace;">
                            {move || with!(|x_ticks, data, data_x| x_ticks.state.long_format(data.nearest_x(*data_x)))}
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
