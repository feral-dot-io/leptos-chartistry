use super::{UseLayout, UseVerticalLayout};
use crate::{
    bounds::Bounds,
    debug::DebugRect,
    edge::Edge,
    state::{PreState, State},
    ticks::{
        AlignedFloats, GeneratedTicks, HorizontalSpan, PeriodicTimestamps, TickGen, VerticalSpan,
    },
    Period,
};
use chrono::prelude::*;
use leptos::*;
use std::rc::Rc;

pub struct TickLabels<Tick: 'static> {
    pub min_chars: RwSignal<usize>,
    pub generator: RwSignal<Rc<dyn TickGen<Tick = Tick>>>,
}

#[derive(Clone)]
pub struct UseTickLabels {
    ticks: Signal<Vec<(f64, String)>>,
}

impl<Tick: crate::Tick> Default for TickLabels<Tick> {
    fn default() -> Self {
        Self::from_generator(Tick::default_generator())
    }
}

impl TickLabels<f64> {
    pub fn aligned_floats() -> Self {
        Self::from_generator(AlignedFloats::default())
    }
}

impl<Tz> TickLabels<DateTime<Tz>>
where
    Tz: TimeZone + 'static,
    Tz::Offset: std::fmt::Display,
{
    pub fn timestamps() -> Self {
        Self::from_generator(PeriodicTimestamps::from_periods(Period::all()))
    }
}

impl<Tick> TickLabels<Tick> {
    pub fn new(gen: impl Into<RwSignal<Rc<dyn TickGen<Tick = Tick>>>>) -> Self {
        Self {
            min_chars: RwSignal::default(),
            generator: gen.into(),
        }
    }

    pub fn from_generator(gen: impl TickGen<Tick = Tick> + 'static) -> Self {
        let gen: Rc<dyn TickGen<Tick = Tick>> = Rc::new(gen);
        Self::new(gen)
    }

    pub fn set_generator(&self, gen: impl TickGen<Tick = Tick> + 'static) {
        self.generator.set(Rc::new(gen));
    }

    fn map_ticks(&self, gen: Signal<GeneratedTicks<Tick>>) -> Signal<Vec<(f64, String)>> {
        Signal::derive(move || {
            gen.with(|GeneratedTicks { ticks, state }| {
                ticks
                    .iter()
                    .map(|tick| (state.position(tick), state.format(tick)))
                    .collect()
            })
        })
    }
}

impl<Tick> Clone for TickLabels<Tick> {
    fn clone(&self) -> Self {
        Self {
            min_chars: self.min_chars,
            generator: self.generator,
        }
    }
}

impl<X: PartialEq> TickLabels<X> {
    pub fn generate_x<Y>(
        &self,
        state: &PreState<X, Y>,
        avail_width: Signal<f64>,
    ) -> Signal<GeneratedTicks<X>> {
        let PreState { font, padding, .. } = *state;
        let range_x = state.data.range_x;
        let min_chars = self.min_chars;
        let gen = self.generator;
        create_memo(move |_| {
            let min_chars = min_chars.get();
            range_x.with(|range_x| {
                range_x
                    .as_ref()
                    .map(|(first, last)| {
                        let font_width = font.get().width();
                        let padding_width = padding.get().width();
                        let span = HorizontalSpan::new(
                            font_width,
                            min_chars,
                            padding_width,
                            avail_width.get(),
                        );
                        gen.get().generate(first, last, Box::new(span))
                    })
                    .unwrap_or_else(GeneratedTicks::none)
            })
        })
        .into()
    }

    pub(super) fn fixed_height<Y>(&self, state: &PreState<X, Y>) -> Signal<f64> {
        let PreState { font, padding, .. } = *state;
        Signal::derive(move || with!(|font, padding| { font.height() + padding.height() }))
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

impl<Y: PartialEq> TickLabels<Y> {
    pub fn generate_y<X>(
        &self,
        state: &PreState<X, Y>,
        avail_height: Signal<f64>,
    ) -> Signal<GeneratedTicks<Y>> {
        let PreState { font, padding, .. } = *state;
        let range_y = state.data.range_y;
        let gen = self.generator;
        create_memo(move |_| {
            range_y.with(|range_y| {
                range_y
                    .as_ref()
                    .map(|(first, last)| {
                        let line_height = font.get().height() + padding.get().height();
                        let span = VerticalSpan::new(line_height, avail_height.get());
                        gen.get().generate(first, last, Box::new(span))
                    })
                    .unwrap_or_else(GeneratedTicks::none)
            })
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
    let PreState { font, padding, .. } = *state;
    Signal::derive(move || {
        let longest_chars = ticks.with(|ticks| {
            ticks
                .iter()
                .map(|(_, label)| label.len())
                .max()
                .unwrap_or_default()
                .max(min_chars.get())
        }) as f64;
        font.get().width() * longest_chars + padding.get().width()
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
pub fn TickLabels<X: Clone + 'static, Y: Clone + 'static>(
    ticks: UseTickLabels,
    edge: Edge,
    bounds: Memo<Bounds>,
    state: State<X, Y>,
) -> impl IntoView {
    let each_tick = move || {
        // Align vertical labels
        let ticks = ticks.ticks.get();
        if edge.is_vertical() {
            let (pos, labels): (Vec<f64>, Vec<String>) = ticks.into_iter().unzip();
            let labels = align_tick_labels(labels);
            pos.into_iter().zip(labels).collect::<Vec<_>>()
        } else {
            ticks
        }
    };

    let state = state.clone();
    view! {
        <g class="_chartistry_tick_labels">
            <For
                each=each_tick
                key=|(_, label)| label.to_owned()
                let:tick>
                <TickLabel edge=edge outer=bounds state=state.clone() tick=tick />
            </For>
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
    let PreState {
        debug,
        font,
        padding,
        ..
    } = state.pre;
    let projection = state.projection;

    let (position, label) = tick;
    let label_len = label.len();
    // Calculate positioning Bounds. Note: tick w / h includes padding
    let bounds = Signal::derive(move || {
        let font = font.get();
        let padding = padding.get();
        let width = font.width() * label_len as f64 + padding.width();
        let height = font.height() + padding.height();

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
        <g class="_chartistry_tick_label">
            <DebugRect label="tick" debug=debug bounds=vec![bounds, content.into()] />
            <text
                x=move || text_position.get().1
                y=move || content.get().centre_y()
                style="white-space: pre;"
                font-family="monospace"
                font-size=move || font.get().height()
                dominant-baseline="middle"
                text-anchor=move || text_position.get().0>
                {label.clone()}
            </text>
        </g>
    }
}
