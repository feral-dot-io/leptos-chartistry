use crate::{bounds::Bounds, projection::Projection};
use leptos::{html::Div, *};
use leptos_use::{
    use_element_hover, use_mouse_with_options, use_resize_observer, UseMouseCoordType,
    UseMouseEventExtractorDefault, UseMouseOptions, UseMouseSourceType,
};

#[derive(Clone, Debug)]
pub struct UseWatchedNode {
    pub bounds: Signal<Option<Bounds>>,
    pub mouse_page: Signal<(f64, f64)>,
    pub mouse_chart: Signal<(f64, f64)>,
    pub mouse_chart_hover: Signal<bool>,
}

pub fn use_watched_node(node: NodeRef<Div>) -> UseWatchedNode {
    // SVG bounds -- dimensions for our root <svg> element inside the document
    let (bounds, set_bounds) = create_signal::<Option<Bounds>>(None);
    use_resize_observer(node, move |entries, _| {
        let rect = &entries[0].target().get_bounding_client_rect();
        let rect = Bounds::new(rect.width(), rect.height());
        set_bounds.set(Some(rect))
    });
    let bounds: Signal<Option<Bounds>> = bounds.into();

    // Mouse position
    let mouse = use_mouse_with_options(
        UseMouseOptions::default()
            .target(node)
            .coord_type(UseMouseCoordType::<UseMouseEventExtractorDefault>::Page)
            .reset_on_touch_ends(true),
    );

    // Mouse absolute coords on page
    let mouse_page = Signal::derive(move || {
        let x = mouse.x.get();
        let y = mouse.y.get();
        (x, y)
    });

    // Mouse relative to SVG
    let mouse_chart: Signal<_> = create_memo(move |_| {
        let (x, y) = mouse_page.get();
        let (left, top) = node
            .get()
            .map(|target| {
                let rect = target.get_bounding_client_rect();
                (rect.left(), rect.top())
            })
            .unwrap_or_default();
        let x = x - left;
        let y = y - top;
        (x, y)
    })
    .into();

    // Mouse inside SVG?
    let el_hover = use_element_hover(node);
    let mouse_chart_hover = create_memo(move |_| {
        let (x, y) = mouse_chart.get();
        mouse.source_type.get() != UseMouseSourceType::Unset
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
    pub fn mouse_hover_inner(&self, proj: Signal<Projection>) -> Signal<bool> {
        let (mouse_rel, hover) = (self.mouse_chart, self.mouse_chart_hover);
        create_memo(move |_| {
            let (x, y) = mouse_rel.get();
            hover.get() && proj.with(|proj| proj.bounds().contains(x, y))
        })
        .into()
    }
}
