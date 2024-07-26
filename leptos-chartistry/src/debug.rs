use leptos::prelude::*;

use crate::bounds::Bounds;

#[component]
pub fn DebugRect(
    #[prop(into)] label: String,
    #[prop(into)] debug: Signal<bool>,
    #[prop(optional)] bounds: Vec<Signal<Bounds>>,
) -> impl IntoView {
    (move || {
        if !debug.get() {
            return view!().into_view();
        };

        log::debug!("rendering {}", label);
        bounds
            .clone()
            .into_iter()
            .map(|bounds| {
                view! {
                    <rect
                        x=move || bounds.get().left_x()
                        y=move || bounds.get().top_y()
                        width=move || bounds.get().width()
                        height=move || bounds.get().height()
                        fill="none"
                        stroke="red"
                        stroke-width=1
                    />
                }
            })
            .collect_view()
    })
    .into_view()
}
