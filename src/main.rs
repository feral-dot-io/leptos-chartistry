use chrono::prelude::*;
use leptos::*;
use leptos_chartistry::*;
use std::str::FromStr;

const DEFAULT_FONT_HEIGHT: f64 = 16.0;
const DEFAULT_FONT_WIDTH: f64 = 10.0;

const ALL_ANCHORS: &[Anchor] = &[Anchor::Start, Anchor::Middle, Anchor::End];
const ALL_EDGES: &[Edge] = &[Edge::Top, Edge::Right, Edge::Bottom, Edge::Left];
const ALL_AXIS_PLACEMENTS: &[AxisPlacement] = &[
    AxisPlacement::Top,
    AxisPlacement::Right,
    AxisPlacement::Bottom,
    AxisPlacement::Left,
    AxisPlacement::HorizontalZero,
    AxisPlacement::VerticalZero,
];

#[derive(Clone)]
struct Options<Opt>(Vec<Opt>);

#[derive(Copy, Clone, Debug, Default, PartialEq)]
enum EdgeOption {
    #[default]
    RotatedLabel,
    Legend,
    TickLabels,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
enum InnerOption {
    #[default]
    AxisMarker,
    //HorizontalGridLine,
    //VerticalGridLine,
    //XGuideLine,
    //YGuideLine,
    Legend,
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[derive(Clone, Copy, PartialEq)]
pub struct Wave {
    x: f64,
    sine: f64,
    cosine: f64,
}

fn load_data() -> Vec<Wave> {
    const SCALE: f64 = 1.0;
    let mut data = Vec::new();
    for i in 0..1000 {
        let x = i as f64 / 1000.0 * std::f64::consts::PI * 2.0 * 2.0;
        let sine = x.sin() * SCALE + 1.0;
        let cosine = x.cos() * SCALE + 1.0;
        data.push(Wave { x, sine, cosine });
    }
    data
}

pub fn f64_to_dt(at: f64) -> DateTime<Utc> {
    let nsecs = (at.fract() * 1_000_000_000.0).round() as u32;
    Utc.timestamp_opt(at as i64, nsecs).unwrap()
}

#[component]
pub fn App() -> impl IntoView {
    let (debug, set_debug) = create_signal(false);

    // Font
    let (font_height, set_font_height) = create_signal(DEFAULT_FONT_HEIGHT);
    let (font_width, set_font_width) = create_signal(DEFAULT_FONT_WIDTH);
    let font = Signal::derive(move || Font::new(font_height.get(), font_width.get()));

    // Padding
    let (padding_value, set_padding) = create_signal(DEFAULT_FONT_WIDTH);
    let padding = Signal::derive(move || Padding::from(padding_value.get()));

    // Data
    let (data, _) = create_signal(load_data());
    let (sine_name, set_sine_name) = create_signal("sine".to_string());
    let (sine_width, set_sine_width) = create_signal(1.0);
    let (cosine_name, set_cosine_name) = create_signal("cosine".to_string());
    let (cosine_width, set_cosine_width) = create_signal(1.0);
    let series = Series::new(&|w: &Wave| f64_to_dt(w.x))
        .line(
            Line::new(&|w: &Wave| w.sine)
                .set_name(sine_name)
                .set_width(sine_width),
        )
        .line(
            Line::new(&|w: &Wave| w.cosine)
                .set_name(cosine_name)
                .set_width(cosine_width),
        );

    // Layout options
    let top = Options::create_signal(vec![RotatedLabel::middle(
        "Hello and welcome to Chartistry!",
    )]);
    let right = Options::create_signal(vec![Legend::middle()]);
    let bottom = Options::create_signal(vec![TickLabels::timestamps()]);
    let left = Options::create_signal(vec![TickLabels::aligned_floats()]);
    let inner: RwSignal<Options<InnerLayout<DateTime<Utc>, f64>>> =
        Options::create_signal(vec![AxisMarker::top_edge()]);

    view! {
        <h1>"Chartistry"</h1>
        <form>
            <p>
                <label>
                    <input type="checkbox" checked=debug on:input=move |ev| set_debug.set(event_target_checked(&ev)) />
                    "Debug"
                </label>
            </p>
            <p>
                <label>
                    "Font height"
                    <input type="number" step="0.1" min="0.1" value=font_height on:input=move |ev| set_font_height.set(event_target_value(&ev).parse().unwrap_or(DEFAULT_FONT_HEIGHT)) />
                </label>
            </p>
            <p>
                <label>
                    "Font width"
                    <input type="number" step="0.1" min="0.1" value=font_width on:input=move |ev| set_font_width.set(event_target_value(&ev).parse().unwrap_or(DEFAULT_FONT_WIDTH)) />
                </label>
            </p>
            <p>
                <label>
                    "Sine"
                    <input type="text" value=sine_name on:input=move |ev| set_sine_name.set(event_target_value(&ev)) />
                </label>
            </p>
            <p>
                <label>
                    "Sine width"
                    <input type="number" step="0.1" min="0.1" value=sine_width on:input=move |ev| set_sine_width.set(event_target_value(&ev).parse().unwrap_or(1.0)) />
                </label>
            </p>
            <p>
                <label>
                    "Cosine"
                    <input type="text" value=cosine_name on:input=move |ev| set_cosine_name.set(event_target_value(&ev)) />
                </label>
            </p>
            <p>
                <label>
                    "Cosine width"
                    <input type="number" step="0.1" min="0.1" value=cosine_width on:input=move |ev| set_cosine_width.set(event_target_value(&ev).parse().unwrap_or(1.0)) />
                </label>
            </p>
            <p>
                <label>
                    "Padding"
                    <input type="number" step="0.1" min="0.1" value=padding_value on:input=move |ev| set_padding.set(event_target_value(&ev).parse().unwrap_or(DEFAULT_FONT_WIDTH)) />
                </label>
            </p>
        </form>

        <ViewEdgeLayoutOpts title="Top" options=top />
        <ViewEdgeLayoutOpts title="Right" options=right />
        <ViewEdgeLayoutOpts title="Bottom" options=bottom />
        <ViewEdgeLayoutOpts title="Left" options=left />
        <ViewInnerOpts options=inner />

        {move || view!{
            <Chart
                aspect_ratio=AspectRatio::outer_width(1100.0, 0.6)
                font=font
                debug=debug
                padding=padding
                top=top.get().into_inner()
                right=right.get().into_inner()
                bottom=bottom.get().into_inner()
                left=left.get().into_inner()
                inner=inner.get().into_inner()
                //tooltip=tooltip
                series=series.clone()
                data=data
            />
        }}
    }
}

#[component]
fn ViewEdgeLayoutOpts<Tick: crate::Tick>(
    title: &'static str,
    options: RwSignal<Options<EdgeLayout<Tick>>>,
) -> impl IntoView {
    let (option, set_option) = create_signal(EdgeOption::default());
    let on_label_change =
        move |ev| set_option.set(event_target_value(&ev).parse().unwrap_or_default());

    let on_move_up = move |index| move |_| options.set(options.get().move_up(index));
    let on_move_down = move |index| move |_| options.set(options.get().move_down(index));
    let on_remove = move |index| move |_| options.set(options.get().remove(index));
    let on_new_line = move |_| {
        options.set(options.get().add(option.get()));
    };

    let existing_tr = Signal::derive(move || {
        let options = options.get().into_inner();
        let last = options.len().saturating_sub(1);
        options
            .into_iter()
            .enumerate()
            .map(|(i, opt)| {
                view! {
                    <tr>
                        <td>{EdgeOption::from(&opt).to_string()}</td>
                        <td><EdgeLayoutOpts option=opt /></td>
                        <td>{(i != 0).then_some(view!(<button on:click=on_move_up(i)>"↑"</button>))}</td>
                        <td>{(i != last).then_some(view!(<button on:click=on_move_down(i)>"↓"</button>))}</td>
                        <td><button on:click=on_remove(i)>"x"</button></td>
                    </tr>
                }
            })
            .collect_view()
    });

    view! {
        <h2>{title}" options"</h2>
        <table>
            <tbody>
                {move || existing_tr}
                <tr>
                    <td>
                        <select on:change=on_label_change>
                            <For each=EdgeOption::all key=|opt| opt.to_string() let:opt>
                                <option selected=move || option.get() == *opt>{opt.to_string()}</option>
                            </For>
                        </select>
                    </td>
                    <td colspan="4"><button on:click=on_new_line>"Add option"</button></td>
                </tr>
            </tbody>
        </table>
    }
}

#[component]
fn ViewInnerOpts<X: Tick, Y: Tick>(options: RwSignal<Options<InnerLayout<X, Y>>>) -> impl IntoView {
    let (option, set_option) = create_signal(InnerOption::default());
    let on_label_change =
        move |ev| set_option.set(event_target_value(&ev).parse().unwrap_or_default());

    let on_move_up = move |index| move |_| options.set(options.get().move_up(index));
    let on_move_down = move |index| move |_| options.set(options.get().move_down(index));
    let on_remove = move |index| move |_| options.set(options.get().remove(index));
    let on_new_line = move |_| {
        options.set(options.get().add(option.get()));
    };

    let existing_tr = Signal::derive(move || {
        let options = options.get().into_inner();
        let last = options.len().saturating_sub(1);
        options
            .into_iter()
            .enumerate()
            .map(|(i, opt)| {
                view! {
                    <tr>
                        <td>{InnerOption::from(&opt).to_string()}</td>
                        <td><InnerLayoutOpts option=opt /></td>
                        <td>{(i != 0).then_some(view!(<button on:click=on_move_up(i)>"↑"</button>))}</td>
                        <td>{(i != last).then_some(view!(<button on:click=on_move_down(i)>"↓"</button>))}</td>
                        <td><button on:click=on_remove(i)>"x"</button></td>
                    </tr>
                }
            })
            .collect_view()
    });

    view! {
        <h2>"Inner options"</h2>
        <table>
            <tbody>
                {move || existing_tr}
                <tr>
                    <td>
                        <select on:change=on_label_change>
                            <For each=InnerOption::all key=|opt| opt.to_string() let:opt>
                                <option selected=move || option.get() == *opt>{opt.to_string()}</option>
                            </For>
                        </select>
                    </td>
                    <td colspan="4"><button on:click=on_new_line>"Add option"</button></td>
                </tr>
            </tbody>
        </table>
    }
}

impl<Opt> Options<Opt> {
    fn create_signal<IO>(opts: impl IntoIterator<Item = IO>) -> RwSignal<Self>
    where
        IO: Into<Opt>,
    {
        let opts = opts.into_iter().map(Into::into).collect();
        create_rw_signal(Self(opts))
    }

