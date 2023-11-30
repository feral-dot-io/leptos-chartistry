use super::{compose::UseLayout, HorizontalLayout, VerticalLayout};
use crate::{
    bounds::Bounds,
    debug::DebugRect,
    edge::Edge,
    series::UseSeries,
    state::{PreState, State},
    ticks::{
        AlignedFloatsGen, GeneratedTicks, HorizontalSpan, TickFormatFn, TickGen, TickState,
        TimestampGen, VerticalSpan,
    },
    Period,
};
use chrono::prelude::*;
use leptos::*;
use std::borrow::Borrow;
use std::rc::Rc;

#[derive(Clone)]
pub struct TickLabels<Tick> {
    min_chars: MaybeSignal<usize>,
    format: TickFormatFn<Tick>,
    generator: Rc<dyn TickGen<Tick = Tick>>,
}

#[derive(Clone)]
pub struct UseTickLabels {
    ticks: Signal<Vec<(f64, String)>>,
}

impl TickLabels<f64> {
    pub fn aligned_floats() -> Self {
        Self::new(AlignedFloatsGen::new())
    }
}

impl<Tz> TickLabels<DateTime<Tz>>
where
    Tz: TimeZone + std::fmt::Debug + 'static,
    Tz::Offset: std::fmt::Display,
{
    pub fn timestamps() -> Self {
        Self::new(TimestampGen::new(Period::all()))
    }

    pub fn timestamp_periods(periods: impl Borrow<[Period]>) -> Self {
        Self::new(TimestampGen::new(periods))
    }

    pub fn timestamp_period(period: Period) -> Self {
        Self::new(TimestampGen::new([period]))
    }
}

impl<Tick> TickLabels<Tick> {
    fn new(gen: impl TickGen<Tick = Tick> + 'static) -> Self {
        Self {
            min_chars: 0.into(),
            format: Rc::new(|s, t| s.short_format(t)),
            generator: Rc::new(gen),
        }
    }

    pub fn set_min_chars(mut self, min_chars: impl Into<MaybeSignal<usize>>) -> Self {
        self.min_chars = min_chars.into();
        self
    }

    pub fn set_format(
        mut self,
        format: impl Fn(&dyn TickState<Tick = Tick>, &Tick) -> String + 'static,
    ) -> Self {
        self.format = Rc::new(format);
        self
    }
}

impl<X: PartialEq> TickLabels<X> {
    pub fn generate_x<Y>(
        &self,
        state: &PreState<X, Y>,
        avail_width: Signal<f64>,
    ) -> Signal<GeneratedTicks<X>> {
        let PreState {
            font,
            padding,
            x_range,
            ..
        } = *state;
        let format = self.format.clone();
        let gen = self.generator.clone();
        create_memo(move |_| {
            x_range.with(|x_range| {
                x_range
                    .as_ref()
                    .map(|(first, last)| {
                        let font_width = font.get().width();
                        let padding_width = padding.get().width();
                        let span = HorizontalSpan::new(
                            format.clone(),
                            font_width,
                            padding_width,
                            avail_width.get(),
                        );
                        gen.generate(first, last, Box::new(span))
                    })
                    .unwrap_or_else(GeneratedTicks::none)
            })
        })
        .into()
    }
}

impl<Y: PartialEq> TickLabels<Y> {
    pub fn generate_y<X>(
        &self,
        state: &PreState<X, Y>,
        avail_height: Signal<f64>,
    ) -> Signal<GeneratedTicks<Y>> {
        let PreState {
            font,
            padding,
            y_range,
            ..
        } = *state;
        let gen = self.generator.clone();
        create_memo(move |_| {
            y_range.with(|y_range| {
                y_range
                    .as_ref()
                    .map(|(first, last)| {
                        let line_height = font.get().height() + padding.get().height();
                        let span = VerticalSpan::new(line_height, avail_height.get());
                        gen.generate(first, last, Box::new(span))
                    })
                    .unwrap_or_else(GeneratedTicks::none)
            })
        })
        .into()
    }
}

