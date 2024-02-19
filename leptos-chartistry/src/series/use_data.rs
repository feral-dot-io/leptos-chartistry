use crate::{bounds::Bounds, series::UseY, state::State, Series, Tick};
use leptos::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct UseData<X: 'static, Y: 'static> {
    pub series: Memo<Vec<UseY>>,

    pub data_x: Memo<Vec<X>>,
    data_y: Memo<Vec<HashMap<usize, Y>>>,

    pub range_x: Memo<Option<(X, X)>>,
    /// Yields the min / max Y values. Still returns a range if min / max are set and no data.
    pub range_y: Memo<Option<(Y, Y)>>,

    pub positions_x: Memo<Vec<f64>>,
    positions_y: Memo<Vec<HashMap<usize, f64>>>,
    pub position_range: Memo<Bounds>,
}

impl<X: Tick, Y: Tick> UseData<X, Y> {
    pub fn new<T: 'static>(series: Series<T, X, Y>, data: Signal<Vec<T>>) -> UseData<X, Y> {
        let lines = series.to_use_lines();
        let Series {
            get_x,
            min_x,
            max_x,
            min_y,
            max_y,
            ..
        } = series;

        // Sort series by name
        let series = {
            let (lines, _): (Vec<_>, Vec<_>) = lines.clone().into_iter().unzip();
            create_memo(move |_| {
                let mut lines = lines.clone();
                lines.sort_by_key(|line| line.name.get());
                lines
            })
        };

        // Data signals
        let data_x = create_memo(move |_| {
            data.with(|data| data.iter().map(|datum| (get_x)(datum)).collect::<Vec<_>>())
        });
        let y_maker = |which: bool| {
            let lines = lines.clone();
            create_memo(move |_| {
                data.with(|data| {
                    data.iter()
                        .map(|datum| {
                            lines
                                .iter()
                                .map(|(line, get_y)| {
                                    let y = if which {
                                        get_y.value(datum)
                                    } else {
                                        get_y.cumulative_value(datum)
                                    };
                                    (line.id, y)
                                })
                                .collect::<HashMap<_, _>>()
                        })
                        .collect::<Vec<_>>()
                })
            })
        };
        // Generate two sets of Ys: original and cumulative value. They can differ when stacked
        let data_y = y_maker(true);
        let data_y_cumulative = y_maker(false);

        // Position signals
        let positions_x = create_memo(move |_| {
            data_x.with(move |data_x| data_x.iter().map(|x| x.position()).collect::<Vec<_>>())
        });
        let positions_y = create_memo(move |_| {
            data_y_cumulative
                .get()
                .into_iter()
                .map(|ys| {
                    ys.into_iter()
                        .map(|(id, y)| (id, y.position()))
                        .collect::<HashMap<_, _>>()
                })
                .collect::<Vec<_>>()
        });

        // Range signals
        let range_x: Memo<Option<(X, X)>> = create_memo(move |_| {
            let range = with!(|positions_x, data_x| {
                positions_x
                    .iter()
                    .enumerate()
                    .fold(None, range_minmax_acc)
                    .map(|((min_i, _), (max_i, _))| {
                        let min_x = data_x[min_i].clone();
                        let max_x = data_x[max_i].clone();
                        (min_x, max_x)
                    })
            });
            extend_range(range, min_x.get(), max_x.get())
        });
        let range_y: Memo<Option<(Y, Y)>> = create_memo(move |_| {
            let range = with!(|positions_y| {
                positions_y
                    .iter()
                    .enumerate()
                    .fold(None, |acc, (i, ys)| {
                        ys.iter()
                            .map(|(j, y)| ((i, j), y))
                            .fold(acc, range_minmax_acc)
                    })
                    .map(|(((min_i, min_j), _), ((max_i, max_j), _))| {
                        data_y_cumulative.with(|data_y| {
                            let min_y = data_y[min_i].get(min_j).unwrap().clone();
                            let max_y = data_y[max_i].get(max_j).unwrap().clone();
                            (min_y, max_y)
                        })
                    })
            });
            extend_range(range, min_y.get(), max_y.get())
        });
        // Position range signal
        let position_range = create_memo(move |_| {
            let (min_x, max_x) = range_x
                .get()
                .map(|(min, max)| (min.position(), max.position()))
                .unwrap_or_default();
            let (min_y, max_y) = range_y
                .get()
                .map(|(min, max)| (min.position(), max.position()))
                .unwrap_or_default();
            Bounds::from_points(min_x, min_y, max_x, max_y)
        });

