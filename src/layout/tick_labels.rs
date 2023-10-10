use super::{compose::UseLayout, LayoutOption};
use crate::{
    bounds::Bounds,
    chart::Attr,
    debug::DebugRect,
    edge::Edge,
    projection::Projection,
    series::UseSeries,
    ticks::{AlignedFloatsGen, GeneratedTicks, HorizontalSpan, TickGen, VerticalSpan},
    Font, Padding,
};
use leptos::*;

pub struct TickLabels<Tick> {
    font: MaybeSignal<Option<Font>>,
    padding: MaybeSignal<Option<Padding>>,
    debug: MaybeSignal<Option<bool>>,
    generator: Box<dyn TickGen<Tick = Tick>>,
}

#[derive(Clone, Debug)]
pub struct UseTickLabels<Tick: 'static> {
    font: MaybeSignal<Font>,
    padding: MaybeSignal<Padding>,
    debug: MaybeSignal<bool>,
    ticks: Signal<GeneratedTicks<Tick>>,
}

impl<Tick> TickLabels<Tick> {
    fn new(gen: impl TickGen<Tick = Tick> + 'static) -> Self {
        Self {
            font: MaybeSignal::default(),
            padding: MaybeSignal::default(),
            debug: MaybeSignal::default(),
            generator: Box::new(gen),
        }
    }

    pub fn set_font(mut self, font: impl Into<MaybeSignal<Option<Font>>>) -> Self {
        self.font = font.into();
        self
    }

    pub fn set_padding(mut self, padding: impl Into<MaybeSignal<Option<Padding>>>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn set_debug(mut self, debug: impl Into<MaybeSignal<Option<bool>>>) -> Self {
        self.debug = debug.into();
        self
    }

    pub fn height(&self, attr: &Attr) -> Signal<f64> {
        let font = attr.font(self.font);
        let padding = attr.padding(self.padding);
        Signal::derive(move || with!(|font, padding| { font.height() + padding.height() }))
    }
}

impl TickLabels<f64> {
    pub fn aligned_floats() -> Self {
        Self::new(AlignedFloatsGen::new())
    }
}

impl<X> TickLabels<X> {
    pub(super) fn to_horizontal_use<Y>(
        self,
        attr: &Attr,
        series: &UseSeries<X, Y>,
        avail_width: Signal<f64>,
    ) -> UseTickLabels<X> {
        let data = series.data;
        let font = attr.font(self.font);
        let padding = attr.padding(self.padding);
        UseTickLabels {
            font,
            padding,
            debug: attr.debug(self.debug),
            ticks: Signal::derive(move || {
                data.with(|data| {
                    let (first, last) = data.x_range();
                    let font_width = font.get().width();
                    let padding_width = padding.get().width();
                    let span = HorizontalSpan::new(font_width, padding_width, avail_width.get());
                    self.generator.generate(first, last, Box::new(span))
                })
            }),
        }
    }
}

impl<Y> TickLabels<Y> {
    pub(super) fn to_vertical_use<X>(
        self,
        attr: &Attr,
        series: &UseSeries<X, Y>,
        avail_height: Signal<f64>,
    ) -> UseTickLabels<Y> {
        let data = series.data;
        let font = attr.font(self.font);
        let padding = attr.padding(self.padding);
        UseTickLabels {
            font,
            padding,
            debug: attr.debug(self.debug),
            ticks: Signal::derive(move || {
                data.with(|data| {
                    let (first, last) = data.y_range();
                    let line_height = font.get().height() + padding.get().height();
                    let span = VerticalSpan::new(line_height, avail_height.get());
                    self.generator.generate(first, last, Box::new(span))
                })
            }),
        }
    }
}

impl<Tick> From<TickLabels<Tick>> for LayoutOption<Tick> {
    fn from(ticks: TickLabels<Tick>) -> Self {
        LayoutOption::TickLabels(ticks)
    }
}

impl<Tick> UseLayout for UseTickLabels<Tick> {
    fn width(&self) -> Signal<f64> {
        let (font, padding, ticks) = (self.font, self.padding, self.ticks);
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
    let (font, padding, debug, ticks) = (ticks.font, ticks.padding, ticks.debug, ticks.ticks);
    let ticks = move || {
        ticks.with(move |GeneratedTicks { state, ticks }| {
            (ticks.iter())
                .map(|tick| {
                    let label = state.short_format(tick);
                    let position = state.position(tick);
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