    pub fn add(mut self, opt: impl Into<Opt>) -> Self {
        self.0.push(opt.into());
        self
    }

    pub fn move_up(mut self, index: usize) -> Self {
        if index > 0 {
            self.0.swap(index, index - 1);
        }
        self
    }

    pub fn move_down(mut self, index: usize) -> Self {
        if index < self.0.len() - 1 {
            self.0.swap(index, index + 1);
        }
        self
    }

    pub fn remove(mut self, index: usize) -> Self {
        if index < self.0.len() {
            self.0.remove(index);
        }
        self
    }

    pub fn into_inner(self) -> Vec<Opt> {
        self.0
    }
}

impl EdgeOption {
    pub fn all() -> &'static [Self] {
        &[Self::RotatedLabel, Self::Legend, Self::TickLabels]
    }
}

impl InnerOption {
    pub fn all() -> &'static [Self] {
        &[Self::AxisMarker, Self::Legend]
    }
}

impl std::fmt::Display for EdgeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EdgeOption::RotatedLabel => write!(f, "Label"),
            EdgeOption::Legend => write!(f, "Legend"),
            EdgeOption::TickLabels => write!(f, "Ticks"),
        }
    }
}

impl std::fmt::Display for InnerOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InnerOption::AxisMarker => write!(f, "Axis marker"),
            InnerOption::Legend => write!(f, "Legend"),
        }
    }
}