        UseData {
            series,
            data_x,
            data_y,
            range_x,
            range_y,
            positions_x,
            positions_y,
            position_range,
        }
    }
}

fn range_minmax_acc<K: Copy>(
    acc: Option<((K, f64), (K, f64))>,
    (key, &pos): (K, &f64),
) -> Option<((K, f64), (K, f64))> {
    if pos.is_finite() {
        acc.map(|(min @ (_, min_pos), max @ (_, max_pos))| {
            (
                if pos < min_pos { (key, pos) } else { min },
                if pos > max_pos { (key, pos) } else { max },
            )
        })
        .or(Some(((key, pos), (key, pos))))
    } else {
        acc
    }
}

/// If upper / lower are set, extends the given range (even if range is None).
fn extend_range<V: Tick>(
    range: Option<(V, V)>,
    lower: Option<V>,
    upper: Option<V>,
) -> Option<(V, V)> {
    // Find a V value
    let (r0, r1) = range.clone().unzip();
    let v = lower.clone().or(upper.clone()).or(r0).or(r1);
    v.map(|v| {
        // Unravel options
        let (min, max) = range.unwrap_or_else(|| (v.clone(), v.clone()));
        let lower = lower.unwrap_or_else(|| v.clone());
        let upper = upper.unwrap_or_else(|| v.clone());
        let lower_p = lower.position();
        let upper_p = upper.position();
        // Extend range?
        (
            if min.position() < lower_p { min } else { lower },
            if max.position() > upper_p { max } else { upper },
        )
    })
}

impl<X: 'static, Y: 'static> UseData<X, Y> {
    fn nearest_index(&self, pos_x: Signal<f64>) -> Signal<Option<usize>> {
        let positions_x = self.positions_x;
        Signal::derive(move || {
            positions_x.with(move |positions_x| {
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
        let data_x = self.data_x;
        let index = self.nearest_index(pos_x);
        create_memo(move |_| {
            index
                .get()
                .map(|index| with!(|data_x| data_x[index].clone()))
        })
    }

    /// Given an arbitrary (unaligned to data) X position, find the nearest X position aligned to data. Returns `f64::NAN` if no data.
    pub fn nearest_position_x(&self, pos_x: Signal<f64>) -> Memo<Option<f64>> {
        let positions_x = self.positions_x;
        let index = self.nearest_index(pos_x);
        create_memo(move |_| index.get().map(|index| positions_x.with(|pos| pos[index])))
    }

    pub fn nearest_data_y(&self, pos_x: Signal<f64>) -> Memo<Vec<(UseY, Option<Y>)>>
    where
        Y: Clone + PartialEq,
    {
        let series = self.series;
        let data_y = self.data_y;
        let index_x = self.nearest_index(pos_x);
        create_memo(move |_| {
            let index_x = index_x.get();
            series
                .get()
                .into_iter()
                .map(|line| {
                    let y_value = index_x
                        .and_then(|index_x| with!(|data_y| data_y[index_x].get(&line.id).cloned()));
                    (line, y_value)
                })
                .collect::<Vec<_>>()
        })
    }
}

#[component]
pub fn RenderData<X: Tick, Y: Tick>(state: State<X, Y>) -> impl IntoView {
    let data = state.pre.data;
    let pos_x = data.positions_x;
    let pos_y = data.positions_y;
    let proj = state.projection;
    let mk_svg_coords = move |id| {
        Signal::derive(move || {
            let proj = proj.get();
            pos_x
                .get()
                .into_iter()
                .enumerate()
                .map(|(i, x)| {
                    // TODO: our data model guarantees unwrap always succeeds but this doesn't hold true if we move to separated data iterators
                    let y = pos_y.with(|pos_y| *pos_y[i].get(&id).unwrap());
                    proj.position_to_svg(x, y)
                })
                .collect::<Vec<_>>()
        })
    };

    view! {
        <g class="_chartistry_series">
            <For
                each=move || data.series.get()
                key=|line| line.id
                children=move |line| line.render(data.clone(), mk_svg_coords(line.id))
            />
        </g>
    }
}
