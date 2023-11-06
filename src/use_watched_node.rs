use crate::{bounds::Bounds, projection::Projection};
use leptos::{svg::Svg, *};
use leptos_use::{
    use_intersection_observer_with_options, use_mouse_with_options, UseIntersectionObserverOptions,
    UseMouseCoordType, UseMouseEventExtractorDefault, UseMouseOptions, UseMouseSourceType,
};

#[derive(Clone, Debug)]
pub struct UseWatchedNode {
    pub bounds: Signal<Option<Bounds>>,
    pub mouse_hover: Signal<bool>,
    pub mouse_abs: Signal<(f64, f64)>,
    pub mouse_rel: Signal<(f64, f64)>,
}

pub fn use_watched_node(node: NodeRef<Svg>) -> UseWatchedNode {
    // SVG bounds -- dimensions for our root <svg> element inside the document
    let (bounds, set_bounds) = create_signal::<Option<Bounds>>(None);
    use_intersection_observer_with_options(
        node,
        move |entries, _| {
            let entry = &entries[0];
            set_bounds.set(Some(entry.bounding_client_rect().into()))
        },
        UseIntersectionObserverOptions::default().immediate(true),
    );

    // Mouse position
    let mouse = use_mouse_with_options(
        UseMouseOptions::default()
            .coord_type(UseMouseCoordType::<UseMouseEventExtractorDefault>::Page)
            .reset_on_touch_ends(true),
    );

    // Mouse absolute coords on page
    let mouse_abs = Signal::derive(move || {
        let x = mouse.x.get();
        let y = mouse.y.get();
        (x, y)
    });

    // Mouse inside SVG?
    let mouse_hover = create_memo(move |_| {
        let (x, y) = mouse_abs.get();
        mouse.source_type.get() != UseMouseSourceType::Unset
            && (bounds.get())
                .map(|bounds| bounds.contains(x, y))
                .unwrap_or(false)
    })
    .into();

    // Mouse relative to SVG
    let mouse_rel: Signal<_> = create_memo(move |_| {
        (bounds.get())
            .map(|svg| {
                let (x, y) = mouse_abs.get();
                let x = x - svg.left_x();
                let y = y - svg.top_y();
                (x, y)
            })
            .unwrap_or_default()
    })
    .into();

    UseWatchedNode {
        bounds: bounds.into(),
        mouse_hover,
        mouse_abs,
        mouse_rel,
    }
}

impl UseWatchedNode {
    // Mouse inside inner chart?
    pub fn mouse_hover_inner(&self, proj: Signal<Projection>) -> Signal<bool> {
        let (mouse_rel, hover) = (self.mouse_rel, self.mouse_hover);
        create_memo(move |_| {
            let (x, y) = mouse_rel.get();
            hover.get() && proj.with(|proj| proj.bounds().contains(x, y))
        })
        .into()
    }
}
