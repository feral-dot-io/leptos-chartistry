use crate::{bounds::Bounds, projection::Projection};
use leptos::{html::Div, *};
use leptos_use::{
    use_mouse_with_options, use_resize_observer, UseMouseCoordType, UseMouseEventExtractorDefault,
    UseMouseOptions, UseMouseSourceType,
};

#[derive(Clone, Debug)]
pub struct UseWatchedNode {
    pub bounds: Signal<Option<Bounds>>,
    pub mouse_hover: Signal<bool>,
    pub mouse_abs: Signal<(f64, f64)>,
    pub mouse_rel: Signal<(f64, f64)>,
}

fn scroll_position() -> (f64, f64) {
    let window = window();
    let x = window.scroll_x().unwrap_or_default();
    let y = window.scroll_y().unwrap_or_default();
    (x, y)
}

pub fn use_watched_node(
    node: NodeRef<Div>,
    width: MaybeProp<f64>,
    height: MaybeProp<f64>,
) -> UseWatchedNode {
    // SVG bounds -- dimensions for our root <svg> element inside the document
    let (bounds, set_bounds) = create_signal::<Option<Bounds>>(None);
    use_resize_observer(node, move |entries, _| {
        let (scroll_x, scroll_y) = scroll_position();
        let rect = &entries[0].target().get_bounding_client_rect();
        let rect = Bounds::from_points(
            rect.left() + scroll_x,
            rect.top() + scroll_y,
            rect.right() + scroll_x,
            rect.bottom() + scroll_y,
        );
        set_bounds.set(Some(rect))
    });

    let bounds: Signal<Option<Bounds>> = create_memo(move |_| {
        bounds.get().or_else(|| {
            // Fallback to given dimensions if they exist
            width
                .get()
                .and_then(|width| height.get().map(|height| Bounds::new(width, height)))
        })
    })
    .into();

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
        bounds,
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
