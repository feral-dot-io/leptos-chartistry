use crate::{
    debug::DebugRect,
    layout::Layout,
    series::{Snippet, UseLine},
    state::{PreState, State},
    ticks::TickFormatFn,
    TickLabels, TickState,
};
use leptos::*;
use std::{
    borrow::Borrow,
    cmp::{Ordering, Reverse},
    rc::Rc,
};

type SortByFn<Y> = dyn Fn(&mut [(UseLine, Option<Y>)]);

#[derive(Clone)]
pub struct Tooltip<X: 'static, Y: 'static> {
    skip_missing: MaybeSignal<bool>,
    table_margin: MaybeSignal<Option<f64>>,
    sort_by: Rc<SortByFn<Y>>,

    x_format: TickFormatFn<X>,
    y_format: TickFormatFn<Y>,

    x_ticks: TickLabels<X>,
    y_ticks: TickLabels<Y>,
}

impl<X: Clone, Y: Clone> Tooltip<X, Y> {
    fn new(x_ticks: impl Borrow<TickLabels<X>>, y_ticks: impl Borrow<TickLabels<Y>>) -> Self {
        Self {
            skip_missing: false.into(),
            table_margin: None.into(),
            sort_by: Rc::new(|_| ()),
            x_format: Rc::new(|s, t| s.long_format(t)),
            y_format: Rc::new(|s, t| s.long_format(t)),
            x_ticks: x_ticks.borrow().clone(),
            y_ticks: y_ticks.borrow().clone(),
        }
    }

    pub fn left_cursor(
        x_ticks: impl Borrow<TickLabels<X>>,
        y_ticks: impl Borrow<TickLabels<Y>>,
    ) -> Self {
        Self::new(x_ticks, y_ticks)
    }
}

impl<X, Y> Tooltip<X, Y> {
    pub fn set_skip_missing(mut self, skip_missing: impl Into<MaybeSignal<bool>>) -> Self {
        self.skip_missing = skip_missing.into();
        self
    }

    pub fn set_table_margin(mut self, table_margin: impl Into<MaybeSignal<Option<f64>>>) -> Self {
        self.table_margin = table_margin.into();
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

    pub fn sort_by(mut self, f: impl Fn(&mut [(UseLine, Option<Y>)]) + 'static) -> Self {
        self.sort_by = Rc::new(f);
        self
    }

    pub fn sort_by_default(self) -> Self {
        self.sort_by(|_| ())
    }
}

impl<X, Y: Clone + Ord + 'static> Tooltip<X, Y> {
    pub fn sort_by_ascending(self) -> Self {
        self.sort_by(|lines: &mut [(UseLine, Option<Y>)]| lines.sort_by_key(|(_, y)| y.clone()))
    }

    pub fn sort_by_descending(self) -> Self {
        self.sort_by(|lines: &mut [(UseLine, Option<Y>)]| {
            lines.sort_by_key(|(_, y)| Reverse(y.clone()))
        })
    }
}

#[derive(Copy, Clone, PartialEq)]
struct F64Ord(f64);

impl PartialOrd for F64Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for F64Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Eq for F64Ord {}

impl<X> Tooltip<X, f64> {
    pub fn sort_by_f64_ascending(self) -> Self {
        self.sort_by(|lines: &mut [(UseLine, Option<f64>)]| {
            lines.sort_by_key(|(_, y)| y.map(F64Ord))
        })
    }

    pub fn sort_by_f64_descending(self) -> Self {
        self.sort_by(|lines: &mut [(UseLine, Option<f64>)]| {
            lines.sort_by_key(|(_, y)| y.map(|y| Reverse(F64Ord(y))))
        })
    }
}

#[component]
pub fn Tooltip<X: Clone + PartialEq + 'static, Y: Clone + PartialEq + 'static>(
    tooltip: Tooltip<X, Y>,
    state: State<X, Y>,
) -> impl IntoView {
    let Tooltip {
        sort_by,
        skip_missing,
        table_margin,
        x_format,
        y_format,
        x_ticks,
        y_ticks,
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
        ..
    } = state;

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

    let nearest_y_values = {
        let nearest_data_y = state.nearest_data_y;
        create_memo(move |_| {
            let mut y_values = nearest_data_y.get();
            // Skip missing?
            if skip_missing.get() {
                y_values = y_values
                    .into_iter()
                    .filter(|(_, y_value)| y_value.is_some())
                    .collect::<Vec<_>>()
            }
            // Sort values
            (sort_by)(&mut y_values);
            y_values
        })
    };

    let nearest_data_y = move || {
        nearest_y_values
            .get()
            .into_iter()
            .map(|(line, y_value)| {
                let y_value = format_y_value(y_value);
                (line, y_value)
            })
            .collect::<Vec<_>>()
    };

    let series_tr = {
        let state = state.clone();
        move |(series, y_value): (UseLine, String)| {
            view! {
                <tr>
                    <td><Snippet series=series state=state.clone() /></td>
                    <td
                        style="white-space: pre; font-family: monospace; text-align: right;"
                        style:padding-top=move || format!("{}px", font.get().height() / 4.0)
                        style:padding-left=move || format!("{}px", font.get().width())>
                        {y_value}
                    </td>
                </tr>
            }
        }
    };

    let table_margin =
        Signal::derive(move || table_margin.get().unwrap_or_else(|| font.get().height()));
    view! {
        <Show when=move || hover_inner.get()>
            <DebugRect label="tooltip" debug=debug />
            <aside
                style="position: absolute; z-index: 1; width: max-content; height: max-content; transform: translateY(-50%); border: 1px solid lightgrey; background-color: #fff; white-space: pre; font-family: monospace;"
                style:top=move || format!("calc({}px)", mouse_page.get().1)
                style:right=move || format!("calc(100% - {}px + {}px)", mouse_page.get().0, table_margin.get())
                style:padding=move || padding.get().to_css_style()>
                <h2
                    style="margin: 0; text-align: center;"
                    style:font-size=move || format!("{}px", font.get().height())>
                    {x_body.clone()}
                </h2>
                <table
                    style="border-collapse: collapse; border-spacing: 0; margin: 0 auto; padding: 0;"
                    style:font-size=move || format!("{}px", font.get().height())>
                    <tbody>
                        <For
                            each=nearest_data_y.clone()
                            key=|(series, y_value)| (series.id, y_value.to_owned())
                            children=series_tr.clone()
                        />
                    </tbody>
                </table>
            </aside>
        </Show>
    }
}
