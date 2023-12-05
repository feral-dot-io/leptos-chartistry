use super::{
    use_series::{self, PrepareSeries, RenderSeries},
    UseLine,
};
use crate::{
    bounds::Bounds,
    colours::{self, ColourScheme},
    state::State,
};
use chrono::prelude::*;
use leptos::*;
use std::rc::Rc;

type GetX<T, X> = Rc<dyn Fn(&T) -> X>;

#[derive(Clone)]
pub struct SeriesData<T: 'static, X: 'static, Y: 'static> {
    get_x: GetX<T, X>,
    series: Vec<Rc<dyn PrepareSeries<T, X, Y>>>,
    colours: ColourScheme,
    min_x: Signal<Option<X>>,
    min_y: Signal<Option<Y>>,
    max_x: Signal<Option<X>>,
    max_y: Signal<Option<Y>>,
}

#[derive(Clone)]
pub struct UseData<X: 'static, Y: 'static> {
    pub lines: Memo<Vec<UseLine>>,
    series: Vec<Rc<dyn RenderSeries<X, Y>>>,

    pub data_x: Memo<Vec<X>>,
    pub data_y: Vec<Memo<Vec<Y>>>,

    pub range_x: Memo<Option<(X, X)>>,
    pub range_y: Vec<Memo<Option<(Y, Y)>>>,
    pub range_y_abs: Memo<Option<(Y, Y)>>,

    pub positions_x: Memo<Vec<f64>>,
    pub positions_y: Vec<Memo<Vec<f64>>>,
    pub position_range: Memo<Bounds>,
}

