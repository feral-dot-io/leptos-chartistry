use super::{compose::UseLayout, HorizontalLayout, VerticalLayout};
use crate::{
    bounds::Bounds,
    debug::DebugRect,
    edge::Edge,
    series::{Data, UseSeries},
    state::{AttrState, State},
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
    pub format: TickFormatFn<Tick>, // TODO
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

impl<Tick: Clone> TickLabels<Tick> {
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

    pub fn set_formatter(
        mut self,
        format: impl Fn(&dyn TickState<Tick = Tick>, &Tick) -> String + 'static,
    ) -> Self {
        self.format = Rc::new(format);
        self
    }
}

impl<X: PartialEq> TickLabels<X> {
    pub fn generate_x<Y>(
        self,
        attr: &AttrState,
        data: Signal<Data<X, Y>>,
        avail_width: Signal<f64>,
    ) -> Signal<GeneratedTicks<X>> {
        let font = attr.font;
        let padding = attr.padding;
        create_memo(move |_| {
            let format = self.format.clone();
            data.with(|data| {
                data.x_range()
                    .map(|(first, last)| {
                        let font_width = font.get().width();
                        let padding_width = padding.get().width();
                        let span = HorizontalSpan::new(
                            format,
                            font_width,
                            padding_width,
                            avail_width.get(),
                        );
                        self.generator.generate(first, last, Box::new(span))
                    })
                    .unwrap_or_else(GeneratedTicks::none)
            })
        })
        .into()
    }
}

impl<Y: PartialEq> TickLabels<Y> {
    pub fn generate_y<X>(
        self,
        attr: &AttrState,
        data: Signal<Data<X, Y>>,
        avail_height: Signal<f64>,
    ) -> Signal<GeneratedTicks<Y>> {
        let font = attr.font;
        let padding = attr.padding;
        create_memo(move |_| {
            data.with(|data| {
                data.y_range()
                    .map(|(first, last)| {
                        let line_height = font.get().height() + padding.get().height();
                        let span = VerticalSpan::new(line_height, avail_height.get());
                        self.generator.generate(first, last, Box::new(span))
                    })
                    .unwrap_or_else(GeneratedTicks::none)
            })
        })
        .into()
    }
}

impl<X: Clone + PartialEq, Y> HorizontalLayout<X, Y> for TickLabels<X> {
    fn fixed_height(&self, attr: &AttrState) -> Signal<f64> {
        let font = attr.font;
        let padding = attr.padding;
        Signal::derive(move || with!(|font, padding| { font.height() + padding.height() }))
    }

    fn into_use(
        self: Rc<Self>,
        attr: &AttrState,
        series: &UseSeries<X, Y>,
        avail_width: Signal<f64>,
    ) -> Rc<dyn UseLayout> {
        Rc::new(UseTickLabels {
            ticks: self.map_ticks((*self).clone().generate_x(attr, series.data, avail_width)),
        })
    }
}

impl<X, Y: Clone + PartialEq> VerticalLayout<X, Y> for TickLabels<Y> {
    fn into_use(
        self: Rc<Self>,
        attr: &AttrState,
        series: &UseSeries<X, Y>,
        avail_height: Signal<f64>,
    ) -> (Signal<f64>, Rc<dyn UseLayout>) {
        let ticks = self.map_ticks((*self).clone().generate_y(attr, series.data, avail_height));
        let width = self.width(attr, ticks);
        (width, Rc::new(UseTickLabels { ticks }))
    }
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

    fn width(&self, attr: &AttrState, ticks: Signal<Vec<(f64, String)>>) -> Signal<f64> {
        let font = attr.font;
        let padding = attr.padding;
        let min_chars = self.min_chars;
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
}

impl UseLayout for UseTickLabels {
    fn render(&self, edge: Edge, bounds: Signal<Bounds>, state: &State) -> View {
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
pub fn TickLabels<'a>(
    ticks: UseTickLabels,
    edge: Edge,
    bounds: Signal<Bounds>,
    state: &'a State,
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
                <TickLabel
                    edge=edge
                    outer=bounds
                    state=&state
                    tick=tick
                />
            </For>
        </g>
    }
}

#[component]
fn TickLabel<'a>(
    edge: Edge,
    outer: Signal<Bounds>,
    state: &'a State,
    tick: (f64, String),
) -> impl IntoView {
    let State {
        projection,
        attr:
            AttrState {
                debug,
                font,
                padding,
                ..
            },
        ..
    } = *state;

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
