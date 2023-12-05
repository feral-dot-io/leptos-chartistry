use super::line::UseLine;
use crate::{colours::Colour, debug::DebugRect, state::State, UseData};
use leptos::*;
use std::rc::Rc;

pub type GetY<T, Y> = Rc<dyn Fn(&T) -> Y>;

pub trait IntoUseLine<T, X, Y> {
    fn into_use_line(self: Rc<Self>, id: usize, colour: Colour) -> (GetY<T, Y>, UseLine);
}

#[component]
pub fn RenderSeriesData<'a, X: Clone + 'static, Y: Clone + 'static>(
    data: UseData<X, Y>,
    state: &'a State<X, Y>,
) -> impl IntoView {
    let proj = state.projection;
    let pos_x = data.positions_x;
    let svg_coords = data
        .positions_y
        .iter()
        .map(|&pos_y| {
            Signal::derive(move || {
                let proj = proj.get();
                with!(|pos_x, pos_y| {
                    pos_x
                        .iter()
                        .zip(pos_y.iter())
                        .map(|(x, y)| proj.position_to_svg(*x, *y))
                        .collect::<Vec<_>>()
                })
            })
        })
        .collect::<Vec<_>>();

    let render = {
        let state = state.clone();
        move |series: UseLine| {
            let positions = svg_coords[series.id];
            series.render(positions, &state)
        }
    };

    view! {
        <g class="_chartistry_series">
            <For
                each=move || data.series.get()
                key=|series| series.id
                children=render
            />
        </g>
    }
}

#[component]
pub fn Snippet<'a, X: 'static, Y: 'static>(
    series: UseLine,
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
    series: UseLine,
    state: &'a State<X, Y>,
) -> impl IntoView {
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
            {series.taster(bounds, state)}
        </svg>
    }
}
