use super::{UseLayout, UseVerticalLayout};
use crate::{
    bounds::Bounds,
    debug::DebugRect,
    edge::Edge,
    state::{PreState, State},
    ticks::{
        AlignedFloats, GeneratedTicks, HorizontalSpan, TickFormat, TickFormatFn, TickGen,
        Timestamps, VerticalSpan,
    },
    Tick,
};
use chrono::prelude::*;
use leptos::prelude::*;
use std::rc::Rc;

/// Builds tick labels for an axis.
///
/// Note that ticks lack an identity resulting in generators and labels not being reactive.
pub struct TickLabels<Tick: 'static> {
    /// Minimum number of characters to display for each tick label.
    ///
    /// Helpful for giving a fixed width to labels e.g., if your graph can display 0-100 then it might show a shorter label on "0" or "42" to "100". Needed to have the same inner chart ratio when using an outer chart ratio. Can also be useful for aligning a list of charts.
    pub min_chars: RwSignal<usize>,
    /// Format function for the tick labels. See [TickLabels::with_format] for details.
    pub format: RwSignal<Rc<TickFormatFn<Tick>>>,
    /// Tick generator for the labels.
    pub generator: RwSignal<Rc<dyn TickGen<Tick = Tick>>>,
}

#[derive(Clone)]
pub struct UseTickLabels {
    ticks: Signal<Vec<(f64, String)>>,
}

impl<Tick> Clone for TickLabels<Tick> {
    fn clone(&self) -> Self {
        Self {
            min_chars: self.min_chars,
            format: self.format,
            generator: self.generator,
        }
    }
}

impl<Tick: crate::Tick> Default for TickLabels<Tick> {
    fn default() -> Self {
        Self::from_generator(Tick::tick_label_generator())
    }
}

impl TickLabels<f64> {
    /// Creates a new tick label generator for floating point numbers. See [AlignedFloats] for details.
    pub fn aligned_floats() -> Self {
        Self::from_generator(AlignedFloats::default())
    }
}

impl<Tz> TickLabels<DateTime<Tz>>
where
    Tz: TimeZone + 'static,
    Tz::Offset: std::fmt::Display,
{
    /// Creates a new tick label generator for timestamps. See [Timestamps] for details.
    pub fn timestamps() -> Self {
        Self::from_generator(Timestamps::default())
    }
}

impl<Tick: crate::Tick> TickLabels<Tick> {
    /// Creates a new tick label generator from a tick generator.
    pub fn from_generator(gen: impl TickGen<Tick = Tick> + 'static) -> Self {
        Self {
            min_chars: RwSignal::default(),
            format: RwSignal::new(HorizontalSpan::identity_format()),
            generator: create_rw_signal(Rc::new(gen)),
        }
    }

    /// Sets the minimum number of characters to display for each tick label.
    pub fn with_min_chars(self, min_chars: usize) -> Self {
        self.min_chars.set(min_chars);
        self
    }

    /// Sets the format function for the tick labels.
    ///
    /// This is a function that takes a `Tick` and a formatter and returns a `String`. It gives an opportunity to customise tick label format. The formatter is the resulting state of the tick generator and does the default aciton. For example if aligned floats decides to use "1000s" then the formatter will use that.
    pub fn with_format(
        self,
        format: impl Fn(&Tick, &dyn TickFormat<Tick = Tick>) -> String + 'static,
    ) -> Self {
        self.format.set(Rc::new(format));
        self
    }

    fn map_ticks(&self, gen: Signal<GeneratedTicks<Tick>>) -> Signal<Vec<(f64, String)>> {
        let format = self.format;
        Signal::derive(move || {
            let format = format.get();
            gen.with(|GeneratedTicks { ticks, state }| {
                ticks
                    .iter()
                    .map(|tick| (tick.position(), (format)(tick, state.as_ref())))
                    .collect()
            })
        })
    }
}

impl<Gen, Tick> From<Gen> for TickLabels<Tick>
where
    Gen: TickGen<Tick = Tick> + 'static,
    Tick: crate::Tick,
{
    fn from(gen: Gen) -> Self {
        Self::from_generator(gen)
    }
}

impl<X: Tick> TickLabels<X> {
    pub(crate) fn generate_x<Y>(
        &self,
        state: &PreState<X, Y>,
        avail_width: Signal<f64>,
    ) -> Signal<GeneratedTicks<X>> {
        let font_width = state.font_width;
        let padding = state.padding;
        let range_x = state.data.range_x;
        let TickLabels {
            min_chars,
            format,
            generator,
        } = self.clone();
        create_memo(move |_| {
            range_x
                .get()
                .range()
                .map(|(first, last)| {
                    let span = HorizontalSpan::new(
                        font_width.get(),
                        min_chars.get(),
                        padding.get().width(),
                        avail_width.get(),
                        format.get(),
                    );
                    generator.get().generate(first, last, &span)
                })
                .unwrap_or_else(GeneratedTicks::none)
        })
        .into()
    }

    pub(super) fn fixed_height<Y>(&self, state: &PreState<X, Y>) -> Signal<f64> {
        let font_height = state.font_height;
        let padding = state.padding;
        Signal::derive(move || font_height.get() + padding.get().height())
    }

    pub(super) fn to_horizontal_use<Y>(
        &self,
        state: &PreState<X, Y>,
        avail_width: Memo<f64>,
    ) -> UseLayout {
        UseLayout::TickLabels(UseTickLabels {
            ticks: self.map_ticks(self.generate_x(state, avail_width.into())),
        })
    }
}

