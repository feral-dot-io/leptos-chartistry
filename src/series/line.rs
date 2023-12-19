use super::use_series::{NextSeries, PrepareSeries, ToUseLine};
use crate::{bounds::Bounds, colours::Colour, series::GetYValue, Font};
use leptos::*;
use std::rc::Rc;

pub struct Line<T, Y> {
    get_y: Rc<dyn GetYValue<T, Y>>,
    name: MaybeSignal<String>,
    width: MaybeSignal<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UseLine {
    pub id: usize,
    pub name: MaybeSignal<String>,
    pub colour: MaybeSignal<Colour>,
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

impl<T, Y> Clone for Line<T, Y> {
    fn clone(&self) -> Self {
        Self {
            get_y: self.get_y.clone(),
            name: self.name.clone(),
            width: self.width,
        }
    }
}

impl<T, Y, U: Fn(&T) -> Y> GetYValue<T, Y> for U {
    fn value(&self, t: &T) -> Y {
        self(t)
    }

    fn position(&self, t: &T) -> Y {
        self(t)
    }
}

impl<T: 'static, X, Y: 'static> PrepareSeries<T, X, Y> for Line<T, Y> {
    fn prepare(self: Rc<Self>, acc: &mut NextSeries<T, Y>) {
        acc.add_line(&*self);
    }
}

impl<T, Y> ToUseLine<T, Y> for Line<T, Y> {
    fn to_use_line(&self, id: usize, colour: Colour) -> (Rc<dyn GetYValue<T, Y>>, UseLine) {
        let line = UseLine {
            id,
            name: self.name.clone(),
            colour: colour.into(),
            width: self.width,
        };
        (self.get_y.clone(), line)
    }
}

impl UseLine {
    pub fn taster_bounds(font: Signal<Font>) -> Memo<Bounds> {
        create_memo(move |_| {
            let font = font.get();
            Bounds::new(font.width() * 2.0, font.height())
        })
    }

    pub fn snippet_width(font: Signal<Font>) -> Signal<f64> {
        let taster_bounds = Self::taster_bounds(font);
        Signal::derive(move || taster_bounds.get().width() + font.get().width())
    }

    pub fn taster(&self, bounds: Memo<Bounds>) -> View {
        view!( <LineTaster line=self.clone() bounds=bounds /> )
    }

    pub(super) fn render(&self, positions: Signal<Vec<(f64, f64)>>) -> View {
        view!( <RenderLine line=self.clone() positions=positions /> )
    }
}

#[component]
fn LineTaster(line: UseLine, bounds: Memo<Bounds>) -> impl IntoView {
    let colour = line.colour;
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
pub fn RenderLine(line: UseLine, positions: Signal<Vec<(f64, f64)>>) -> impl IntoView {
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
