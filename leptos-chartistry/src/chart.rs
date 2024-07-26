use crate::{
    aspect_ratio::KnownAspectRatio,
    debug::DebugRect,
    inner::InnerLayout,
    layout::{EdgeLayout, Layout},
    overlay::tooltip::Tooltip,
    projection::Projection,
    series::{RenderData, UseData},
    state::{PreState, State},
    use_watched_node::{use_watched_node, UseWatchedNode},
    AspectRatio, Padding, Series, Tick,
};
use leptos::{html::Div, *};

pub const FONT_HEIGHT: f64 = 16.0;
pub const FONT_WIDTH: f64 = 10.0;

/// Renders an SVG chart.
///
/// Check the required and optional props list near the bottom for a quick overview.
///
/// ## Examples
///
/// There is an [large, assorted list of examples](https://feral-dot-io.github.io/leptos-chartistry/examples.html) available. See below for a quick [line chart example](https://feral-dot-io.github.io/leptos-chartistry/examples.html#line-chart):
/// ```rust
/// use leptos::prelude::*;
/// use leptos_chartistry::*;
///
/// # use chrono::prelude::*;
/// # struct MyData { x: DateTime<Utc>, y1: f64, y2: f64 }
/// # fn load_data() -> Signal<Vec<MyData>> { Signal::default() }
///
/// # #[component]
/// # fn SimpleChartComponent() -> impl IntoView {
/// let data: Signal<Vec<MyData>> = load_data(/* pull data from a resource */);
/// view! {
///     <Chart
///         // Sets the width and height
///         aspect_ratio=AspectRatio::from_outer_ratio(600.0, 300.0)
///
///         // Decorate our chart
///         top=RotatedLabel::middle("My garden")
///         left=TickLabels::aligned_floats()
///         right=Legend::end()
///         bottom=TickLabels::timestamps()
///         inner=[
///             AxisMarker::left_edge().into_inner(),
///             AxisMarker::bottom_edge().into_inner(),
///             XGridLine::default().into_inner(),
///             YGridLine::default().into_inner(),
///             XGuideLine::over_data().into_inner(),
///             YGuideLine::over_mouse().into_inner(),
///         ]
///         tooltip=Tooltip::left_cursor()
///
///         // Describe the data
///         series=Series::new(|data: &MyData| data.x)
///             .line(Line::new(|data: &MyData| data.y1).with_name("butterflies"))
///             .line(Line::new(|data: &MyData| data.y2).with_name("dragonflies"))
///         data=data
///     />
/// }
/// # }
/// ```
///
/// ## Layout props
///
/// The chart is built up from layout components. Each edge has a `top`, `right`, `bottom`, and `left` prop while inside the chart has the `inner` prop. These layout props follow the builder pattern where you'll create a component, configure it to your liking, and then call [IntoEdge](crate::IntoEdge) or [IntoInner](crate::IntoInner) to get an edge layout or inner layout respectively.
///
/// Here's an example of building a [TickLabels](crate::TickLabels) component, setting the minimum number of characters to 5, and then converting it for use to an edge layout:
///
/// ```rust
/// # use leptos_chartistry::*;
/// TickLabels::aligned_floats().with_min_chars(5).into_edge();
/// ```
///
/// ### Fine-grained reactivity
///
/// You'll also have access to this API via [`RwSignals`](https://docs.rs/leptos/latest/leptos/struct.RwSignal.html) allowing you to make changes after the chart creation. This enables fine-grained reactivity.
///
/// ```rust
/// # use leptos::prelude::*;
/// # use leptos_chartistry::*;
/// # #[component]
/// # fn FindGrainedComponent() -> impl IntoView {
/// let y_ticks = TickLabels::aligned_floats().with_min_chars(5);
/// // Copy the min_chars RwSignal
/// let y_ticks_min_chars = y_ticks.min_chars;
/// // Later on, you can change it on the fly:
/// # view! {
/// <button on:click=move |_| y_ticks_min_chars.set(10)>"Set min chars to 10"</button>
/// # } }
/// ```
///
/// ## Next steps
///
/// See the props below for more details. Copy and paste [examples](https://feral-dot-io.github.io/leptos-chartistry/examples.html) to get going quickly.
#[component]
pub fn Chart<T: 'static, X: Tick, Y: Tick>(
    /// Determines the width and height of the chart. Charts with a different aspect ratio and axis ranges are difficult to compare. You're encouraged to pick an [inner aspect ratio](AspectRatio::inner_ratio) while the closest to a "don't think about it" approach is to automatically [use the environment](AspectRatio::environment).
    ///
    /// See [AspectRatio](AspectRatio) for a detailed explanation.
    #[prop(into)]
    aspect_ratio: MaybeSignal<AspectRatio>,

    /// The height of the font used in the chart. Passed to [SVG text](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/text). Default is 16.
    #[prop(into, optional)]
    font_height: Option<MaybeSignal<f64>>,

    /// The width must be the exact width of a monospaced character in the font used. Along with font_height, it is used to calculate the dimensions of text. These dimensions are then fed into layout composition to render the chart. The default is 10.
    #[prop(into, optional)]
    font_width: Option<MaybeSignal<f64>>,

    /// Debug mode. If enabled shows lines around components and prints render info to the console. Useful for getting an idea of how the chart is rendering itself. Below is an example of how you might use it in development. Default is false.
    ///
    /// ```rust
    /// # use leptos::prelude::*;
    /// # use leptos_chartistry::*;
    /// # #[component]
    /// # fn DebugComponent() -> impl IntoView {
    /// let (debug, set_debug) = create_signal(true);
    /// view! {
    ///     <p>
    ///         <label>
    ///             <input type="checkbox" input type="checkbox"
    ///                 on:input=move |ev| set_debug.set(event_target_checked(&ev)) />
    ///             " Toggle debug mode"
    ///         </label>
    ///     </p>
    ///     <Chart
    ///         // Toggle debug on the fly
    ///         debug=debug
    ///         // ... fill in the rest of your props
    /// #       aspect_ratio=AspectRatio::from_outer_ratio(600.0, 300.0)
    /// #       series=Series::new(|(x, _): &(f64, f64)| *x).line(|(_, y): &(f64, f64)| *y)
    /// #       data=Signal::default()
    ///     />
    /// }
    /// # }
    /// ```
    #[prop(into, optional)]
    debug: MaybeSignal<bool>,

    /// Padding adds spacing around chart components. Default is the font width.
    #[prop(into, optional)]
    padding: Option<MaybeSignal<Padding>>,

    /// Top edge components. See [IntoEdge](crate::IntoEdge) for details. Default is none.
    #[prop(into, optional)]
    top: Vec<EdgeLayout<X>>,
    /// Right edge components. See [IntoEdge](crate::IntoEdge) for details. Default is none.
    #[prop(into, optional)]
    right: Vec<EdgeLayout<Y>>,
    /// Bottom edge components. See [IntoEdge](crate::IntoEdge) for details. Default is none.
    #[prop(into, optional)]
    bottom: Vec<EdgeLayout<X>>,
    /// Left edge components. See [IntoEdge](crate::IntoEdge) for details. Default is none.
    #[prop(into, optional)]
    left: Vec<EdgeLayout<Y>>,

    /// Inner chart area components. Does not render lines -- use [Series] for that. See [IntoInner](crate::IntoInner) for details. Default is none.
    #[prop(into, optional)]
    inner: Vec<InnerLayout<X, Y>>,
    /// Tooltip to show on mouse hover. See [Tooltip](crate::Tooltip) for details. Default is hidden.
    #[prop(into, optional)]
    tooltip: Tooltip<X, Y>,

    /// Series to render. Maps `T` to lines, bars, etc. See [Series] for details.
    #[prop(into)]
    series: Series<T, X, Y>,
    /// Data to render. Must be sorted.
    #[prop(into)]
    data: Signal<Vec<T>>,
) -> impl IntoView {
    let root = create_node_ref::<Div>();
    let watch = use_watched_node(root);

    // Aspect ratio signal
    let have_dimensions = create_memo(move |_| watch.bounds.get().is_some());
    let width = create_memo(move |_| watch.bounds.get().unwrap_or_default().width());
    let height = create_memo(move |_| watch.bounds.get().unwrap_or_default().height());
    let calc = AspectRatio::known_signal(aspect_ratio.clone(), width, height);
    let env_size = move || {
        if aspect_ratio.get().is_env() {
            "100%"
        } else {
            "fit-content"
        }
    };

    let debug = create_memo(move |_| debug.get());
    let font_height = create_memo(move |_| font_height.map(|f| f.get()).unwrap_or(FONT_HEIGHT));
    let font_width = create_memo(move |_| font_width.map(|f| f.get()).unwrap_or(FONT_WIDTH));
    let padding = create_memo(move |_| {
        padding
            .map(|p| p.get())
            .unwrap_or_else(move || Padding::from(font_width.get()))
    });

    // Edges are added top to bottom, left to right. Layout compoeses inside out:
    let mut top = top;
    let mut left = left;
    top.reverse();
    left.reverse();

    // Build data
    let data = UseData::new(series, data);
    let pre = PreState::new(debug.into(), font_height, font_width, padding.into(), data);

    view! {
        <div
            node_ref=root
            class="_chartistry"
            style:width=env_size.clone()
            style:height=env_size
            style="overflow: visible;">
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
fn RenderChart<'a, X: Tick, Y: Tick>(
    watch: UseWatchedNode,
    pre_state: PreState<X, Y>,
    aspect_ratio: Memo<KnownAspectRatio>,
    top: &'a [EdgeLayout<X>],
    right: &'a [EdgeLayout<Y>],
    bottom: &'a [EdgeLayout<X>],
    left: &'a [EdgeLayout<Y>],
    inner: Vec<InnerLayout<X, Y>>,
    tooltip: Tooltip<X, Y>,
) -> impl IntoView {
    let debug = pre_state.debug;

    // Compose edges
    let (layout, edges) = Layout::compose(top, right, bottom, left, aspect_ratio, &pre_state);

    // Finalise state
    let projection = {
        let range_x = pre_state.data.range_x;
        let range_y = pre_state.data.range_y;
        let includes_bars = pre_state.data.includes_bars;
        create_memo(move |_| {
            let mut inner = layout.inner.get();
            // If we include bars, shrink the sides by half the width of X
            if includes_bars.get() {
                let half = layout.x_width.get() / 2.0;
                inner = inner.shrink(0.0, half, 0.0, half);
            }

            Projection::new(inner, range_x.get().positions(), range_y.get().positions())
        })
        .into()
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
            <CommonDefs />
            {inner}
            {edges}
            <RenderData state=state.clone() />
        </svg>
        <Tooltip tooltip=tooltip state=state />
    }
}

#[component]
fn CommonDefs() -> impl IntoView {
    view! {
        <defs>
            <marker
                id="marker_axis_arrow"
                markerUnits="strokeWidth"
                markerWidth=7
                markerHeight=8
                refX=0
                refY=4
                orient="auto">
                <path d="M0,0 L0,8 L7,4 z" fill="context-stroke" />
            </marker>
        </defs>
    }
}
