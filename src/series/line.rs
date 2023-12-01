use super::{IntoSeries, Series, UseSeries};
use crate::{colours::Colour, series::GetY, state::State};
use leptos::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct Line<T, Y> {
    get_y: GetY<T, Y>,
    name: MaybeSignal<String>,
    colour: Option<MaybeSignal<Colour>>,
    width: MaybeSignal<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UseLine {
    id: usize,
    name: MaybeSignal<String>,
    colour: MaybeSignal<Colour>,
    width: MaybeSignal<f64>,
}

impl<T, Y> Line<T, Y> {
    pub fn new(get_y: impl Fn(&T) -> Y + 'static) -> Self {
        Self {
            get_y: Rc::new(get_y),
            name: MaybeSignal::default(),
            colour: None,
            width: 1.0.into(),
        }
    }

    pub fn set_name(mut self, name: impl Into<MaybeSignal<String>>) -> Self {
        self.name = name.into();
        self
    }

    pub fn set_colour(mut self, colour: impl Into<MaybeSignal<Colour>>) -> Self {
        self.colour = Some(colour.into());
        self
    }

    pub fn set_width(mut self, width: impl Into<MaybeSignal<f64>>) -> Self {
        self.width = width.into();
        self
    }
}

impl<T, X, Y> IntoSeries<T, X, Y> for Line<T, Y> {
    fn into_use(
        self: Rc<Self>,
        id: usize,
        colour: Colour,
    ) -> (GetY<T, Y>, Rc<dyn UseSeries<X, Y>>) {
        let line = UseLine {
            id,
            name: self.name.clone(),
            colour: self.colour.unwrap_or_else(|| colour.into()),
            width: self.width,
        };
        (self.get_y.clone(), Rc::new(line))
    }
}

impl<X, Y> UseSeries<X, Y> for UseLine {
    fn describe(&self) -> Series {
        Series {
            id: self.id,
            name: self.name.clone(),
            colour: self.colour,
        }
    }

    fn render(&self, positions: Vec<(f64, f64)>, _state: &State<X, Y>) -> View {
        view!( <RenderLine line=self positions=positions /> )
    }
}

#[component]
pub fn RenderLine<'a>(line: &'a UseLine, positions: Vec<(f64, f64)>) -> impl IntoView {
    let mut need_move = true;
    let path = positions
        .into_iter()
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
        .collect::<String>();
    let colour = line.colour;
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
