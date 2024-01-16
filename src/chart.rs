use crate::{
    aspect_ratio::{AspectRatioCalc, CalcUsing},
    debug::DebugRect,
    inner::InnerLayout,
    layout::{HorizontalLayout, HorizontalVec, Layout, VerticalLayout, VerticalVec},
    overlay::tooltip::Tooltip,
    projection::Projection,
    series::{RenderData, UseData},
    state::{PreState, State},
    use_watched_node::{use_watched_node, UseWatchedNode},
    AspectRatio, Font, Padding, Position, Series,
};
use leptos::{html::Div, *};

#[component]
pub fn Chart<T, X, Y>(
    #[prop(into)] aspect_ratio: MaybeSignal<AspectRatio>,
    #[prop(into)] font: MaybeSignal<Font>,
    #[prop(into, optional)] debug: MaybeSignal<bool>,
    #[prop(into, optional)] padding: Option<MaybeSignal<Padding>>,

    #[prop(into, optional)] mut top: HorizontalVec<X>,
    #[prop(into, optional)] right: VerticalVec<Y>,
    #[prop(into, optional)] bottom: HorizontalVec<X>,
    #[prop(into, optional)] mut left: VerticalVec<Y>,

    #[prop(into, optional)] inner: Vec<InnerLayout<X, Y>>,
    #[prop(into, optional)] tooltip: Option<Tooltip<X, Y>>,

    //#[prop(into)] get_x: MaybeSignal<GetX>,
    //#[prop(into)] lines: MaybeSignal<Vec<Series<T, X, Y>>>,
    #[prop(into)] series: Series<T, X, Y>,
    #[prop(into, optional)] min_x: MaybeSignal<Option<X>>,
    #[prop(into, optional)] max_x: MaybeSignal<Option<X>>,
    #[prop(into, optional)] min_y: MaybeSignal<Option<Y>>,
    #[prop(into, optional)] max_y: MaybeSignal<Option<Y>>,
    #[prop(into)] data: Signal<Vec<T>>,
) -> impl IntoView
where
    T: 'static,
    X: Clone + PartialEq + PartialOrd + Position + 'static,
    Y: Clone + PartialEq + PartialOrd + Position + 'static,
{
    let root = create_node_ref::<Div>();
    let watch = use_watched_node(root);

    // Aspect ratio signal
    let have_dimensions = create_memo(move |_| watch.bounds.get().is_some());
    let width = create_memo(move |_| watch.bounds.get().unwrap_or_default().width());
    let height = create_memo(move |_| watch.bounds.get().unwrap_or_default().height());
    let calc = create_memo(move |_| match aspect_ratio.get().0 {
        CalcUsing::Env(calc) => calc.mk_signal(width, height),
        CalcUsing::Known(calc) => calc,
    });

    let debug = create_memo(move |_| debug.get());
    let padding = create_memo(move |_| {
        padding
            .map(|p| p.get())
            .unwrap_or_else(move || Padding::from(font.get().width()))
    });

    // Edges are added top to bottom, left to right. Layout compoeses inside out:
    top.reverse();
    left.reverse();

    // Build data
    let data = UseData::new(series, min_x, max_x, min_y, max_y, data);
    let pre = PreState::new(
        debug.into(),
        Signal::derive(move || font.get()),
        padding.into(),
        data,
    );

    view! {
        <div class="_chartistry" node_ref=root style="width: fit-content; height: fit-content; overflow: visible;">
            <DebugRect label="Chart" debug=debug />
            <Show when=move || have_dimensions.get() fallback=|| view!(<p>"Loading..."</p>)>
                <RenderChart
                    watch=watch.clone()
                    pre_state=pre.clone()
                    aspect_ratio=calc
                    top=top.as_slice()
                    right=right.as_slice()
                    bottom=bottom.as_slice()
                    left=left.as_slice()
                    inner=inner.clone()
                    tooltip=tooltip.clone()
                />
            </Show>
        </div>
    }
}

#[component]
fn RenderChart<'a, X, Y>(
    watch: UseWatchedNode,
    pre_state: PreState<X, Y>,
    aspect_ratio: Memo<AspectRatioCalc>,
    top: &'a [HorizontalLayout<X>],
    right: &'a [VerticalLayout<Y>],
    bottom: &'a [HorizontalLayout<X>],
    left: &'a [VerticalLayout<Y>],
    inner: Vec<InnerLayout<X, Y>>,
    tooltip: Option<Tooltip<X, Y>>,
) -> impl IntoView
where
    X: Clone + PartialEq + 'static,
    Y: Clone + PartialEq + 'static,
{
    let debug = pre_state.debug;

    // Compose edges
    let (layout, edges) = Layout::compose(top, right, bottom, left, aspect_ratio, &pre_state);

    // Finalise state
    let projection = {
        let inner = layout.inner;
        let position_range = pre_state.data.position_range;
        create_memo(move |_| Projection::new(inner.get(), position_range.get())).into()
    };
    let state = State::new(pre_state, &watch, layout, projection);

    // Render edges
    let edges = edges
        .into_iter()
        .map(|r| r.render(state.clone()))
        .collect_view();

    // Inner
    let inner = inner
        .into_iter()
        .map(|opt| opt.into_use(&state).render(state.clone()))
        .collect_view();

    let outer = state.layout.outer;
    view! {
        <svg
            width=move || format!("{}px", outer.get().width())
            height=move || format!("{}px", outer.get().height())
            viewBox=move || with!(|outer| format!("0 0 {} {}", outer.width(), outer.height()))
            style="display: block; overflow: visible;">
            <DebugRect label="RenderChart" debug=debug bounds=vec![outer.into()] />
            {inner}
            {edges}
            <RenderData state=state.clone() />
        </svg>
        {tooltip.map(|tooltip| view! {
            <Tooltip tooltip=tooltip state=state />
        })}
    }
}
