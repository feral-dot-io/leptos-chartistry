use crate::{
    chart::Attr,
    debug::DebugRect,
    projection::Projection,
    series::UseSeries,
    ticks::{Ticks, UseTicks},
    TickLabels,
};
use leptos::*;

pub struct GridLine<Tick> {
    width: MaybeSignal<f64>,
    ticks: TickLabels<Tick>,
}

pub struct HorizontalGridLine<Tick>(GridLine<Tick>);
pub struct VerticalGridLine<Tick>(GridLine<Tick>);

pub struct GridLineAttr<Tick> {
    width: MaybeSignal<f64>,
    ticks: Ticks<Tick>,
}

#[derive(Clone, Debug)]
pub struct UseGridLine<Tick: 'static> {
    width: MaybeSignal<f64>,
    ticks: UseTicks<Tick>,
}

impl<Tick> GridLine<Tick> {
    fn new(ticks: TickLabels<Tick>) -> Self {
        Self {
            width: 1.0.into(),
            ticks,
        }
    }

    /// Horizontal grid lines running parallel to the x-axis. These run from left to right at each tick.
    pub fn horizontal(ticks: impl Into<TickLabels<Tick>>) -> VerticalGridLine<Tick> {
        VerticalGridLine(Self::new(ticks.into()))
    }
    /// Vertical grid lines running parallel to the y-axis. These run from top to bottom at each tick.
    pub fn vertical(ticks: impl Into<TickLabels<Tick>>) -> HorizontalGridLine<Tick> {
        HorizontalGridLine(Self::new(ticks.into()))
    }

    pub(crate) fn apply_attr(self, attr: &Attr) -> GridLineAttr<Tick> {
        GridLineAttr {
            width: self.width,
            ticks: self.ticks.apply_attr(attr).0,
        }
    }
}

impl<Tick> HorizontalGridLine<Tick> {
    pub(crate) fn apply_attr(self, attr: &Attr) -> GridLineAttr<Tick> {
        self.0.apply_attr(attr)
    }
}

impl<Tick> VerticalGridLine<Tick> {
    pub(crate) fn apply_attr(self, attr: &Attr) -> GridLineAttr<Tick> {
        self.0.apply_attr(attr)
    }
}

impl<Tick> GridLineAttr<Tick> {
    pub fn generate_x<Y>(
        self,
        series: &UseSeries<Tick, Y>,
        proj: Signal<Projection>,
    ) -> UseGridLine<Tick> {
        let avail_width = Signal::derive(move || with!(|proj| proj.bounds().width()));
        UseGridLine {
            width: self.width,
            ticks: self.ticks.generate_x(series.data, avail_width),
        }
    }

    pub fn generate_y<X>(
        self,
        series: &UseSeries<X, Tick>,
        proj: Signal<Projection>,
    ) -> UseGridLine<Tick> {
        let avail_height = Signal::derive(move || with!(|proj| proj.bounds().height()));
        UseGridLine {
            width: self.width,
            ticks: self.ticks.generate_y(series.data, avail_height),
        }
    }
}

#[component]
pub fn UseHorizontalGridLine<X: 'static>(
    line: UseGridLine<X>,
    projection: Signal<Projection>,
) -> impl IntoView {
    let ticks = Signal::derive(move || {
        let ticks = line.ticks.ticks; // Ticky ticky tick tick
        with!(|ticks, projection| {
            (ticks.ticks.iter())
                .map(|tick| {
                    let x = ticks.state.position(tick);
                    let x = projection.data_to_svg(x, 0.0).0;
                    let bounds = projection.bounds();
                    view! {
                        <line
                            x1=x
                            y1=move || bounds.top_y()
                            x2=x
                            y2=move || bounds.bottom_y()
                            stroke="gainsboro"
                            stroke-width=line.width />
                    }
                })
                .collect_view()
        })
    });
    view! {
        <g class="_chartistry_grid_line_horizontal">
            <DebugRect label="HorizontalGridLine" debug=line.ticks.debug />
            {ticks}
        </g>
    }
}

#[component]
pub fn UseVerticalGridLine<Y: 'static>(
    line: UseGridLine<Y>,
    projection: Signal<Projection>,
) -> impl IntoView {
    let ticks = Signal::derive(move || {
        let ticks = line.ticks.ticks;
        with!(|ticks, projection| {
            (ticks.ticks.iter())
                .map(|tick| {
                    let y = ticks.state.position(tick);
                    let y = projection.data_to_svg(0.0, y).1;
                    let bounds = projection.bounds();
                    view! {
                        <line
                            x1=move || bounds.left_x()
                            y1=y
                            x2=move || bounds.right_x()
                            y2=y
                            stroke="gainsboro"
                            stroke-width=line.width />
                    }
                })
                .collect_view()
        })
    });
    view! {
        <g class="_chartistry_grid_line_vertical">
            <DebugRect label="VerticalGridLine" debug=line.ticks.debug />
            {ticks}
        </g>
    }
}
