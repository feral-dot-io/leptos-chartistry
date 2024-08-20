use crate::bounds::Bounds;
use leptos::{html::Div, *};
use leptos_use::{
    use_element_hover, use_mouse_with_options, use_resize_observer_with_options, UseMouseCoordType,
    UseMouseOptions, UseMouseSourceType, UseResizeObserverOptions,
};
use std::convert::Infallible;
use web_sys::ResizeObserverBoxOptions;

#[derive(Clone, Debug)]
pub struct UseWatchedNode {
    pub bounds: Signal<Option<Bounds>>,
    pub mouse_page: Signal<(f64, f64)>,
    pub mouse_chart: Signal<(f64, f64)>,
    pub mouse_chart_hover: Signal<bool>,
}

pub fn use_watched_node(node: NodeRef<Div>) -> UseWatchedNode {
    // Outer chart bounds -- dimensions for our root element inside the document
    // Note <svg> has issues around observing size changes. So wrap in a <div>
    // Note also that the box_ option doesn't seem to work for us so wrap in another <div>
    let (bounds, set_bounds) = create_signal::<Option<Bounds>>(None);
    use_resize_observer_with_options(
        node,
        move |entries, _| {
            let rect = &entries[0].target().get_bounding_client_rect();
            let rect = Bounds::new(rect.width(), rect.height());
            set_bounds.set(Some(rect))
        },
        UseResizeObserverOptions {
            box_: Some(ResizeObserverBoxOptions::BorderBox),
        },
    );
    let bounds: Signal<Option<Bounds>> = bounds.into();

    // Mouse position
    let mouse_page = use_mouse_with_options(
        UseMouseOptions::default()
            .target(node)
            .coord_type(UseMouseCoordType::<Infallible>::Page)
            .reset_on_touch_ends(true),
    );

    // Mouse absolute coords on page
    let mouse_page_type = mouse_page.source_type;
    let mouse_page = Signal::derive(move || {
        let x = mouse_page.x.get();
        let y = mouse_page.y.get();
        (x, y)
    });

    // Mouse relative to SVG
    let mouse_client = use_mouse_with_options(
        UseMouseOptions::default()
            .target(node)
            .coord_type(UseMouseCoordType::<Infallible>::Client)
            .reset_on_touch_ends(true),
    );
    let mouse_chart: Signal<_> = create_memo(move |_| {
        let (left, top) = node
            .get()
            .map(|target| {
                let rect = target.get_bounding_client_rect();
                (rect.left(), rect.top())
            })
            .unwrap_or_default();
        let x = mouse_client.x.get() - left;
        let y: f64 = mouse_client.y.get() - top;
        (x, y)
    })
    .into();

    // Mouse inside SVG?
    let el_hover = use_element_hover(node);
    let mouse_chart_hover = create_memo(move |_| {
        let (x, y) = mouse_chart.get();
        mouse_page_type.get() != UseMouseSourceType::Unset
            && el_hover.get()
            && bounds
                .get()
                .map(|bounds| bounds.contains(x, y))
                .unwrap_or(false)
    })
    .into();

    UseWatchedNode {
        bounds,
        mouse_page,
        mouse_chart,
        mouse_chart_hover,
    }
}

impl UseWatchedNode {
    // Mouse inside inner chart?
    pub fn mouse_hover_inner(&self, inner: Memo<Bounds>) -> Signal<bool> {
        let (mouse_rel, hover) = (self.mouse_chart, self.mouse_chart_hover);
        create_memo(move |_| {
            let (x, y) = mouse_rel.get();
            hover.get() && inner.get().contains(x, y)
        })
        .into()
    }
}
