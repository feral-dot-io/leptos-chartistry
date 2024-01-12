use crate::{bounds::Bounds, series::UseLine, state::State, Series};
use chrono::prelude::*;
use leptos::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct UseData<X: 'static, Y: 'static> {
    series_by_id: HashMap<usize, UseLine>,
    pub series: Memo<Vec<UseLine>>,

    pub data_x: Memo<Vec<X>>,
    pub data_y_lines: HashMap<usize, Memo<Vec<Y>>>,

    pub range_x: Memo<Option<(X, X)>>,
    pub range_y: Memo<Option<(Y, Y)>>,
    pub range_y_lines: HashMap<usize, Memo<Option<(Y, Y)>>>,

    pub positions_x: Memo<Vec<f64>>,
    pub positions_y_lines: HashMap<usize, Memo<Vec<f64>>>,
    pub position_range: Memo<Bounds>,
}

impl<X: Clone + PartialEq + 'static, Y: Clone + PartialEq + 'static> UseData<X, Y> {
    pub fn new<T: 'static>(
        series: Series<T, X, Y>,
        min_x: MaybeSignal<Option<X>>,
        max_x: MaybeSignal<Option<X>>,
        min_y: MaybeSignal<Option<Y>>,
        max_y: MaybeSignal<Option<Y>>,
        data: Signal<Vec<T>>,
    ) -> UseData<X, Y>
    where
        X: PartialOrd + Position,
        Y: PartialOrd + Position,
    {
        let Series {
            get_x,
            lines,
            get_ys,
            ..
        } = series;

        // Sort series by name
        let series = {
            let series = lines.values().cloned().collect::<Vec<_>>();
            create_memo(move |_| {
                let mut series = series.clone();
                series.sort_by_key(|series| series.name.get());
                series
            })
        };

        // Data signals
        let data_x = create_memo(move |_| {
            data.with(|data| data.iter().map(|datum| (get_x)(datum)).collect::<Vec<_>>())
        });
        let y_maker = |value: bool| {
            get_ys
                .clone()
                .into_iter()
                .map(|(id, get_y)| {
                    let values = create_memo(move |_| {
                        data.with(|data| {
                            data.iter()
                                .map(|datum| {
                                    // This is ick yet practical
                                    if value {
                                        get_y.value(datum)
                                    } else {
                                        get_y.position(datum)
                                    }
                                })
                                .collect::<Vec<_>>()
                        })
                    });
                    (id, values)
                })
                .collect::<HashMap<_, _>>()
        };
        // Generate two sets of Ys: original and chart position. They can differ when stacked
        let data_y_lines = y_maker(true);
        let data_y_positions = y_maker(false);

        // Position signals
        let positions_x = create_memo(move |_| {
            data_x.with(move |data_x| data_x.iter().map(|x| x.position()).collect::<Vec<_>>())
        });
        let positions_y_lines = data_y_positions
            .iter()
            .map(|(id, &data_y)| {
                let positions = create_memo(move |_| {
                    data_y
                        .with(move |data_y| data_y.iter().map(|y| y.position()).collect::<Vec<_>>())
                });
                (*id, positions)
            })
            .collect::<HashMap<_, _>>();

        // Range signals
        let range_x: Memo<Option<(X, X)>> = create_memo(move |_| {
            let range: Option<(X, X)> =
                with!(|positions_x, data_x| Self::data_range(positions_x, data_x));

            // Expand specified range to single Option
            let specified: Option<(X, X)> = match (min_x.get(), max_x.get()) {
                (Some(min_x), Some(max_x)) => Some((min_x.clone(), max_x.clone())),
                (Some(min_x), None) => Some((min_x.clone(), min_x.clone())),
                (None, Some(max_x)) => Some((max_x.clone(), max_x.clone())),
                (None, None) => None,
            };

            // Extend range by specified?
            match (range, specified) {
                (None, None) => None, // No data, no range

                // One of range or specified
                (Some(range), None) => Some(range),
                (None, Some(specified)) => Some(specified),

                // Calculate min / max of range and specified
                (Some((min_r, max_r)), Some((min_s, max_s))) => Some((
                    if min_r.position() < min_s.position() {
                        min_r
                    } else {
                        min_s
                    },
                    if max_r.position() > max_s.position() {
                        max_r
                    } else {
                        max_s
                    },
                )),
            }
        });
        let range_y_lines = lines
            .keys()
            .map(|&id| {
                let positions_y = positions_y_lines[&id];
                let data_y = data_y_positions[&id];
                let ranges = create_memo(move |_| {
                    with!(|positions_y, data_y| Self::data_range(positions_y, data_y))
                });
                (id, ranges)
            })
            .collect::<HashMap<_, _>>();

        let range_y = {
            let range_y = range_y_lines.clone().into_values().collect::<Vec<_>>();
            create_memo(move |_| {
                // Fetch min / max from each range
                let ranges = range_y.iter().map(|r| r.get());
                let min = ranges
                    .clone()
                    .map(|r| r.map(|(min, _)| min))
                    .chain([min_y.get()]) // Specified min
                    .flatten()
                    // Note: ranges are all is_finite
                    .min_by(|a, b| a.position().total_cmp(&b.position()));
                let max = ranges
                    .map(|r| r.map(|(_, max)| max))
                    .chain([max_y.get()]) // Specified max
                    .flatten()
                    .max_by(|a, b| a.position().total_cmp(&b.position()));
                min.zip(max).map(|(min, max)| (min.clone(), max.clone()))
            })
        };

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
            series_by_id: lines,
            series,
            data_x,
            data_y_lines,
            range_x,
            range_y,
            range_y_lines,
            positions_x,
            positions_y_lines,
            position_range,
        }
    }

    /// Given a list of positions. Finds the min / max indexes using is_finite to skip infinite and NaNs. Returns the data values at those indexes. Returns `None` if no data.
    fn data_range<V: Clone + PartialOrd>(positions: &[f64], data: &[V]) -> Option<(V, V)> {
        // Find min / max indexes in positions
        let indexes = positions.iter().enumerate().fold(None, |acc, (i, &pos)| {
            if pos.is_finite() {
                acc.map(|(min, max)| {
                    (
                        if pos < positions[min] { i } else { min },
                        if pos > positions[max] { i } else { max },
                    )
                })
                .or(Some((i, i)))
            } else {
                acc
            }
        });
        // Return data values
        indexes.map(|(min, max)| (data[min].clone(), data[max].clone()))
    }
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
    pub fn nearest_position_x(&self, pos_x: Signal<f64>) -> Memo<f64> {
        let positions_x = self.positions_x;
        let index = self.nearest_index(pos_x);
        create_memo(move |_| {
            index
                .get()
                .map(|index| with!(|positions_x| positions_x[index]))
                .unwrap_or(f64::NAN)
        })
    }

    pub fn nearest_position_y(&self, pos_x: Signal<f64>) -> Vec<(UseLine, Memo<f64>)>
    where
        Y: Clone + PartialEq,
    {
        let index_x = self.nearest_index(pos_x);
        self.positions_y_lines
            .iter()
            .map(|(id, &pos_y)| {
                let line = self.series_by_id[&id].clone();
                let value = create_memo(move |_| {
                    index_x
                        .get()
                        .map(|index_x| with!(|pos_y| pos_y[index_x]))
                        .unwrap_or(f64::NAN)
                });
                (line, value)
            })
            .collect::<Vec<_>>()
    }

    pub fn nearest_data_y(&self, pos_x: Signal<f64>) -> Vec<(UseLine, Memo<Option<Y>>)>
    where
        Y: Clone + PartialEq,
    {
        let index_x = self.nearest_index(pos_x);
        self.data_y_lines
            .iter()
            .map(|(id, &data_y)| {
                let line = self.series_by_id[&id].clone();
                let values = create_memo(move |_| {
                    index_x
                        .get()
                        .map(|index_x| with!(|data_y| data_y[index_x].clone()))
                });
                (line, values)
            })
            .collect::<Vec<_>>()
    }
}

