use leptos::*;

use crate::bounds::Bounds;

#[component]
pub fn DebugRect(
    #[prop(into)] label: String,
    debug: MaybeSignal<bool>,
    #[prop(optional, into)] bounds: Signal<Vec<Bounds>>,
) -> impl IntoView {
    move || {
        if !debug.get() {
            return view!().into_view();
        };

        log::debug!("rendering {}", label);
        (bounds.get().iter())
            .map(|bounds| {
                view! {
                    <rect
                        x=bounds.left_x()
                        y=bounds.top_y()
                        width=bounds.width()
                        height=bounds.height()
                        fill="none"
                        stroke="red"
                        stroke-width=1
                    />
                }
            })
            .collect_view()
    }
}
