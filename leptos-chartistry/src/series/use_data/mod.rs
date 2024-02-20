mod range;
mod values;

pub use range::Range;
pub use values::Values;

use crate::{
    series::{use_y::RenderUseY, UseY},
    state::State,
    Series, Tick,
};
use leptos::*;

#[derive(Clone)]
pub struct UseData<X: 'static, Y: 'static> {
    pub series: Memo<Vec<UseY>>,

    values: Memo<Values<X, Y>>,
    pub range_x: Memo<Range<X>>,
    pub range_y: Memo<Range<Y>>,
}

impl<X: Tick, Y: Tick> UseData<X, Y> {
    pub fn new<T: 'static>(series: Series<T, X, Y>, data: Signal<Vec<T>>) -> UseData<X, Y> {
        let lines = series.to_use_lines();

        // Data -> values
        let values = {
            let lines = lines.clone();
            create_memo(move |_| {
                let get_x = series.get_x.clone();
                data.with(|data| {
                    Values::new(
                        get_x,
                        lines
                            .clone()
                            .into_iter()
                            .map(|(use_y, get_y)| (use_y.id, get_y))
                            .collect(),
                        data,
                    )
                })
            })
        };

        // Range signals
        let range_x: Memo<Range<X>> = create_memo(move |_| {
            values
                .with(|values| values.range_x.clone())
                .maybe_update(vec![series.min_x.get(), series.max_x.get()])
        });
        let range_y: Memo<Range<Y>> = create_memo(move |_| {
            values
                .with(|values| values.range_y.clone())
                .maybe_update(vec![series.min_y.get(), series.max_y.get()])
        });

        // Sort series by name
        let series = {
            let (lines, _): (Vec<_>, Vec<_>) = lines.into_iter().unzip();
            create_memo(move |_| {
                let mut lines = lines.clone();
                lines.sort_by_key(|line| line.name.get());
                lines
            })
        };

        UseData {
            series,
            values,
            range_x,
            range_y,
        }
    }
}

impl<X: 'static, Y: 'static> UseData<X, Y> {
    fn nearest_index(&self, pos_x: Signal<f64>) -> Signal<Option<usize>> {
        let values = self.values;
        Signal::derive(move || {
            values.with(move |values| {
                let positions_x = &values.positions_x;
                // No values
                if positions_x.is_empty() {
                    return None;
                }
                // Find index after pos
                let pos_x = pos_x.get();
                let index = positions_x.partition_point(|&v| v < pos_x);
                // No value before
                if index == 0 {
                    return Some(0);
                }
                // No value ahead
                if index == positions_x.len() {
                    return Some(index - 1);
                }
                // Find closest index
                let ahead = positions_x[index] - pos_x;
                let before = pos_x - positions_x[index - 1];
                if ahead < before {
                    Some(index)
                } else {
                    Some(index - 1)
                }
            })
        })
    }

    pub fn nearest_data_x(&self, pos_x: Signal<f64>) -> Memo<Option<X>>
    where
        X: Clone + PartialEq,
    {
        let values = self.values;
        let index = self.nearest_index(pos_x);
        create_memo(move |_| {
            index
                .get()
                .map(|index| values.with(|values| values.data_x[index].clone()))
        })
    }

    /// Given an arbitrary (unaligned to data) X position, find the nearest X position aligned to data. Returns `f64::NAN` if no data.
    pub fn nearest_position_x(&self, pos_x: Signal<f64>) -> Memo<Option<f64>> {
        let values = self.values;
        let index = self.nearest_index(pos_x);
        create_memo(move |_| {
            index
                .get()
                .map(|index| values.with(|values| values.positions_x[index]))
        })
    }

    pub fn nearest_data_y(&self, pos_x: Signal<f64>) -> Memo<Vec<(UseY, Option<Y>)>>
    where
        Y: Clone + PartialEq,
    {
        let series = self.series;
        let values = self.values;
        let index_x = self.nearest_index(pos_x);
        create_memo(move |_| {
            let index_x = index_x.get();
            series
                .get()
                .into_iter()
                .map(|line| {
                    let y_value = index_x.and_then(|index_x| {
                        values.with(|values| values.data_y[index_x].get(&line.id).cloned())
                    });
                    (line, y_value)
                })
                .collect::<Vec<_>>()
        })
    }
}

#[component]
pub fn RenderData<X: Tick, Y: Tick>(state: State<X, Y>) -> impl IntoView {
    let data = state.pre.data;
    let mk_svg_coords = move |id| {
        Signal::derive(move || {
            let proj = state.projection.get();
            data.values.with(|data| {
                data.positions_x
                    .iter()
                    .enumerate()
                    .map(|(i, &x)| {
                        let y = data.positions_y[i][&id];
                        proj.position_to_svg(x, y)
                    })
                    .collect::<Vec<_>>()
            })
        })
    };

    view! {
        <g class="_chartistry_series">
            <For
                each=move || data.series.get()
                key=|use_y| use_y.id
                let:use_y>
                <RenderUseY use_y=use_y.clone() data=data.clone() positions=mk_svg_coords(use_y.id) />
            </For>
        </g>
    }
}