impl<T: 'static, X: Clone + PartialEq + 'static, Y: Clone + PartialEq + 'static>
    SeriesData<T, X, Y>
{
    pub fn new(get_x: impl Fn(&T) -> X + 'static) -> Self {
        Self {
            get_x: Rc::new(get_x),
            series: Vec::new(),
            colours: colours::ARBITRARY.as_ref().into(),
            min_x: Signal::default(),
            max_x: Signal::default(),
            min_y: Signal::default(),
            max_y: Signal::default(),
        }
    }

    pub fn set_colours(mut self, colours: impl Into<ColourScheme>) -> Self {
        self.colours = colours.into();
        self
    }

    pub fn set_x_min<Opt>(mut self, lower: impl Into<MaybeSignal<Opt>>) -> Self
    where
        Opt: Clone + Into<Option<X>> + 'static,
    {
        let lower = lower.into();
        self.min_x = Signal::derive(move || lower.get().into());
        self
    }

    pub fn set_x_max<Opt>(mut self, upper: impl Into<MaybeSignal<Opt>>) -> Self
    where
        Opt: Clone + Into<Option<X>> + 'static,
    {
        let upper = upper.into();
        self.max_x = Signal::derive(move || upper.get().into());
        self
    }

    pub fn set_x_range<Lower, Upper>(
        self,
        lower: impl Into<MaybeSignal<Lower>>,
        upper: impl Into<MaybeSignal<Upper>>,
    ) -> Self
    where
        Lower: Clone + Into<Option<X>> + 'static,
        Upper: Clone + Into<Option<X>> + 'static,
    {
        self.set_x_min(lower).set_x_max(upper)
    }

    pub fn set_y_min<Opt>(mut self, lower: impl Into<MaybeSignal<Opt>>) -> Self
    where
        Opt: Clone + Into<Option<Y>> + 'static,
    {
        let lower = lower.into();
        self.min_y = Signal::derive(move || lower.get().into());
        self
    }

    pub fn set_y_max<Opt>(mut self, upper: impl Into<MaybeSignal<Opt>>) -> Self
    where
        Opt: Clone + Into<Option<Y>> + 'static,
    {
        let upper = upper.into();
        self.max_y = Signal::derive(move || upper.get().into());
        self
    }

    pub fn set_y_range<LowerOpt, UpperOpt>(
        self,
        lower: impl Into<MaybeSignal<LowerOpt>>,
        upper: impl Into<MaybeSignal<UpperOpt>>,
    ) -> Self
    where
        LowerOpt: Clone + Into<Option<Y>> + 'static,
        UpperOpt: Clone + Into<Option<Y>> + 'static,
    {
        self.set_y_min(lower).set_y_max(upper)
    }

    pub fn add_series(mut self, series: impl PrepareSeries<T, X, Y> + 'static) -> Self {
        self.series.push(Rc::new(series));
        self
    }

    pub fn use_data(self, data: impl Into<Signal<Vec<T>>>) -> UseData<X, Y>
    where
        X: PartialOrd + Position + std::fmt::Debug,
        Y: PartialOrd + Position + std::fmt::Debug,
    {
        let data = data.into();

        // Build list of series
        let (get_ys, lines, series) = use_series::prepare(self.series, self.colours);
        // Sort series by name
        let lines = create_memo(move |_| {
            let mut series = lines.clone();
            series.sort_by_key(|series| series.name.get());
            series
        });

        // Data signals
        let data_x = create_memo(move |_| {
            let get_x = self.get_x.clone();
            data.with(|data| data.iter().map(|datum| (get_x)(datum)).collect::<Vec<_>>())
        });
        let data_y = get_ys
            .into_iter()
            .map(|get_y| {
                create_memo(move |_| {
                    data.with(|data| data.iter().map(|datum| (get_y)(datum)).collect::<Vec<_>>())
                })
            })
            .collect::<Vec<_>>();

        // Position signals
        let positions_x = create_memo(move |_| {
            data_x.with(move |data_x| data_x.iter().map(|x| x.position()).collect::<Vec<_>>())
        });
        let positions_y = data_y
            .iter()
            .map(|&data_y| {
                create_memo(move |_| {
                    data_y
                        .with(move |data_y| data_y.iter().map(|y| y.position()).collect::<Vec<_>>())
                })
            })
            .collect::<Vec<_>>();

        // Range signals
        let range_x: Memo<Option<(X, X)>> = create_memo(move |_| {
            let range: Option<(X, X)> =
                with!(|positions_x, data_x| Self::data_range(positions_x, data_x));

            // Expand specified range to single Option
            let specified: Option<(X, X)> = match (self.min_x.get(), self.max_x.get()) {
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
        let range_y = positions_y
            .iter()
            .zip(data_y.iter())
            .map(|(&positions_y, &data_y)| {
                create_memo(move |_| {
                    with!(|positions_y, data_y| Self::data_range(positions_y, data_y))
                })
            })
            .collect::<Vec<_>>();
        let range_y_abs: Memo<Option<(Y, Y)>> = {
            let range_y = range_y.to_vec();
            create_memo(move |_| {
                // Fetch min / max from each range
                let ranges = range_y.iter().map(|r| r.get());
                let min = ranges
                    .clone()
                    .map(|r| r.map(|(min, _)| min))
                    .chain([self.min_y.get()]) // Specified min
                    .flatten()
                    // Note: ranges are all is_finite
                    .min_by(|a, b| a.position().total_cmp(&b.position()));
                let max = ranges
                    .map(|r| r.map(|(_, max)| max))
                    .chain([self.max_y.get()]) // Specified max
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
            let (min_y, max_y) = range_y_abs
                .get()
                .map(|(min, max)| (min.position(), max.position()))
                .unwrap_or_default();
            Bounds::from_points(min_x, min_y, max_x, max_y)
        });

        UseData {
            lines,
            series,
            data_x,
            data_y,
            range_x,
            range_y,
            range_y_abs,
            positions_x,
            positions_y,
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

    pub fn nearest_data_y(&self, pos_x: Signal<f64>) -> Vec<Memo<Option<Y>>>
    where
        Y: Clone + PartialEq,
    {
        let index = self.nearest_index(pos_x);
        self.data_y
            .iter()
            .map(|&data_y| {
                create_memo(move |_| {
                    index
                        .get()
                        .map(|index| with!(|data_y| data_y[index].clone()))
                })
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
pub fn RenderData<X: Clone + 'static, Y: Clone + 'static>(
    data: UseData<X, Y>,
    state: State<X, Y>,
) -> impl IntoView {
    let proj = state.projection;
    let pos_x = data.positions_x;
    let svg_coords = data
        .positions_y
        .iter()
        .map(|&pos_y| {
            Signal::derive(move || {
                let proj = proj.get();
                with!(|pos_x, pos_y| {
                    pos_x
                        .iter()
                        .zip(pos_y.iter())
                        .map(|(x, y)| proj.position_to_svg(*x, *y))
                        .collect::<Vec<_>>()
                })
            })
        })
        .collect::<Vec<_>>();

    let series = data
        .series
        .into_iter()
        .map(|series| series.render(svg_coords.clone(), &state))
        .collect_view();

    view! {
        <g class="_chartistry_series">
            {series}
        </g>
    }
}
