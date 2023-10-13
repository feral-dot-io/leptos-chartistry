use crate::{
    bounds::Bounds,
    debug::DebugRect,
    inner::{InnerLayout, InnerOption},
    layout::{HorizontalLayout, HorizontalOption, Layout, VerticalLayout, VerticalOption},
    series::{Series, UseSeries},
    Font, Padding,
};
use leptos::{svg::Svg, *};
use leptos_use::{
    use_intersection_observer_with_options, use_mouse_with_options, UseIntersectionObserverOptions,
    UseMouseCoordType, UseMouseEventExtractorDefault, UseMouseOptions, UseMouseSourceType,
};

pub struct Chart<X: 'static, Y: 'static> {
    width: MaybeSignal<f64>,
    height: MaybeSignal<f64>,
    padding: Option<MaybeSignal<Padding>>,
    debug: Option<MaybeSignal<bool>>,
    attr: Attr,

    top: Vec<Box<dyn HorizontalOption<X, Y>>>,
    right: Vec<Box<dyn VerticalOption<X, Y>>>,
    bottom: Vec<Box<dyn HorizontalOption<X, Y>>>,
    left: Vec<Box<dyn VerticalOption<X, Y>>>,
    inner: Vec<Box<dyn InnerOption<X, Y>>>,
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
            inner: vec![],
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

    pub fn add_top(mut self, opt: impl HorizontalLayout<X, Y>) -> Self {
        self.top.push(opt.apply_attr(&self.attr));
        self
    }

    pub fn add_right(mut self, opt: impl VerticalLayout<X, Y>) -> Self {
        self.right.push(opt.apply_attr(&self.attr));
        self
    }

    pub fn add_bottom(mut self, opt: impl HorizontalLayout<X, Y>) -> Self {
        self.bottom.push(opt.apply_attr(&self.attr));
        self
    }

    pub fn add_left(mut self, opt: impl VerticalLayout<X, Y>) -> Self {
        self.left.push(opt.apply_attr(&self.attr));
        self
    }

    pub fn add(mut self, opt: impl InnerLayout<X, Y>) -> Self {
        self.inner.push(opt.apply_attr(&self.attr));
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
        inner,
        series,
    } = chart;

    let padding = padding.unwrap_or(attr.padding);
    let debug = debug.unwrap_or(attr.debug);

    // Layout
    let chart_bounds = Signal::derive(move || Bounds::new(width.get(), height.get()));
    let outer_bounds = Signal::derive(move || padding.get().apply(chart_bounds.get()));
    let layout = Layout::compose(outer_bounds, top, right, bottom, left, &series);

    // SVG root
    let (root, svg_bounds, mouse) = root_node();

    // Inner layout
    let inner = (inner.into_iter())
        .map(|opt| {
            opt.to_use(&series, layout.projection)
                .render(layout.projection, mouse)
        })
        .collect_view();

    view! {
        <div
            style="margin: 0 auto;"
            style:width=move || format!("{}px", width.get())
            style:height=move || format!("{}px", height.get())>
            <svg
                node_ref=root
                style="overflow: visible;"
                viewBox=move || format!("0 0 {} {}", width.get(), height.get())>
                {inner}
                <DebugRect label="Chart" debug=debug bounds=move || vec![chart_bounds.get(), outer_bounds.get()] />
                {layout.view}
                <Series series=series projection=layout.projection />
            </svg>
        </div>
    }
}

fn root_node() -> (
    NodeRef<Svg>,
    Signal<Option<Bounds>>,
    Signal<Option<(f64, f64)>>,
) {
    let root = create_node_ref::<Svg>();

    // SVG bounds -- dimensions for our root <svg> element inside the document
    let (svg_bounds, set_svg_bounds) = create_signal::<Option<Bounds>>(None);
    use_intersection_observer_with_options(
        root,
        move |entries, _| {
            let entry = &entries[0];
            set_svg_bounds.set(Some(entry.bounding_client_rect().into()))
        },
        UseIntersectionObserverOptions::default().immediate(true),
    );

    // Mouse position relative to SVG
    let page_mouse = use_mouse_with_options(
        UseMouseOptions::default()
            .coord_type(UseMouseCoordType::<UseMouseEventExtractorDefault>::Page)
            .reset_on_touch_ends(true),
    );
    let mouse = Signal::derive(move || {
        if page_mouse.source_type.get() != UseMouseSourceType::Unset {
            svg_bounds.get().map(|svg| {
                let x = page_mouse.x.get() - svg.left_x();
                let y = page_mouse.y.get() - svg.top_y();
                (x, y)
            })
        } else {
            None
        }
    });

    (root, svg_bounds.into(), mouse)
}