impl FromStr for EdgeOption {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "label" => Ok(EdgeOption::RotatedLabel),
            "legend" => Ok(EdgeOption::Legend),
            "ticks" => Ok(EdgeOption::TickLabels),
            _ => Err("unknown edge layout option"),
        }
    }
}

impl FromStr for InnerOption {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "axis marker" => Ok(InnerOption::AxisMarker),
            "legend" => Ok(InnerOption::Legend),
            _ => Err("unknown inner option"),
        }
    }
}

impl<Tick> From<&EdgeLayout<Tick>> for EdgeOption {
    fn from(layout: &EdgeLayout<Tick>) -> Self {
        match layout {
            EdgeLayout::RotatedLabel(_) => Self::RotatedLabel,
            EdgeLayout::Legend(_) => Self::Legend,
            EdgeLayout::TickLabels(_) => Self::TickLabels,
            _ => EdgeOption::default(),
        }
    }
}

impl<X: Tick, Y: Tick> From<&InnerLayout<X, Y>> for InnerOption {
    fn from(layout: &InnerLayout<X, Y>) -> Self {
        match layout {
            InnerLayout::AxisMarker(_) => Self::AxisMarker,
            InnerLayout::Legend(_) => Self::Legend,
            _ => InnerOption::default(),
        }
    }
}

impl<Tick: crate::Tick> From<EdgeOption> for EdgeLayout<Tick> {
    fn from(option: EdgeOption) -> Self {
        match option {
            EdgeOption::RotatedLabel => Self::RotatedLabel(RotatedLabel::middle("")),
            EdgeOption::Legend => Self::Legend(Legend::middle()),
            EdgeOption::TickLabels => Self::TickLabels(TickLabels::default()),
        }
    }
}

