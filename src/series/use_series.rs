use super::line::UseLine;
use crate::{bounds::Bounds, colours::Colour, debug::DebugRect, state::State, Font};
use leptos::*;

#[derive(Clone, Debug, PartialEq)]
pub struct UseSeries {
    pub id: usize,
    pub name: MaybeSignal<String>,
    pub colour: MaybeSignal<Colour>,
    render: RenderSeries,
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub(super) enum RenderSeries {
    Line(UseLine),
}

impl UseSeries {
    pub(super) fn new(
        id: usize,
        name: impl Into<MaybeSignal<String>>,
        colour: impl Into<MaybeSignal<Colour>>,
        render: RenderSeries,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            colour: colour.into(),
            render,
        }
    }

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

    pub fn taster<X, Y>(&self, bounds: Memo<Bounds>, state: &State<X, Y>) -> View {
        match &self.render {
            RenderSeries::Line(line) => line.taster(self, bounds, state),
        }
    }

    pub fn render<X, Y>(&self, positions: Signal<Vec<(f64, f64)>>, state: &State<X, Y>) -> View {
        match &self.render {
            RenderSeries::Line(line) => line.render(self, positions, state),
        }
    }
}

#[component]
pub fn Snippet<'a, X: 'static, Y: 'static>(
    series: UseSeries,
    state: &'a State<X, Y>,
) -> impl IntoView {
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
pub fn Taster<'a, X: 'static, Y: 'static>(
    series: UseSeries,
    state: &'a State<X, Y>,
) -> impl IntoView {
    let debug = state.pre.debug;
    let font = state.pre.font;
    let bounds = UseSeries::taster_bounds(font);
    view! {
        <svg
            class="_chartistry_taster"
            width=move || bounds.get().width()
            height=move || bounds.get().height()
            viewBox=move || format!("0 0 {} {}", bounds.get().width(), bounds.get().height())
            style:padding-right=move || format!("{}px", font.get().width())>
            <DebugRect label="taster" debug=debug bounds=vec![bounds.into()] />
            {series.taster(bounds, state)}
        </svg>
    }
}
