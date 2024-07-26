use super::{ApplyUseSeries, GetYValue, IntoUseBar, SeriesAcc, UseY};
use crate::{state::State, Colour, Tick};
use leptos::prelude::*;
use std::rc::Rc;

/// Default gap ratio between bars.
pub const BAR_GAP: f64 = 0.1;
/// Default gap ratio inside a group of bars.
pub const BAR_GAP_INNER: f64 = 0.05;

/// Draws a bar on the chart.
///
/// # Example
/// Creating a bar chart is simple. Add it to a series and pass it to a chart:
/// ```rust
/// # use leptos_chartistry::*;
/// # struct MyData { x: f64, y1: f64, y2: f64 }
/// let series = Series::new(|data: &MyData| data.x)
///     .bar(|data: &MyData| data.y1)
///     .bar(|data: &MyData| data.y2);
/// ```
/// See this in action with a [full bar chart example](https://feral-dot-io.github.io/leptos-chartistry/examples.html#bar-chart).
#[non_exhaustive]
pub struct Bar<T, Y> {
    get_y: Rc<dyn GetYValue<T, Y>>,
    /// Set the name of the bar as used in the legend and tooltip.
    pub name: RwSignal<String>,
    /// Set the colour of the bar. If not set, the next colour in the series will be used. Default is `None`.
    pub colour: RwSignal<Option<Colour>>,
    /// Sets where the bar's bottom is placed. Defaults to the zero line.
    pub placement: RwSignal<BarPlacement>,
    /// Set the gap between group bars. Clamped to 0.0 and 1.0. Defaults to 0.1.
    ///
    /// The gap is the ratio of the available width for an X value. For example if the chart has a width of 200px and 5 items (`T`) that leaves 40px per item. So a gap of 0.1 (10%) would leave 4px between each item with 2px on either side.
    pub gap: RwSignal<f64>,
    /// Set the gap inside a group of bars. Clamped to 0.0 and 1.0. Defaults to 0.05.
    ///
    /// The group gap is the ratio of the available width for a single bar in a group of bars (for a single X value). Carrying on the example from [gap](Self::gap) a group gap of 0.05 (5%) and two bars would result in 1px (40 / 2 * 0.05). This group gap becomes the space inbetween each bar.
    pub group_gap: RwSignal<f64>,
}

/// The location of where the bar extends from.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum BarPlacement {
    /// The bar extends from the zero line.
    #[default]
    Zero,
    /// The bar extends from the edge of the chart.
    Edge,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UseBar {
    group_id: usize,
    colour: Signal<Colour>,
    placement: RwSignal<BarPlacement>,
    gap: RwSignal<f64>,
    group_gap: RwSignal<f64>,
}

impl<T, Y> Bar<T, Y> {
    /// Create a new bar. Use `get_y` to extract the Y value from your struct.
    ///
    /// See the module documentation for examples.
    pub fn new(get_y: impl Fn(&T) -> Y + 'static) -> Self
    where
        Y: Tick,
    {
        Self {
            get_y: Rc::new(get_y),
            name: RwSignal::default(),
            colour: RwSignal::default(),
            placement: RwSignal::default(),
            gap: create_rw_signal(BAR_GAP),
            group_gap: create_rw_signal(BAR_GAP_INNER),
        }
    }

    /// Set the name of the bar. Used in the legend.
    pub fn with_name(self, name: impl Into<String>) -> Self {
        self.name.set(name.into());
        self
    }

    /// Set the colour of the bar. If not set, the next colour in the series will be used.
    pub fn with_colour(self, colour: impl Into<Option<Colour>>) -> Self {
        self.colour.set(colour.into());
        self
    }

    /// Set the placement of the bar.
    pub fn with_placement(self, placement: impl Into<BarPlacement>) -> Self {
        self.placement.set(placement.into());
        self
    }

    /// Set the gap between a group of bars. Clamped to 0.0 and 1.0. Defaults to 0.1.
    pub fn with_gap(self, gap: f64) -> Self {
        self.gap.set(gap);
        self
    }

    /// Set the gap inside a group of bars. Clamped to 0.0 and 1.0. Defaults to 0.05.
    pub fn with_group_gap(self, group_gap: f64) -> Self {
        self.group_gap.set(group_gap);
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

impl<T, Y: Tick, F: Fn(&T) -> Y + 'static> From<F> for Bar<T, Y> {
    fn from(f: F) -> Self {
        Self::new(f)
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
                colour,
                placement: self.placement,
                gap: self.gap,
                group_gap: self.group_gap,
            },
        );
        (bar, self.get_y.clone())
    }
}

#[component]
pub fn RenderBar<X: 'static, Y: 'static>(
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

            // Find width of each X position
            // Note: this should possibly be on Layout
            let gap = bar.gap.get().clamp(0.0, 1.0);
            let width = state.layout.x_width.get() * (1.0 - gap);
            // Find width of each group in an X position
            let group_gap = bar.group_gap.get().clamp(0.0, 1.0);
            let group_width = width / bars.get() as f64;
            let group_width_inner = group_width * (1.0 - group_gap);
            let group_gap = group_width * group_gap;

            let offset = group_gap / 2.0 - width / 2.0;
            positions
                .iter()
                .map(|&(x, y)| {
                    view! {
                        <rect
                            x=x + group_width * bar.group_id as f64 + offset
                            y=y
                            width=group_width_inner
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
