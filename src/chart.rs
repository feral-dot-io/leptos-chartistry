use crate::{
    bounds::Bounds,
    debug::DebugRect,
    layout::{Layout, LayoutOption},
    series::{Series, UseSeries},
    Font, Padding,
};
use leptos::*;

pub struct Chart<X: 'static, Y: 'static> {
    width: MaybeSignal<f64>,
    height: MaybeSignal<f64>,
    padding: Option<MaybeSignal<Padding>>,
    debug: Option<MaybeSignal<bool>>,
    attr: Attr,

    top: Vec<LayoutOption<X>>,
    right: Vec<LayoutOption<Y>>,
    bottom: Vec<LayoutOption<X>>,
    left: Vec<LayoutOption<Y>>,

    series: UseSeries<X, Y>,
}

#[derive(Clone, Debug)]
pub struct Attr {
    pub font: MaybeSignal<Font>,
    pub padding: MaybeSignal<Padding>,
    pub debug: MaybeSignal<bool>,
}

impl<X, Y> Chart<X, Y> {
    pub fn new(
        width: impl Into<MaybeSignal<f64>>,
        height: impl Into<MaybeSignal<f64>>,
        font: impl Into<MaybeSignal<Font>>,
        series: UseSeries<X, Y>,
    ) -> Self {
        Self {
            width: width.into(),
            height: height.into(),
            padding: None,
            debug: None,
            attr: Attr {
                font: font.into(),
                padding: MaybeSignal::default(),
                debug: MaybeSignal::default(),
            },

            top: vec![],
            right: vec![],
            bottom: vec![],
            left: vec![],

            series,
        }
    }

    pub fn inherit_font(mut self, font: impl Into<MaybeSignal<Font>>) -> Self {
        self.attr.font = font.into();
        self
    }

    pub fn set_padding(mut self, padding: impl Into<MaybeSignal<Padding>>) -> Self {
        self.padding = Some(padding.into());
        self
    }
    pub fn inherit_padding(mut self, padding: impl Into<MaybeSignal<Padding>>) -> Self {
        self.attr.padding = padding.into();
        self
    }

    pub fn set_debug(mut self, debug: impl Into<MaybeSignal<bool>>) -> Self {
        self.debug = Some(debug.into());
        self
    }
    pub fn inherit_debug(mut self, debug: impl Into<MaybeSignal<bool>>) -> Self {
        self.attr.debug = debug.into();
        self
    }

    pub fn add_top(mut self, opt: impl Into<LayoutOption<X>>) -> Self {
        self.top.push(opt.into());
        self
    }

    pub fn add_right(mut self, opt: impl Into<LayoutOption<Y>>) -> Self {
        self.right.push(opt.into());
        self
    }

    pub fn add_bottom(mut self, opt: impl Into<LayoutOption<X>>) -> Self {
        self.bottom.push(opt.into());
        self
    }

    pub fn add_left(mut self, opt: impl Into<LayoutOption<Y>>) -> Self {
        self.left.push(opt.into());
        self
    }
}

#[component]
pub fn Chart<X: 'static, Y: 'static>(chart: Chart<X, Y>) -> impl IntoView {
    let Chart {
        width,
        height,
        padding,
        debug,
        attr,

        top,
        right,
        bottom,
        left,

        series,
    } = chart;

    let padding = padding.unwrap_or(attr.padding);
    let debug = debug.unwrap_or(attr.debug);

    let chart_bounds = Signal::derive(move || Bounds::new(width.get(), height.get()));
    let outer_bounds = Signal::derive(move || padding.get().apply(chart_bounds.get()));
    let layout = Layout::compose(outer_bounds, top, right, bottom, left, &attr, &series);

    view! {
        <div
            style="margin: 0 auto;"
            style:width=move || format!("{}px", width.get())
            style:height=move || format!("{}px", height.get())>
            <svg
                style="overflow: visible;"
                viewBox=move || format!("0 0 {} {}", width.get(), height.get())>
                <DebugRect label="Chart" debug=debug bounds=move || vec![chart_bounds.get(), outer_bounds.get()] />
                {layout.view}
                <Series series=series projection=layout.projection />
            </svg>
        </div>
    }
}
