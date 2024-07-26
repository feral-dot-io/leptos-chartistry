use super::{
    bar::{RenderBar, UseBar},
    line::{RenderLine, UseLine},
};
use crate::{bounds::Bounds, debug::DebugRect, state::State};
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct UseY {
    pub id: usize,
    pub name: RwSignal<String>,
    desc: UseYDesc,
}

#[derive(Clone, Debug, PartialEq)]
enum UseYDesc {
    Line(UseLine),
    Bar(UseBar),
}

impl UseY {
    pub(super) fn new_line(id: usize, name: RwSignal<String>, line: UseLine) -> Self {
        let desc = UseYDesc::Line(line);
        Self { id, name, desc }
    }

    pub(super) fn new_bar(id: usize, name: RwSignal<String>, bar: UseBar) -> Self {
        let desc = UseYDesc::Bar(bar);
        Self { id, name, desc }
    }

    pub(crate) fn bar(&self) -> Option<&UseBar> {
        match &self.desc {
            UseYDesc::Bar(bar) => Some(bar),
            _ => None,
        }
    }

    fn taster_bounds(font_height: Memo<f64>, font_width: Memo<f64>) -> Memo<Bounds> {
        create_memo(move |_| Bounds::new(font_width.get() * 2.5, font_height.get()))
    }

    pub fn snippet_width(font_height: Memo<f64>, font_width: Memo<f64>) -> Signal<f64> {
        let taster_bounds = Self::taster_bounds(font_height, font_width);
        Signal::derive(move || taster_bounds.get().width() + font_width.get())
    }
}

#[component]
pub(super) fn RenderUseY<X: 'static, Y: 'static>(
    use_y: UseY,
    state: State<X, Y>,
    positions: Signal<Vec<(f64, f64)>>,
) -> impl IntoView {
    let desc = use_y.desc.clone();
    match desc {
        UseYDesc::Line(line) => view! {
            <RenderLine
                use_y=use_y
                line=line
                data=state.pre.data
                positions=positions
                markers=positions />
        },
        UseYDesc::Bar(bar) => view! {
            <RenderBar bar=bar state=state positions=positions />
        },
    }
}

#[component]
pub fn Snippet<X: 'static, Y: 'static>(series: UseY, state: State<X, Y>) -> impl IntoView {
    let debug = state.pre.debug;
    let name = series.name;
    view! {
        <div class="_chartistry_snippet" style="white-space: nowrap;">
            <DebugRect label="snippet" debug=debug />
            <Taster series=series state=state />
            {name}
        </div>
    }
}

#[component]
fn Taster<X: 'static, Y: 'static>(series: UseY, state: State<X, Y>) -> impl IntoView {
    const Y_OFFSET: f64 = 2.0;
    let debug = state.pre.debug;
    let font_width = state.pre.font_width;
    let right_padding = Signal::derive(move || font_width.get() / 2.0);
    let bounds = UseY::taster_bounds(state.pre.font_height, font_width);
    // Mock positions from left to right of our bounds
    let positions = Signal::derive(move || {
        let bounds = bounds.get();
        let y = bounds.centre_y() + Y_OFFSET;
        vec![(bounds.left_x(), y), (bounds.right_x(), y)]
    });

    let desc = match &series.desc {
        UseYDesc::Line(line) => {
            // One marker in the middle
            let markers = Signal::derive(move || {
                let bounds = bounds.get();
                vec![(bounds.centre_x(), bounds.centre_y() + Y_OFFSET)]
            });
            view! {
                <RenderLine
                    use_y=series.clone()
                    line=line.clone()
                    data=state.pre.data
                    positions=positions
                    markers=markers />
            }
        }
        UseYDesc::Bar(bar) => view! {
            <RenderBar bar=bar.clone() state=state positions=positions />
        },
    };

    view! {
        <svg
            viewBox=move || format!("0 0 {} {}", bounds.get().width(), bounds.get().height())
            width=move || bounds.get().width() + right_padding.get()
            height=move || bounds.get().height()
            class="_chartistry_taster"
            style="box-sizing: border-box;"
            style:padding-right=move || format!("{}px", right_padding.get())
            >
            <DebugRect label="taster" debug=debug bounds=vec![bounds.into()] />
            {desc}
        </svg>
    }
}
