use crate::{colours::Colour, series::GetY};
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
    pub id: usize,
    pub name: MaybeSignal<String>,
    pub colour: MaybeSignal<Colour>,
    pub width: MaybeSignal<f64>,
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

    pub(super) fn use_line(self, id: usize, colour: Colour) -> (GetY<T, Y>, UseLine) {
        let line = UseLine {
            id,
            name: self.name,
            colour: self.colour.unwrap_or_else(|| colour.into()),
            width: self.width,
        };
        (self.get_y, line)
    }
}

#[component]
pub fn Line<'a>(line: &'a UseLine, positions: Vec<(f64, f64)>) -> impl IntoView {
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