impl<X: Clone + PartialEq, Y: Clone> HorizontalLayout<X, Y> for TickLabels<X> {
    fn fixed_height(&self, state: &PreState<X, Y>) -> Signal<f64> {
        let PreState { font, padding, .. } = *state;
        Signal::derive(move || with!(|font, padding| { font.height() + padding.height() }))
    }

    fn into_use(
        self: Rc<Self>,
        state: &PreState<X, Y>,
        _: &UseSeries<X, Y>,
        avail_width: Memo<f64>,
    ) -> Box<dyn UseLayout<X, Y>> {
        Box::new(UseTickLabels {
            ticks: self.map_ticks((*self).clone().generate_x(state, avail_width.into())),
        })
    }
}

impl<X: Clone, Y: Clone + PartialEq> VerticalLayout<X, Y> for TickLabels<Y> {
    fn into_use(
        self: Rc<Self>,
        state: &PreState<X, Y>,
        _: &UseSeries<X, Y>,
        avail_height: Memo<f64>,
    ) -> (Signal<f64>, Box<dyn UseLayout<X, Y>>) {
        let ticks = self.map_ticks((*self).clone().generate_y(state, avail_height.into()));
        let width = mk_width(self.min_chars, state, ticks);
        (width, Box::new(UseTickLabels { ticks }))
    }
}

fn mk_width<X, Y>(
    min_chars: MaybeSignal<usize>,
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

impl<Tick: Clone> TickLabels<Tick> {
    fn map_ticks(&self, gen: Signal<GeneratedTicks<Tick>>) -> Signal<Vec<(f64, String)>> {
        let format = self.format.clone();
        Signal::derive(move || {
            gen.with(|GeneratedTicks { ticks, state }| {
                ticks
                    .iter()
                    .map(|tick| (state.position(tick), (format)(&**state, tick)))
                    .collect()
            })
        })
    }
}

impl<X: Clone, Y: Clone> UseLayout<X, Y> for UseTickLabels {
    fn render(&self, edge: Edge, bounds: Memo<Bounds>, state: &State<X, Y>) -> View {
        view! { <TickLabels ticks=self.clone() edge=edge bounds=bounds state=state /> }
    }
}

pub fn align_tick_labels(labels: Vec<String>) -> Vec<String> {
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
pub fn TickLabels<'a, X: Clone + 'static, Y: Clone + 'static>(
    ticks: UseTickLabels,
    edge: Edge,
    bounds: Memo<Bounds>,
    state: &'a State<X, Y>,
) -> impl IntoView {
    let state = state.clone();
    let UseTickLabels { ticks, .. } = ticks;

    let ticks = move || {
        // Align vertical labels
        let ticks = ticks.get();
        if edge.is_vertical() {
            let (pos, labels): (Vec<f64>, Vec<String>) = ticks.into_iter().unzip();
            let labels = align_tick_labels(labels);
            pos.into_iter().zip(labels).collect::<Vec<_>>()
        } else {
            ticks
        }
    };

    view! {
        <g class="_chartistry_tick_labels">
            <For
                each=ticks
                key=|(_, label)| label.to_owned()
                let:tick
            >
                <TickLabel edge=edge outer=bounds state=&state tick=tick />
            </For>
        </g>
    }
}

#[component]
fn TickLabel<'a, X: 'static, Y: 'static>(
    edge: Edge,
    outer: Memo<Bounds>,
    state: &'a State<X, Y>,
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
                let (x, _) = proj.data_to_svg(position, 0.0);
                let x = x - width / 2.0;
                Bounds::from_points(x, outer.top_y(), x + width, outer.bottom_y())
            }

            Edge::Left | Edge::Right => {
                let (_, y) = proj.data_to_svg(0.0, position);
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
