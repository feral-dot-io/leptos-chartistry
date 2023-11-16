use crate::{
    bounds::Bounds,
    colours::{self, ColourScheme},
    line::{Line, UseLine},
    projection::Projection,
};
use chrono::prelude::*;
use leptos::*;
use std::borrow::Borrow;

type GetX<T, X> = Box<dyn Fn(&T) -> X>;
type GetY<T, Y> = Box<dyn Fn(&T) -> Y>;

pub struct Series<T: 'static, X: 'static, Y: 'static> {
    get_x: GetX<T, X>,
    get_ys: Vec<GetY<T, Y>>,
    lines: Vec<Line>,
    colours: ColourScheme,
    x_lower: Signal<Option<X>>,
    x_upper: Signal<Option<X>>,
    y_lower: Signal<Option<Y>>,
    y_upper: Signal<Option<Y>>,
}

#[derive(Clone, Debug)]
pub struct UseSeries<X: 'static, Y: 'static> {
    pub(crate) lines: Vec<UseLine>,
    pub(crate) data: Signal<Data<X, Y>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Data<X, Y> {
    position_range: Bounds,
    x_points: Vec<X>,
    x_positions: Vec<f64>,
    x_range: (X, X),
    y_points: Vec<Y>,
    y_positions: Vec<f64>,
    y_range: (Y, Y),
}

impl<T, X: Clone + PartialEq + 'static, Y: Clone + PartialEq + 'static> Series<T, X, Y> {
    pub fn new(get_x: impl Fn(&T) -> X + 'static) -> Self {
        Series {
            get_x: Box::new(get_x),
            get_ys: Vec::new(),
            lines: Vec::new(),
            colours: colours::ARBITRARY.as_ref().into(),
            x_lower: Signal::default(),
            x_upper: Signal::default(),
            y_lower: Signal::default(),
            y_upper: Signal::default(),
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
        self.x_lower = Signal::derive(move || lower.get().into());
        self
    }

    pub fn set_x_max<Opt>(mut self, upper: impl Into<MaybeSignal<Opt>>) -> Self
    where
        Opt: Clone + Into<Option<X>> + 'static,
    {
        let upper = upper.into();
        self.x_upper = Signal::derive(move || upper.get().into());
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
        self.y_lower = Signal::derive(move || lower.get().into());
        self
    }

    pub fn set_y_max<Opt>(mut self, upper: impl Into<MaybeSignal<Opt>>) -> Self
    where
        Opt: Clone + Into<Option<Y>> + 'static,
    {
        let upper = upper.into();
        self.y_upper = Signal::derive(move || upper.get().into());
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

    pub fn add_line(mut self, line: impl Into<Line>, get_y: impl Fn(&T) -> Y + 'static) -> Self {
        self.get_ys.push(Box::new(get_y));
        self.lines.push(line.into());
        self
    }

    pub fn use_data<Ts>(self, data: impl Into<MaybeSignal<Ts>> + 'static) -> UseSeries<X, Y>
    where
        Ts: Borrow<[T]> + 'static,
        X: Clone + PartialOrd + Position,
        Y: Clone + PartialOrd + Position,
    {
        let Series { get_x, get_ys, .. } = self;

        // Apply colours to lines
        let lines = self
            .lines
            .into_iter()
            .zip(self.colours.iter())
            .map(|(line, colour)| line.use_line(colour))
            .collect::<Vec<_>>();

        // Convert data to a signal
        let data = data.into();
        let data = create_memo(move |_| {
            let get_x = &get_x;
            let x_lower = self.x_lower.get();
            let x_upper = self.x_upper.get();
            let y_lower = self.y_lower.get();
            let y_upper = self.y_upper.get();
            let get_ys = get_ys.iter().as_slice();
            data.with(move |data| {
                let data = data.borrow();

                // Collect data points
                let x_points = data.iter().map(get_x).collect::<Vec<_>>();
                let x_positions = x_points.iter().map(|x| x.position()).collect::<Vec<_>>();
                let y_points = get_ys
                    .iter()
                    .flat_map(|get_y| data.iter().map(get_y))
                    .collect::<Vec<_>>();
                let y_positions = y_points.iter().map(|y| y.position()).collect::<Vec<_>>();

                // Find min/max
                let x_range_i = Self::find_min_max_index(&x_positions);
                let y_range_i = Self::find_min_max_index(&y_positions);
                let x_range = (get_x(&data[x_range_i.0]), get_x(&data[x_range_i.1]));
                let y_range = (
                    Self::reverse_get_y(get_ys, data, y_range_i.0),
                    Self::reverse_get_y(get_ys, data, y_range_i.1),
                );

                // Apply min/max range overrides
                let x_range = Self::apply_min_max_range(x_range, x_lower, x_upper);
                let y_range = Self::apply_min_max_range(y_range, y_lower, y_upper);

                Data {
                    position_range: Bounds::from_points(
                        x_range.0.position(),
                        y_range.0.position(),
                        x_range.1.position(),
                        y_range.1.position(),
                    ),
                    x_points,
                    x_positions,
                    x_range,
                    y_points,
                    y_positions,
                    y_range,
                }
            })
        })
        .into();

        UseSeries { lines, data }
    }

    fn apply_min_max_range<V: PartialOrd>(
        (min, max): (V, V),
        lower: Option<V>,
        upper: Option<V>,
    ) -> (V, V) {
        (
            match lower {
                Some(l) if l < min => l,
                _ => min,
            },
            match upper {
                Some(u) if u > max => u,
                _ => max,
            },
        )
    }

    fn find_min_max_index(positions: &[f64]) -> (usize, usize) {
        positions
            .iter()
            .enumerate()
            // TODO handle empty data
            .fold((0, 0), |(min_i, max_i), (i, &pos)| {
                (
                    if pos < positions[min_i] { i } else { min_i },
                    if pos > positions[max_i] { i } else { max_i },
                )
            })
    }

    /// Given an Data::y_points index, return the corresponding y value. Note that y_points is a flat map of all the y values for each series.
    fn reverse_get_y(get_ys: &[GetY<T, Y>], data: &[T], index: usize) -> Y {
        let series_i = index / data.len();
        let data_i = index % data.len();
        (get_ys[series_i])(&data[data_i])
    }
}

impl<X, Y> Data<X, Y> {
    pub fn position_range(&self) -> Bounds {
        self.position_range
    }

    pub fn x_range(&self) -> (&X, &X) {
        (&self.x_range.0, &self.x_range.1)
    }

    pub fn y_range(&self) -> (&Y, &Y) {
        (&self.y_range.0, &self.y_range.1)
    }

    fn nearest_x_index(&self, pos: f64) -> usize {
        // Find index after pos
        let index = self.x_positions.partition_point(|&v| v < pos);
        // No value before
        if index == 0 {
            return 0;
        }
        // No value ahead
        if index == self.x_points.len() {
            return index - 1;
        }
        // Find closest index
        let ahead = self.x_positions[index] - pos;
        let before = pos - self.x_positions[index - 1];
        if ahead < before {
            index
        } else {
            index - 1
        }
    }

    pub fn nearest_x(&self, x_pos: f64) -> &X {
        let x_index = self.nearest_x_index(x_pos);
        &self.x_points[x_index]
    }

    pub fn nearest_x_position(&self, x_pos: f64) -> f64 {
        let x_index = self.nearest_x_index(x_pos);
        self.x_positions[x_index]
    }

    pub fn nearest_y(&self, x_pos: f64, line_id: usize) -> &Y {
        let x_index = self.nearest_x_index(x_pos);
        &self.y_points[line_id * self.x_points.len() + x_index]
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
