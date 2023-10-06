use crate::{projection::Projection, Line};
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
    data: Signal<Data<X, Y>>,
}

#[derive(Clone, Debug)]
pub struct Data<X: 'static, Y: 'static> {
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

                let (x_points, x_positions) = (data.into_iter())
                    .map(|t| {
                        let x = (get_x)(t);
                        let pos = x.position();
                        (x, pos)
                    })
                    .unzip();

                let (y_points, y_positions) = (get_ys.into_iter())
                    .flat_map(|get_y| {
                        data.into_iter().map(move |t| {
                            let y = get_y(t);
                            let pos = y.position();
                            (y, pos)
                        })
                    })
                    .unzip();

                Data {
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
