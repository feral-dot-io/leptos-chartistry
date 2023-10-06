use crate::{bounds::Bounds, projection::Projection, Line};
use leptos::*;
use std::borrow::Borrow;

pub struct Series<T: 'static, X: 'static, Y: 'static> {
    get_x: &'static dyn Fn(&T) -> X,
    get_ys: Vec<&'static dyn Fn(&T) -> Y>,
    lines: Vec<Line>,
}

#[derive(Clone, Debug)]
pub struct UseSeries<X: 'static, Y: 'static> {
    pub(crate) lines: Vec<Line>,
    pub(crate) data: Signal<Data<X, Y>>,
}

#[derive(Clone, Debug)]
pub struct Data<X, Y> {
    position_range: Bounds,
    x_points: Vec<X>,
    x_positions: Vec<f64>,
    y_points: Vec<Y>,
    y_positions: Vec<f64>,
}

impl<T, X, Y> Series<T, X, Y> {
    pub fn new(get_x: &'static dyn Fn(&T) -> X) -> Self {
        Series {
            get_x,
            get_ys: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn add(mut self, line: Line, get_y: &'static dyn Fn(&T) -> Y) -> Self {
        self.get_ys.push(get_y);
        self.lines.push(line);
        self
    }

    pub fn use_data<Ts>(self, data: impl Into<MaybeSignal<Ts>> + 'static) -> UseSeries<X, Y>
    where
        Ts: Borrow<[T]> + 'static,
        X: PartialOrd + Position,
        Y: PartialOrd + Position,
    {
        let Series {
            get_x,
            get_ys,
            lines,
        } = self;

        let data = data.into();
        let data = Signal::derive(move || {
            let get_ys = get_ys.iter();
            data.with(move |data| {
                let data = data.borrow();

                // Collect data points
                let x_points = data.iter().map(get_x).collect::<Vec<_>>();
                let x_positions = x_points.iter().map(|x| x.position()).collect::<Vec<_>>();
                let y_points = (get_ys.into_iter())
                    .flat_map(|get_y| data.iter().map(get_y))
                    .collect::<Vec<_>>();
                let y_positions = y_points.iter().map(|y| y.position()).collect::<Vec<_>>();

                // Position range
                // TODO handle empty data -- should be configurable
                let (x_min, x_max) = (x_positions.iter())
                    .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &x| {
                        (min.min(x), max.max(x))
                    });
                let (y_min, y_max) = (y_positions.iter())
                    .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &y| {
                        (min.min(y), max.max(y))
                    });

                Data {
                    position_range: Bounds::from_points(x_min, y_min, x_max, y_max),
                    x_points,
                    x_positions,
                    y_points,
                    y_positions,
                }
            })
        });

        UseSeries { lines, data }
    }
}

impl<X, Y> Data<X, Y> {
    pub fn position_range(&self) -> Bounds {
        self.position_range
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

#[component]
pub(crate) fn Series<X: 'static, Y: 'static>(
    series: UseSeries<X, Y>,
    projection: Signal<Projection>,
) -> impl IntoView {
    let lines = move || {
        let proj = projection.get();
        series.data.with(|data| {
            let points = data.x_points.len();
            (series.lines.iter())
                .enumerate()
                .map(|(line_i, line)| {
                    let positions = (data.x_positions.iter())
                        .enumerate()
                        .map(|(i, &x)| {
                            let y = data.y_positions[line_i * points + i];
                            // Map from data to viewport coords
                            proj.data_to_svg(x, y)
                        })
                        .collect::<Vec<_>>();
                    view! {
                        <g class=format!("_chartistry_line_{}", line_i)>
                            <Line line=line positions=positions />
                        </g>
                    }
                })
                .collect_view()
        })
    };
    view! {
        <g class="_chartistry_series">{lines}</g>
    }
}