impl<Y: Tick> TickLabels<Y> {
    pub(crate) fn generate_y<X>(
        &self,
        state: &PreState<X, Y>,
        avail_height: Signal<f64>,
    ) -> Signal<GeneratedTicks<Y>> {
        let font_height = state.font_height;
        let padding = state.padding;
        let range_y = state.data.range_y;
        let generator = self.generator;
        create_memo(move |_| {
            range_y
                .get()
                .range()
                .map(|(first, last)| {
                    let span = VerticalSpan::new(
                        font_height.get() + padding.get().height(),
                        avail_height.get(),
                    );
                    generator.get().generate(first, last, &span)
                })
                .unwrap_or_else(GeneratedTicks::none)
        })
        .into()
    }

    pub(super) fn to_vertical_use<X>(
        &self,
        state: &PreState<X, Y>,
        avail_height: Memo<f64>,
    ) -> UseVerticalLayout {
        let ticks = self.map_ticks(self.generate_y(state, avail_height.into()));
        UseVerticalLayout {
            width: mk_width(self.min_chars, state, ticks),
            layout: UseLayout::TickLabels(UseTickLabels { ticks }),
        }
    }
}

fn mk_width<X, Y>(
    min_chars: RwSignal<usize>,
    state: &PreState<X, Y>,
    ticks: Signal<Vec<(f64, String)>>,
) -> Signal<f64> {
    let font_width = state.font_width;
    let padding = state.padding;
    Signal::derive(move || {
        let longest_chars = ticks.with(|ticks| {
            ticks
                .iter()
                .map(|(_, label)| label.len())
                .max()
                .unwrap_or_default()
                .max(min_chars.get())
        }) as f64;
        font_width.get() * longest_chars + padding.get().width()
    })
}

fn align_tick_labels(labels: Vec<String>) -> Vec<String> {
    // Find longest label length
    let min_label = labels
        .iter()
        .map(|label| label.len())
        .max()
        .unwrap_or_default();
    // Pad labels to same length
    labels
        .into_iter()
        .map(|mut label| {
            let spaces = " ".repeat(min_label.saturating_sub(label.len()));
            label.insert_str(0, &spaces);
            label
        })
        .collect::<Vec<_>>()
}

#[component]
pub(super) fn TickLabels<X: Clone + 'static, Y: Clone + 'static>(
    ticks: UseTickLabels,
    edge: Edge,
    bounds: Memo<Bounds>,
    state: State<X, Y>,
) -> impl IntoView {
    let ticks = move || {
        // Align vertical labels
        let ticks = ticks.ticks.get();
        let ticks = if edge.is_vertical() {
            let (pos, labels): (Vec<f64>, Vec<String>) = ticks.into_iter().unzip();
            let labels = align_tick_labels(labels);
            pos.into_iter().zip(labels).collect::<Vec<_>>()
        } else {
            ticks
        };
        ticks
            .into_iter()
            .map(|tick| {
                view! {
                    <TickLabel edge=edge outer=bounds state=state.clone() tick=tick />
                }
            })
            .collect_view()
    };
    view! {
        <g class="_chartistry_tick_labels">
            {ticks}
        </g>
    }
}

#[component]
fn TickLabel<X: 'static, Y: 'static>(
    edge: Edge,
    outer: Memo<Bounds>,
    state: State<X, Y>,
    tick: (f64, String),
) -> impl IntoView {
    let debug = state.pre.debug;
    let font_height = state.pre.font_height;
    let font_width = state.pre.font_width;
    let padding = state.pre.padding;
    let projection = state.projection;

    let (position, label) = tick;
    let label_len = label.len();
    // Calculate positioning Bounds. Note: tick w / h includes padding
    let bounds = Signal::derive(move || {
        let padding = padding.get();
        let width = font_width.get() * label_len as f64 + padding.width();
        let height = font_height.get() + padding.height();

        let proj = projection.get();
        let outer = outer.get();
        match edge {
            Edge::Top | Edge::Bottom => {
                let (x, _) = proj.position_to_svg(position, 0.0);
                let x = x - width / 2.0;
                Bounds::from_points(x, outer.top_y(), x + width, outer.bottom_y())
            }

            Edge::Left | Edge::Right => {
                let (_, y) = proj.position_to_svg(0.0, position);
                let y = y - height / 2.0;
                Bounds::from_points(outer.left_x(), y, outer.right_x(), y + height)
            }
        }
    });
    let content = create_memo(move |_| padding.get().apply(bounds.get()));

    // Determine text position
    let text_position = create_memo(move |_| {
        let content = content.get();
        match edge {
            Edge::Top | Edge::Bottom => ("middle", content.centre_x()),

            Edge::Left | Edge::Right => {
                let (x, anchor) = if edge == Edge::Left {
                    (content.right_x(), "end")
                } else {
                    (content.left_x(), "start")
                };
                (anchor, x)
            }
        }
    });

    view! {
        <g
            class="_chartistry_tick_label"
            font-family="monospace">
            <DebugRect label="tick" debug=debug bounds=vec![bounds, content.into()] />
            <text
                x=move || text_position.get().1
                y=move || content.get().centre_y()
                style="white-space: pre;"
                font-size=move || font_height.get()
                dominant-baseline="middle"
                text-anchor=move || text_position.get().0>
                {label.clone()}
            </text>
        </g>
    }
}
