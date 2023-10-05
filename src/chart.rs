use crate::{bounds::Bounds, debug::DebugRect, layout::LayoutOption, Font, Padding};
use leptos::*;

#[derive(Clone, Debug)]
pub struct Chart {
    width: MaybeSignal<f64>,
    height: MaybeSignal<f64>,
    padding: MaybeSignal<Option<Padding>>,
    debug: MaybeSignal<Option<bool>>,

    attr: Attr,
    top: Vec<LayoutOption>,
    right: Vec<LayoutOption>,
    bottom: Vec<LayoutOption>,
    left: Vec<LayoutOption>,
}

#[derive(Clone, Debug)]
pub struct Attr {
    font: MaybeSignal<Font>,
    padding: MaybeSignal<Padding>,
    debug: MaybeSignal<bool>,
}

impl Chart {
    pub fn new(
        width: impl Into<MaybeSignal<f64>>,
        height: impl Into<MaybeSignal<f64>>,
        font: impl Into<MaybeSignal<Font>>,
    ) -> Self {
        Self {
            width: width.into(),
            height: height.into(),
            padding: MaybeSignal::default(),
            debug: MaybeSignal::default(),

            attr: Attr::new(font.into()),
            top: vec![],
            right: vec![],
            bottom: vec![],
            left: vec![],
        }
    }

    pub fn set_padding(mut self, padding: impl Into<MaybeSignal<Option<Padding>>>) -> Self {
        self.padding = padding.into();
        self
    }
    pub fn with_padding(mut self, padding: impl Into<MaybeSignal<Padding>>) -> Self {
        self.attr.padding = padding.into();
        self
    }

    pub fn set_debug(mut self, debug: impl Into<MaybeSignal<Option<bool>>>) -> Self {
        self.debug = debug.into();
        self
    }
    pub fn with_debug(mut self, debug: impl Into<MaybeSignal<bool>>) -> Self {
        self.attr.debug = debug.into();
        self
    }

    pub fn add_top(mut self, opt: impl Into<LayoutOption>) -> Self {
        self.top.push(opt.into());
        self
    }

    pub fn add_right(mut self, opt: impl Into<LayoutOption>) -> Self {
        self.right.push(opt.into());
        self
    }

    pub fn add_bottom(mut self, opt: impl Into<LayoutOption>) -> Self {
        self.bottom.push(opt.into());
        self
    }

    pub fn add_left(mut self, opt: impl Into<LayoutOption>) -> Self {
        self.left.push(opt.into());
        self
    }
}

impl Attr {
    pub fn new(font: MaybeSignal<Font>) -> Self {
        Self {
            font,
            padding: MaybeSignal::default(),
            debug: MaybeSignal::default(),
        }
    }

    fn inherit<T: Clone>(
        &self,
        optional: MaybeSignal<Option<T>>,
        fallback: MaybeSignal<T>,
    ) -> MaybeSignal<T> {
        MaybeSignal::derive(move || optional.get().unwrap_or_else(|| fallback.get()))
    }

    pub fn font(&self, optional: MaybeSignal<Option<Font>>) -> MaybeSignal<Font> {
        self.inherit(optional, self.font)
    }

    pub fn padding(&self, optional: MaybeSignal<Option<Padding>>) -> MaybeSignal<Padding> {
        self.inherit(optional, self.padding)
    }

    pub fn debug(&self, optional: MaybeSignal<Option<bool>>) -> MaybeSignal<bool> {
        self.inherit(optional, self.debug)
    }
}

#[component]
pub fn Chart(chart: Chart) -> impl IntoView {
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
    } = chart;

    let chart_padding = attr.padding(padding);
    let debug = attr.debug(debug);

    let chart_bounds = Signal::derive(move || Bounds::new(width.get(), height.get()));
    let outer_bounds = Signal::derive(move || chart_padding.get().apply(chart_bounds.get()));
    let layout = LayoutOption::compose(outer_bounds, attr, top, right, bottom, left);

    view! {
        <div
            style="margin: 0 auto;"
            style:width=move || format!("{}px", width.get())
            style:height=move || format!("{}px", height.get())>
            <svg
                style="overflow: visible;"
                viewBox=move || format!("0 0 {} {}", width.get(), height.get())>
                <DebugRect label="Chart" debug=debug bounds=move || vec![chart_bounds.get(), outer_bounds.get()] />
                {layout}
            </svg>
        </div>
    }
}