impl<X: Tick, Y: Tick> From<InnerOption> for InnerLayout<X, Y> {
    fn from(option: InnerOption) -> Self {
        match option {
            InnerOption::AxisMarker => AxisMarker::top_edge().into(),
            InnerOption::Legend => InsetLegend::top_left().into(),
        }
    }
}

#[component]
fn EdgeLayoutOpts<Tick: 'static>(option: EdgeLayout<Tick>) -> impl IntoView {
    match option {
        EdgeLayout::RotatedLabel(label) => view! {
            <RotatedLabelOpts label=label />
        }
        .into_view(),
        EdgeLayout::Legend(legend) => view! {
            <LegendOpts legend=legend />
        }
        .into_view(),
        EdgeLayout::TickLabels(ticks) => view! {
            <TickLabelsOpts ticks=ticks />
        }
        .into_view(),
        _ => ().into_view(),
    }
}

#[component]
fn InnerLayoutOpts<X: Tick, Y: Tick>(option: InnerLayout<X, Y>) -> impl IntoView {
    match option {
        InnerLayout::AxisMarker(marker) => view! {
            <AxisMarkerOpts marker=marker />
        }
        .into_view(),
        InnerLayout::Legend(legend) => view! {
            <InsetLegendOpts legend=legend />
        }
        .into_view(),
        _ => ().into_view(),
    }
}

#[component]
fn StepLabel<T: Clone + Default + IntoAttribute + FromStr + 'static>(
    value: RwSignal<T>,
    #[prop(into)] step: String,
    #[prop(into, optional)] min: Option<String>,
    #[prop(into, optional)] max: Option<String>,
    children: Children,
) -> impl IntoView {
    let on_input = move |ev| {
        let min = event_target_value(&ev).parse().unwrap_or_default();
        value.set(min)
    };
    view! {
        <label>
            {children()}
            ", "
            <input
                type="number"
                style="width: 8ch;"
                step=step
                min=min
                max=max
                value=value
                on:input=on_input />
        </label>
    }
}

#[component]
fn SelectOption<Opt>(
    #[prop(into)] label: String,
    value: RwSignal<Opt>,
    all: &'static [Opt],
) -> impl IntoView
where
    Opt: Copy + FromStr + PartialEq + ToString + 'static,
{
    let on_change = move |ev| value.set(event_target_value(&ev).parse().unwrap_or(all[0]));
    view! {
        <select on:change=on_change>
            <optgroup label=label>
                <For each=move || all key=|opt| opt.to_string() let:opt>
                    <option selected=move || value.get() == *opt>{opt.to_string()}</option>
                </For>
            </optgroup>
        </select>
    }
}

macro_rules! select_impl {
    ($fn:ident, $label:literal, $input:ident, $signal:ty, $all:ident) => {
        #[component]
        fn $fn($input: RwSignal<$signal>) -> impl IntoView {
            view!(<SelectOption label=$label value=$input all=$all />)
        }
    };
}

select_impl!(SelectAnchor, "Anchor", anchor, Anchor, ALL_ANCHORS);
select_impl!(SelectEdge, "Edge", edge, Edge, ALL_EDGES);
select_impl!(
    SelectAxisPlacement,
    "Placement",
    placement,
    AxisPlacement,
    ALL_AXIS_PLACEMENTS
);

#[component]
fn RotatedLabelOpts(label: RotatedLabel) -> impl IntoView {
    view! {
        <SelectAnchor anchor=label.anchor />
        <input type="text" value=label.text on:input=move |ev| label.text.set(event_target_value(&ev)) />
    }
}

#[component]
fn LegendOpts(legend: Legend) -> impl IntoView {
    view! {
        <SelectAnchor anchor=legend.anchor />
    }
}

#[component]
fn TickLabelsOpts<Tick: 'static>(ticks: TickLabels<Tick>) -> impl IntoView {
    view! {
        <StepLabel value=ticks.min_chars step="1" min="0">"Min chars:"</StepLabel>
    }
}

#[component]
fn AxisMarkerOpts(marker: AxisMarker) -> impl IntoView {
    let on_arrow = move |ev| marker.arrow.set(event_target_checked(&ev));
    view! {
        <SelectAxisPlacement placement=marker.placement />
        "colour: TODO"
        ", "
        <label>
            <input type="checkbox" checked=marker.arrow on:input=on_arrow />
            "arrow"
        </label>
        ", "
        <StepLabel value=marker.width step="0.1" min="0.1">"width:"</StepLabel>
    }
}

#[component]
fn InsetLegendOpts(legend: InsetLegend) -> impl IntoView {
    view! {
        <SelectOption label="Edge" value=legend.edge all=ALL_EDGES />
        <LegendOpts legend=legend.legend />
    }
}
