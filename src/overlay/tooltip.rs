use super::{OverlayLayout, UseOverlay};
use crate::{
    chart::Attr,
    layout::snippet::{SnippetTd, UseSnippet},
    projection::Projection,
    series::UseSeries,
    ticks::Ticks,
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
pub struct UseTooltip<X: 'static, Y: 'static> {
    snippet: UseSnippet,
    table_spacing: MaybeSignal<f64>,
    padding: MaybeSignal<Padding>,

    x_ticks: Ticks<X>,
    y_ticks: Ticks<Y>,
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

impl<X: Clone + 'static, Y: Clone + 'static> OverlayLayout<X, Y> for Tooltip<X, Y> {
    fn apply_attr(self, attr: &Attr) -> Box<dyn UseOverlay<X, Y>> {
        Box::new(UseTooltip {
            snippet: self.snippet.to_use(attr),
            table_spacing: self.table_spacing,
            padding: self.padding.unwrap_or(attr.padding),

            x_ticks: self.x_ticks.apply_attr(attr),
            y_ticks: self.y_ticks.apply_attr(attr),
        })
    }
}

impl<X: Clone, Y: Clone> UseOverlay<X, Y> for UseTooltip<X, Y> {
    fn render(
        self: Box<Self>,
        series: UseSeries<X, Y>,
        proj: Signal<Projection>,
        watch: &UseWatchedNode,
    ) -> View {
        let (mouse_abs, mouse_rel, over_inner) =
            (watch.mouse_abs, watch.mouse_rel, watch.over_inner);
        Signal::derive(move || {
            if !over_inner.get() {
                return view!().into_view();
            }

            let series = series.clone();
            let (abs_x, abs_y) = mouse_abs.get();
            let (rel_x, rel_y) = mouse_rel.get();

            view! {
                <Tooltip
                    tooltip=*self.clone()
                    series=series
                    projection=proj
                    abs_x=abs_x
                    abs_y=abs_y
                    rel_x=rel_x
                    rel_y=rel_y
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
    abs_x: f64,
    abs_y: f64,
    rel_x: f64,
    rel_y: f64,
) -> impl IntoView {
    let UseTooltip {
        snippet,
        table_spacing,
        padding,
        x_ticks,
        y_ticks,
    } = tooltip;
    let data = series.data;
    let avail_width = Projection::derive_width(projection);
    let avail_height = Projection::derive_height(projection);
    let x_ticks = x_ticks.generate_x(series.data, avail_width).ticks;
    let y_ticks = y_ticks.generate_y(series.data, avail_height).ticks;
    let font = snippet.font;

    // Get nearest values
    let data_x = Signal::derive(move || {
        let (data_x, _) = projection.get().svg_to_data(rel_x, rel_y);
        data_x
    });

    // Y-values
    let y_body = Signal::derive(move || {
        with!(|data, data_x| {
            let ys = data.nearest_y(*data_x);
            (series.lines.clone().into_iter())
                .zip(ys)
                .map(|(line, y)| {
                    let name = line.name.clone();
                    view! {
                        <tr>
                            <SnippetTd snippet=snippet.clone() line=line>{name} ":"</SnippetTd>
                            <td
                                style="text-align: left; white-space: pre; font-family: monospace;"
                                style:padding-left=format!("{}px", snippet.font.get().width())>
                                {with!(|y_ticks| y_ticks.state.long_format(y))}
                            </td>
                        </tr>
                    }
                })
                .collect_view()
        })
    });

    view! {
        <div
            style="position: absolute; z-index: 1; width: max-content; height: max-content; transform: translateY(-50%); border: 1px solid lightgrey; background-color: #fff;"
            style:top=format!("calc({abs_y}px)")
            style:right=move || format!("calc(100% - {abs_x}px + {}px)", table_spacing.get())
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
