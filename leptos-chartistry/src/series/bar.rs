use super::{ApplyUseSeries, GetYValue, IntoUseBar, SeriesAcc, UseY};
use crate::{state::State, Colour, Tick};
use leptos::*;
use std::rc::Rc;

pub const BAR_GAP: f64 = 0.1;
pub const BAR_GAP_INNER: f64 = 0.05;

pub struct Bar<T, Y> {
    get_y: Rc<dyn GetYValue<T, Y>>,
    pub name: RwSignal<String>,
    pub placement: RwSignal<BarPlacement>,
    pub gap: RwSignal<f64>,
    pub group_gap: RwSignal<f64>,
    pub colour: RwSignal<Option<Colour>>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum BarPlacement {
    #[default]
    Zero,
    Edge,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UseBar {
    group_id: usize,
    placement: RwSignal<BarPlacement>,
    gap: RwSignal<f64>,
    group_gap: RwSignal<f64>,
    colour: Signal<Colour>,
}

impl<T, Y> Bar<T, Y> {
    pub fn new(get_y: impl Fn(&T) -> Y + 'static) -> Self
    where
        Y: Tick,
    {
        Self {
            get_y: Rc::new(get_y),
            placement: RwSignal::default(),
            gap: create_rw_signal(BAR_GAP),
            group_gap: create_rw_signal(BAR_GAP_INNER),
            name: RwSignal::default(),
            colour: RwSignal::default(),
        }
    }

    pub fn with_placement(mut self, placement: BarPlacement) -> Self {
        self.placement.set(placement);
        self
    }
}

impl<T, Y> Clone for Bar<T, Y> {
    fn clone(&self) -> Self {
        Self {
            get_y: self.get_y.clone(),
            placement: self.placement,
            gap: self.gap,
            group_gap: self.group_gap,
            name: self.name,
            colour: self.colour,
        }
    }
}

impl<T, Y> ApplyUseSeries<T, Y> for Bar<T, Y> {
    fn apply_use_series(self: Rc<Self>, series: &mut SeriesAcc<T, Y>) {
        let colour = series.next_colour();
        _ = series.push_bar(colour, (*self).clone());
    }
}

impl<T, Y> IntoUseBar<T, Y> for Bar<T, Y> {
    fn into_use_bar(
        self,
        id: usize,
        group_id: usize,
        colour: Memo<Colour>,
    ) -> (UseY, Rc<dyn GetYValue<T, Y>>) {
        let override_colour = self.colour;
        let colour = Signal::derive(move || override_colour.get().unwrap_or(colour.get()));
        let bar = UseY::new_bar(
            id,
            self.name,
            UseBar {
                group_id,
                placement: self.placement,
                gap: self.gap,
                group_gap: self.group_gap,
                colour,
            },
        );
        (bar, self.get_y.clone())
    }
}

#[component]
pub fn RenderBar<X: 'static, Y: 'static>(
    use_y: UseY,
    bar: UseBar,
    state: State<X, Y>,
    positions: Signal<Vec<(f64, f64)>>,
) -> impl IntoView {
    let bars = create_memo(move |_| {
        state
            .pre
            .data
            .series
            .get()
            .iter()
            .filter_map(|series| series.bar().map(|bar| (series.clone(), bar.clone())))
            .collect::<Vec<_>>()
            .len()
    });

    let rects = move || {
        positions.with(|positions| {
            // Find the bottom Y position of each bar
            let bottom_y = match bar.placement.get() {
                BarPlacement::Zero => state.svg_zero.get().1,
                BarPlacement::Edge => state.layout.inner.get().bottom_y(),
            };

            let gap = bar.gap.get().clamp(0.0, 1.0);
            let width = state.layout.inner.get().width() / positions.len() as f64 * (1.0 - gap);
            let gap = width * gap;

            let group_width = width / bars.get() as f64;
            let group_gap = width * bar.group_gap.get().clamp(0.0, 1.0);

            let offset = gap / 2.0 + group_gap / 2.0;
            positions
                .iter()
                .map(|&(x, y)| {
                    view! {
                        <rect
                            x=x + offset + group_width * bar.group_id as f64
                            y=y
                            width=group_width - group_gap
                            height=bottom_y - y />
                    }
                })
                .collect::<Vec<_>>()
        })
    };
    view! {
        <g
            class="_chartistry_bar"
            fill=move || bar.colour.get().to_string()>
            {rects}
        </g>
    }
}
