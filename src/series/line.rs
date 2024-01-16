use super::{ApplyUseSeries, IntoUseLine, UseSeries};
use crate::{
    bounds::Bounds, colours::Colour, debug::DebugRect, series::GetYValue, state::State, Font,
};
use leptos::*;
use std::rc::Rc;

pub struct Line<T, Y> {
    get_y: Rc<dyn GetYValue<T, Y>>,
    name: MaybeSignal<String>,
    colour: MaybeSignal<Option<Colour>>,
    width: MaybeSignal<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UseLine {
    pub id: usize,
    pub name: MaybeSignal<String>,
    pub colour: Signal<Colour>,
    width: MaybeSignal<f64>,
}

impl<T, Y> Line<T, Y> {
    pub fn new(get_y: impl Fn(&T) -> Y + 'static) -> Self {
        Self {
            get_y: Rc::new(get_y),
            name: MaybeSignal::default(),
            colour: MaybeSignal::default(),
            width: 1.0.into(),
        }
    }

    pub fn set_name(mut self, name: impl Into<MaybeSignal<String>>) -> Self {
        self.name = name.into();
        self
    }

    pub fn set_colour(mut self, colour: impl Into<MaybeSignal<Option<Colour>>>) -> Self {
        self.colour = colour.into();
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
            colour: self.colour,
            width: self.width,
        }
    }
}

impl<T, Y, F: Fn(&T) -> Y + 'static> From<F> for Line<T, Y> {
    fn from(f: F) -> Self {
        Self::new(f)
    }
}

impl<T, Y, U: Fn(&T) -> Y> GetYValue<T, Y> for U {
    fn value(&self, t: &T) -> Y {
        self(t)
    }

    fn cumulative_value(&self, t: &T) -> Y {
        self(t)
    }
}

impl<T, X, Y> ApplyUseSeries<T, X, Y> for Line<T, Y> {
    fn apply_use_series(self: Rc<Self>, series: &mut UseSeries<T, X, Y>) {
        let colour = series.next_colour();
        _ = series.push(colour, (*self).clone());
    }
}

impl<T, Y> IntoUseLine<T, Y> for Line<T, Y> {
    fn into_use_line(self, id: usize, colour: Memo<Colour>) -> (UseLine, Rc<dyn GetYValue<T, Y>>) {
        let override_colour = self.colour;
        let colour = Signal::derive(move || override_colour.get().unwrap_or(colour.get()));
        let line = UseLine {
            id,
            name: self.name.clone(),
            colour,
            width: self.width,
        };
        (line, self.get_y.clone())
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

    pub(crate) fn render(&self, positions: Signal<Vec<(f64, f64)>>) -> View {
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

#[component]
pub fn Snippet<X: 'static, Y: 'static>(series: UseLine, state: State<X, Y>) -> impl IntoView {
    let debug = state.pre.debug;
    let name = series.name.clone();
    view! {
        <div class="_chartistry_snippet" style="white-space: nowrap;">
            <DebugRect label="snippet" debug=debug />
            <Taster series=series state=state />
            {name}
        </div>
    }
}

#[component]
pub fn Taster<X: 'static, Y: 'static>(series: UseLine, state: State<X, Y>) -> impl IntoView {
    let debug = state.pre.debug;
    let font = state.pre.font;
    let bounds = UseLine::taster_bounds(font);
    view! {
        <svg
            class="_chartistry_taster"
            width=move || bounds.get().width()
            height=move || bounds.get().height()
            viewBox=move || format!("0 0 {} {}", bounds.get().width(), bounds.get().height())
            style:padding-right=move || format!("{}px", font.get().width())>
            <DebugRect label="taster" debug=debug bounds=vec![bounds.into()] />
            {series.taster(bounds)}
        </svg>
    }
}
