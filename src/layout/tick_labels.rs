use super::{
    compose::UseLayout, HorizontalLayout, HorizontalOption, VerticalLayout, VerticalOption,
};
use crate::{
    bounds::Bounds,
    chart::Attr,
    debug::DebugRect,
    edge::Edge,
    projection::Projection,
    series::UseSeries,
    ticks::{AlignedFloatsGen, GeneratedTicks, TickGen, Ticks, TimestampGen, UseTicks},
    Font, Padding, Period,
};
use chrono::prelude::*;
use leptos::*;
use std::borrow::Borrow;
use std::rc::Rc;

#[derive(Clone)]
pub struct TickLabels<Tick: Clone> {
    font: Option<MaybeSignal<Font>>,
    padding: Option<MaybeSignal<Padding>>,
    debug: Option<MaybeSignal<bool>>,
    generator: Rc<dyn TickGen<Tick = Tick>>,
}

#[derive(Clone)]
pub struct TickLabelsAttr<Tick>(pub(crate) Ticks<Tick>);

#[derive(Clone, Debug)]
pub struct UseTickLabels<Tick: 'static>(UseTicks<Tick>);

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
            debug: None,
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

    pub fn set_debug(mut self, debug: impl Into<MaybeSignal<bool>>) -> Self {
        self.debug = Some(debug.into());
        self
    }

    pub(crate) fn apply_attr(self, attr: &Attr) -> Ticks<Tick> {
        Ticks {
            font: self.font.unwrap_or(attr.font),
            padding: self.padding.unwrap_or(attr.padding),
            debug: self.debug.unwrap_or(attr.debug),
            generator: self.generator,
        }
    }
}

impl<X: Clone + PartialEq + 'static, Y: 'static> HorizontalLayout<X, Y> for TickLabels<X> {
    fn apply_attr(self, attr: &Attr) -> Rc<dyn HorizontalOption<X, Y>> {
        Rc::new(TickLabelsAttr(self.apply_attr(attr)))
    }
}

impl<X: 'static, Y: Clone + PartialEq + 'static> VerticalLayout<X, Y> for TickLabels<Y> {
    fn apply_attr(self, attr: &Attr) -> Rc<dyn VerticalOption<X, Y>> {
        Rc::new(TickLabelsAttr(self.apply_attr(attr)))
    }
}

impl<X: Clone + PartialEq, Y> HorizontalOption<X, Y> for TickLabelsAttr<X> {
    fn height(&self) -> Signal<f64> {
        let (font, padding) = (self.0.font, self.0.padding);
        Signal::derive(move || with!(|font, padding| { font.height() + padding.height() }))
    }

    fn to_use(
        self: Rc<Self>,
        series: &UseSeries<X, Y>,
        avail_width: Signal<f64>,
    ) -> Box<dyn UseLayout> {
        Box::new(UseTickLabels(
            self.0.clone().generate_x(series.data, avail_width),
        ))
    }
}

impl<X, Y: Clone + PartialEq> VerticalOption<X, Y> for TickLabelsAttr<Y> {
    fn to_use(
        self: Rc<Self>,
        series: &UseSeries<X, Y>,
        avail_height: Signal<f64>,
    ) -> Box<dyn UseLayout> {
        Box::new(UseTickLabels(
            self.0.clone().generate_y(series.data, avail_height),
        ))
    }
}

impl<Tick> UseLayout for UseTickLabels<Tick> {
    fn width(&self) -> Signal<f64> {
        let (font, padding, ticks) = (self.0.font, self.0.padding, self.0.ticks);
        Signal::derive(move || {
            let chars = ticks.with(|ticks| {
                (ticks.ticks.iter())
                    .map(|tick| ticks.state.short_format(tick).len())
                    .max()
                    .unwrap_or_default()
            });
            font.get().width() * chars as f64 + padding.get().width()
        })
    }

    fn render<'a>(&self, edge: Edge, bounds: Bounds, proj: Signal<Projection>) -> View {
        view! { <TickLabels ticks=self edge=edge bounds=bounds projection=proj /> }
    }
}

#[component]
pub fn TickLabels<'a, Tick: 'static>(
    ticks: &'a UseTickLabels<Tick>,
    edge: Edge,
    bounds: Bounds,
    projection: Signal<Projection>,
) -> impl IntoView {
    let (font, padding, debug, ticks) =
        (ticks.0.font, ticks.0.padding, ticks.0.debug, ticks.0.ticks);
    let ticks = move || {
        ticks.with(move |GeneratedTicks { state, ticks }| {
            // Generate tick labels
            let labels = ticks
                .iter()
                .map(|tick| (state.position(tick), state.short_format(tick)))
                .collect::<Vec<_>>();

            // Align vertical labels
            let labels = if edge.is_vertical() {
                // Find longest label length
                let min_label = labels
                    .iter()
                    .map(|(_, label)| label.len())
                    .max()
                    .unwrap_or_default();
                // Pad labels to same length
                labels
                    .into_iter()
                    .map(|(pos, mut label)| {
                        let spaces = " ".repeat(min_label.saturating_sub(label.len()));
                        label.insert_str(0, &spaces);
                        (pos, label)
                    })
                    .collect::<Vec<_>>()
            } else {
                labels
            };

            // Render tick labels
            labels
                .into_iter()
                .map(|(position, label)| {
                    view! {
                        <TickLabel
                            edge=edge
                            bounds=bounds
                            projection=projection
                            label=label
                            position=position
                            font=font
                            padding=padding
                            debug=debug
                        />
                    }
                })
                .collect_view()
        })
    };

    view! {
        <g class="_chartistry_tick_labels">
            {ticks}
        </g>
    }
}

#[component]
fn TickLabel(
    edge: Edge,
    bounds: Bounds,
    projection: Signal<Projection>,
    label: String,
    position: f64,
    font: MaybeSignal<Font>,
    padding: MaybeSignal<Padding>,
    debug: MaybeSignal<bool>,
) -> impl IntoView {
    move || {
        let proj = projection.get();
        let font = font.get();
        let padding = padding.get();

        // Calculate positioning Bounds. Note: tick w / h includes padding
        let width = font.width() * label.len() as f64 + padding.width();
        let height = font.height() + padding.height();
        let bounds = match edge {
            Edge::Top | Edge::Bottom => {
                let (x, _) = proj.data_to_svg(position, 0.0);
                let x = x - width / 2.0;
                Bounds::from_points(x, bounds.top_y(), x + width, bounds.bottom_y())
            }

            Edge::Left | Edge::Right => {
                let (_, y) = proj.data_to_svg(0.0, position);
                let y = y - height / 2.0;
                Bounds::from_points(bounds.left_x(), y, bounds.right_x(), y + height)
            }
        };
        let content = padding.apply(bounds);

        // Determine text position
        let (anchor, x) = match edge {
            Edge::Top | Edge::Bottom => ("middle", content.centre_x()),

            Edge::Left | Edge::Right => {
                let (x, anchor) = if edge == Edge::Left {
                    (content.right_x(), "end")
                } else {
                    (content.left_x(), "start")
                };
                (anchor, x)
            }
        };

        view! {
            <g class="_chartistry_tick_label">
                <DebugRect label="tick" debug=debug bounds=move || vec![bounds, content] />
                <text
                    x=x
                    y=content.centre_y()
                    style="white-space: pre;"
                    font-family="monospace"
                    font-size=font.height()
                    dominant-baseline="middle"
                    text-anchor=anchor>
                    {label.clone()}
                </text>
            </g>
        }
    }
}
