use super::use_series::{NextSeries, PrepareSeries, RenderSeries, ToUseLine};
use crate::{bounds::Bounds, colours::Colour, series::GetY, state::State, Font};
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

impl<T: 'static, X, Y: 'static> PrepareSeries<T, X, Y> for Line<T, Y> {
    fn prepare(self: Rc<Self>, acc: &mut NextSeries<T, Y>) -> Rc<dyn RenderSeries<X, Y>> {
        let (_, line) = acc.add_line(&*self);
        Rc::new(line)
    }
}

impl<T, Y> ToUseLine<T, Y> for Line<T, Y> {
    fn to_use_line(&self, id: usize, colour: Colour) -> (GetY<T, Y>, UseLine) {
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

    pub fn taster<X, Y>(&self, bounds: Memo<Bounds>, _: &State<X, Y>) -> View {
        view!( <LineTaster line=self bounds=bounds /> )
    }
}

impl<X, Y> RenderSeries<X, Y> for UseLine {
    fn render(self: Rc<Self>, positions: Vec<Signal<Vec<(f64, f64)>>>, _: &State<X, Y>) -> View {
        view!( <RenderLine line=&self positions=positions[self.id]  /> )
    }
}

#[component]
fn LineTaster<'a>(line: &'a UseLine, bounds: Memo<Bounds>) -> impl IntoView {
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
pub fn RenderLine<'a>(line: &'a UseLine, positions: Signal<Vec<(f64, f64)>>) -> impl IntoView {
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