pub trait Position {
    fn position(&self) -> f64;
}

impl Position for f64 {
    fn position(&self) -> f64 {
        *self
    }
}

impl<Tz: TimeZone> Position for DateTime<Tz> {
    fn position(&self) -> f64 {
        self.timestamp() as f64 + (self.timestamp_subsec_nanos() as f64 / 1e9)
    }
}

#[component]
pub fn RenderData<X: Clone + 'static, Y: Clone + 'static>(state: State<X, Y>) -> impl IntoView {
    let data = state.pre.data;
    let proj = state.projection;
    let pos_x = data.positions_x;
    let svg_coords = data
        .positions_y_lines
        .iter()
        .map(|(&id, &pos_y)| {
            let coords = Signal::derive(move || {
                let proj = proj.get();
                with!(|pos_x, pos_y| {
                    pos_x
                        .iter()
                        .zip(pos_y.iter())
                        .map(|(x, y)| proj.position_to_svg(*x, *y))
                        .collect::<Vec<_>>()
                })
            });
            (id, coords)
        })
        .collect::<HashMap<_, _>>();

    view! {
        <g class="_chartistry_series">
            <For
                each=move || data.series.get()
                key=|line| line.id
                children=move |line| line.render(svg_coords[&line.id])
            />
        </g>
    }
}
