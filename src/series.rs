use crate::{projection::Projection, Line};
use leptos::*;
use std::borrow::Borrow;

pub struct SeriesBuilder<T: 'static, X: 'static, Y: 'static> {
    get_x: &'static dyn Fn(&T) -> X,
    get_ys: Vec<&'static dyn Fn(&T) -> Y>,
    lines: Vec<Line>,
}

#[derive(Clone, Debug)]
pub struct Series<X: 'static, Y: 'static> {
    lines: Vec<Line>,
    x_points: Vec<X>,
    x_positions: Vec<f64>,
    // Capacity: no. of series * x_points.len() == y_points.len()
    // Indexed by all points from first series, then second series, etc.
    y_points: Vec<Y>,
    y_positions: Vec<f64>,
}

impl<X, Y> Series<X, Y> {
    pub fn new<T>(get_x: &'static dyn Fn(&T) -> X) -> SeriesBuilder<T, X, Y> {
        SeriesBuilder {
            get_x,
            get_ys: Vec::new(),
            lines: Vec::new(),
        }
    }
}

impl<T, X, Y> SeriesBuilder<T, X, Y> {
    pub fn add(mut self, line: Line, get_y: &'static dyn Fn(&T) -> Y) -> Self {
        self.get_ys.push(get_y);
        self.lines.push(line);
        self
    }

    pub fn with_data<Ts>(self, data: impl Into<MaybeSignal<Ts>> + 'static) -> Signal<Series<X, Y>>
    where
        Ts: Borrow<[T]> + 'static,
        T: 'static,
        X: PartialOrd + Position,
        Y: PartialOrd + Position,
    {
        let data = data.into();
        Signal::derive(move || {
            let SeriesBuilder {
                get_x,
                get_ys,
                lines,
            } = &self;
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

                Series {
                    lines: lines.clone(),
                    x_points,
                    x_positions,
                    y_points,
                    y_positions,
                }
            })
        })
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
pub fn Series<X: 'static, Y: 'static>(
    series: Signal<Series<X, Y>>,
    projection: Signal<Projection>,
) -> impl IntoView {
    let lines = move || {
        let proj = projection.get();
        series.with(|series| {
            let points = series.x_points.len();
            (series.lines.iter())
                .enumerate()
                .map(|(line_i, line)| {
                    let positions = (series.x_positions.iter())
                        .enumerate()
                        .map(|(i, &x)| {
                            let y = series.y_positions[line_i * points + i];
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
