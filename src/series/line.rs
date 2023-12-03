use super::{
    data::IntoSeries,
    use_series::{RenderSeries, UseSeries},
};
use crate::{bounds::Bounds, colours::Colour, series::GetY, state::State};
use leptos::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct Line<T, Y> {
    get_y: GetY<T, Y>,
    name: MaybeSignal<String>,
    width: MaybeSignal<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UseLine {
    width: MaybeSignal<f64>,
}

impl<T, Y> Line<T, Y> {
    pub fn new(get_y: impl Fn(&T) -> Y + 'static) -> Self {
        Self {
            get_y: Rc::new(get_y),
            name: MaybeSignal::default(),
            width: 1.0.into(),
        }
    }

    pub fn set_name(mut self, name: impl Into<MaybeSignal<String>>) -> Self {
        self.name = name.into();
        self
    }

    pub fn set_width(mut self, width: impl Into<MaybeSignal<f64>>) -> Self {
        self.width = width.into();
        self
    }
}

impl<T, X, Y> IntoSeries<T, X, Y> for Line<T, Y> {
    fn into_use(self: Rc<Self>, id: usize, colour: Colour) -> (GetY<T, Y>, UseSeries) {
        let line = RenderSeries::Line(UseLine { width: self.width });
        let series = UseSeries::new(id, self.name.clone(), colour, line);
        (self.get_y.clone(), series)
    }
}

impl UseLine {
    pub fn taster<X, Y>(&self, series: &UseSeries, bounds: Memo<Bounds>, _: &State<X, Y>) -> View {
        view!( <LineTaster line=self series=series bounds=bounds /> )
    }

    pub fn render<X, Y>(
        &self,
        series: &UseSeries,
        positions: Signal<Vec<(f64, f64)>>,
        _state: &State<X, Y>,
    ) -> View {
        view!( <RenderLine line=self series=series positions=positions /> )
    }
}

#[component]
pub fn LineTaster<'a>(
    series: &'a UseSeries,
    line: &'a UseLine,
    bounds: Memo<Bounds>,
) -> impl IntoView {
    let colour = series.colour;
    view! {
        <line
            x1=move || bounds.get().left_x()
            x2=move || bounds.get().right_x()
            y1=move || bounds.get().centre_y() + 1.0
            y2=move || bounds.get().centre_y() + 1.0
            stroke=move || colour.get().to_string()
            stroke-width=line.width
        />
    }
}

#[component]
fn RenderLine<'a>(
    series: &'a UseSeries,
    line: &'a UseLine,
    positions: Signal<Vec<(f64, f64)>>,
) -> impl IntoView {
    let path = move || {
        positions.with(|positions| {
            let mut need_move = true;
            positions
                .iter()
                .map(|(x, y)| {
                    if x.is_nan() || y.is_nan() {
                        need_move = true;
                        "".to_string()
                    } else if need_move {
                        need_move = false;
                        format!("M {} {} ", x, y)
                    } else {
                        format!("L {} {} ", x, y)
                    }
                })
                .collect::<String>()
        })
    };
    let colour = series.colour;
    view! {
        <g class="_chartistry_line">
            <path
                d=path
                fill="none"
                stroke=move || colour.get().to_string()
                stroke-width=line.width
            />
        </g>
    }
}
