use super::{
    compose::UseLayout, HorizontalLayout, HorizontalOption, VerticalLayout, VerticalOption,
};
use crate::{
    bounds::Bounds,
    chart::Attr,
    debug::DebugRect,
    edge::Edge,
    projection::Projection,
    series::{Data, UseSeries},
    state::State,
    ticks::{
        short_format_fn, AlignedFloatsGen, GeneratedTicks, HorizontalSpan, TickFormatFn, TickGen,
        TickState, TimestampGen, VerticalSpan,
    },
    Font, Padding, Period,
};
use chrono::prelude::*;
use leptos::*;
use std::borrow::Borrow;
use std::rc::Rc;

#[derive(Clone)]
pub struct TickLabels<Tick: Clone> {
    font: Option<MaybeSignal<Font>>,
    min_chars: MaybeSignal<usize>,
    padding: Option<MaybeSignal<Padding>>,
    debug: Option<MaybeSignal<bool>>,
    format: Option<TickFormatFn<Tick>>,
    generator: Rc<dyn TickGen<Tick = Tick>>,
}

#[derive(Clone)]
pub struct TickLabelsAttr<Tick> {
    font: MaybeSignal<Font>,
    min_chars: MaybeSignal<usize>,
    padding: MaybeSignal<Padding>,
    debug: MaybeSignal<bool>,
    pub format: TickFormatFn<Tick>,
    generator: Rc<dyn TickGen<Tick = Tick>>,
}

#[derive(Clone)]
pub struct UseTickLabels {
    font: MaybeSignal<Font>,
    min_chars: MaybeSignal<usize>,
    padding: MaybeSignal<Padding>,
    debug: MaybeSignal<bool>,
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
            font: None,
            padding: None,
            min_chars: 0.into(),
            debug: None,
            format: None,
            generator: Rc::new(gen),
        }
    }

    pub fn set_font(mut self, font: impl Into<MaybeSignal<Font>>) -> Self {
        self.font = Some(font.into());
        self
    }

    pub fn set_padding(mut self, padding: impl Into<MaybeSignal<Padding>>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn set_min_chars(mut self, min_chars: impl Into<MaybeSignal<usize>>) -> Self {
        self.min_chars = min_chars.into();
        self
    }

    pub fn set_debug(mut self, debug: impl Into<MaybeSignal<bool>>) -> Self {
        self.debug = Some(debug.into());
        self
    }

    pub fn set_formatter(
        mut self,
        format: impl Fn(&dyn TickState<Tick = Tick>, &Tick) -> String + 'static,
    ) -> Self {
        self.format = Some(Rc::new(format));
        self
    }

    pub(crate) fn apply_attr(
        self,
        attr: &Attr,
        def_format: TickFormatFn<Tick>,
    ) -> TickLabelsAttr<Tick> {
        TickLabelsAttr {
            font: self.font.unwrap_or(attr.font),
            padding: self.padding.unwrap_or(attr.padding),
            min_chars: self.min_chars,
            debug: self.debug.unwrap_or(attr.debug),
            format: self.format.unwrap_or(def_format),
            generator: self.generator,
        }
    }
}

impl<X: PartialEq> TickLabelsAttr<X> {
    pub fn generate_x<Y>(
        self,
        data: Signal<Data<X, Y>>,
        avail_width: Signal<f64>,
    ) -> Signal<GeneratedTicks<X>> {
        let (font, padding) = (self.font, self.padding);
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

impl<Y: PartialEq> TickLabelsAttr<Y> {
    pub fn generate_y<X>(
        self,
        data: Signal<Data<X, Y>>,
        avail_height: Signal<f64>,
    ) -> Signal<GeneratedTicks<Y>> {
        let (font, padding) = (self.font, self.padding);
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

impl<X: Clone + PartialEq + 'static, Y: 'static> HorizontalLayout<X, Y> for TickLabels<X> {
    fn apply_attr(self, attr: &Attr) -> Rc<dyn HorizontalOption<X, Y>> {
        Rc::new(self.apply_attr(attr, short_format_fn()))
    }
}

impl<X: 'static, Y: Clone + PartialEq + 'static> VerticalLayout<X, Y> for TickLabels<Y> {
    fn apply_attr(self, attr: &Attr) -> Rc<dyn VerticalOption<X, Y>> {
        Rc::new(self.apply_attr(attr, short_format_fn()))
    }
}

impl<X: Clone + PartialEq, Y> HorizontalOption<X, Y> for TickLabelsAttr<X> {
    fn height(&self) -> Signal<f64> {
        let (font, padding) = (self.font, self.padding);
        Signal::derive(move || with!(|font, padding| { font.height() + padding.height() }))
    }

    fn into_use(
        self: Rc<Self>,
        series: &UseSeries<X, Y>,
        avail_width: Signal<f64>,
    ) -> Rc<dyn UseLayout> {
        Rc::new(UseTickLabels {
            font: self.font,
            padding: self.padding,
            min_chars: self.min_chars,
            debug: self.debug,
            ticks: self.map_ticks((*self).clone().generate_x(series.data, avail_width)),
        })
    }
}

impl<X, Y: Clone + PartialEq> VerticalOption<X, Y> for TickLabelsAttr<Y> {
    fn into_use(
        self: Rc<Self>,
        series: &UseSeries<X, Y>,
        avail_height: Signal<f64>,
    ) -> Rc<dyn UseLayout> {
        Rc::new(UseTickLabels {
            font: self.font,
            padding: self.padding,
            min_chars: self.min_chars,
            debug: self.debug,
            ticks: self.map_ticks((*self).clone().generate_y(series.data, avail_height)),
        })
    }
}

impl<Tick> TickLabelsAttr<Tick> {
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

impl UseLayout for UseTickLabels {
    fn width(&self) -> Signal<f64> {
        let font = self.font;
        let padding = self.padding;
        let min_chars = self.min_chars;
        let labels = self.ticks;
        Signal::derive(move || {
            let longest_chars = labels.with(|labels| {
                labels
                    .iter()
                    .map(|(_, label)| label.len())
                    .max()
                    .unwrap_or_default()
                    .max(min_chars.get())
            }) as f64;
            font.get().width() * longest_chars + padding.get().width()
        })
    }

    fn render(&self, edge: Edge, bounds: Signal<Bounds>, state: &State) -> View {
        view! { <TickLabels ticks=self.clone() edge=edge bounds=bounds projection=state.projection /> }
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
pub fn TickLabels(
    ticks: UseTickLabels,
    edge: Edge,
    bounds: Signal<Bounds>,
    projection: Signal<Projection>,
) -> impl IntoView {
    let UseTickLabels {
        font,
        padding,
        debug,
        ticks,
        ..
    } = ticks;

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
                    projection=projection
                    tick=tick
                    font=font
                    padding=padding
                    debug=debug
                />
            </For>
        </g>
    }
}

#[component]
fn TickLabel(
    edge: Edge,
    outer: Signal<Bounds>,
    projection: Signal<Projection>,
    tick: (f64, String),
    font: MaybeSignal<Font>,
    padding: MaybeSignal<Padding>,
    debug: MaybeSignal<bool>,
) -> impl IntoView {
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
