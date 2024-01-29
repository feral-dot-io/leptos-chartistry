use chrono::prelude::*;
use leptos::*;
use leptos_chartistry::{colours::Colour, *};
use leptos_meta::Style;
use std::str::FromStr;

const DEFAULT_ASPECT_RATIO: AspectRatio = AspectRatio::outer(800.0, 600.0);
const DEFAULT_FONT_HEIGHT: f64 = 16.0;
const DEFAULT_FONT_WIDTH: f64 = 10.0;

const ALL_ALIGN_OVER: &[AlignOver] = &[AlignOver::Mouse, AlignOver::Data];
const ALL_ANCHORS: &[Anchor] = &[Anchor::Start, Anchor::Middle, Anchor::End];
const ALL_AXIS_PLACEMENTS: &[AxisPlacement] = &[
    AxisPlacement::Top,
    AxisPlacement::Right,
    AxisPlacement::Bottom,
    AxisPlacement::Left,
    AxisPlacement::HorizontalZero,
    AxisPlacement::VerticalZero,
];
const ALL_EDGES: &[Edge] = &[Edge::Top, Edge::Right, Edge::Bottom, Edge::Left];

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
    XGridLine,
    YGridLine,
    XGuideLine,
    YGuideLine,
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
    let aspect = create_rw_signal(DEFAULT_ASPECT_RATIO);
    let padding = create_rw_signal(DEFAULT_FONT_WIDTH);
    let font_height = create_rw_signal(DEFAULT_FONT_HEIGHT);
    let font_width = create_rw_signal(DEFAULT_FONT_WIDTH);

    // Data
    let (data, _) = create_signal(load_data());
    let (sine_name, set_sine_name) = create_signal("sine".to_string());
    let sine_width = create_rw_signal(1.0);
    let (cosine_name, set_cosine_name) = create_signal("cosine".to_string());
    let cosine_width = create_rw_signal(1.0);
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
    let top: RwSignal<Options<EdgeLayout<_>>> = Options::create_signal(vec![RotatedLabel::middle(
        "Hello and welcome to Chartistry!",
    )]);
    let right = Options::create_signal(vec![Legend::middle()]);
    let bottom = Options::create_signal(vec![TickLabels::timestamps()]);
    let left = Options::create_signal(vec![TickLabels::aligned_floats()]);
    let inner: RwSignal<Options<InnerLayout<DateTime<Utc>, f64>>> = Options::create_signal(vec![
        AxisMarker::top_edge().into_inner_layout(),
        XGridLine::default().into_inner_layout(),
        YGridLine::default().into_inner_layout(),
        XGuideLine::default().into_inner_layout(),
        YGuideLine::default().into_inner_layout(),
    ]);

    view! {
        <Style>"
            ._chartistry {
                margin: 2em auto;
            }

            .outer {
                display: flex;
                gap: 2em;
                flex-wrap: wrap;
                align-items: flex-start;
            }

            .card {
                width: 16em;
                border: 1px solid #333;
                border-radius: 0.5em;
                padding: 1em;
                display: grid;
                grid-template-columns: max-content 1fr;
                gap: 0.5em;
            }

            .card h2 {
                grid-column: 1 / -1;
                font-size: 100%;
                margin: 0 auto;
            }

            .card h3 {
                grid-column: 2 / -1;
                font-size: 100%;
                margin: 0;
                align-self: end;
            }

            .card > p {
                display: contents;
            }

            .card > p > :first-child {
                text-align: right;
                grid-column: 1;
            }
            .card > p > :nth-child(2) {
                grid-column: 2;
            }

            .card input[type=number] {
                width: 8ch;
            }

            .card input[type=color] {
                width: 4ch;
            }
        "</Style>

        {move || view!{
            <Chart
                aspect_ratio=aspect
                font=Signal::derive(move || Font::new(font_height.get(), font_width.get()))
                debug=debug
                padding=Signal::derive(move || Padding::from(padding.get()))
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

        <div class="outer">
            <div class="card options">
                <h2>"Chart options"</h2>
                <p>
                    <span>
                        <input type="checkbox" id="debug" checked=debug
                            on:input=move |ev| set_debug.set(event_target_checked(&ev)) />
                    </span>
                    <label for="debug">"Debug"</label>
                </p>
                <p><label for="aspect">"Aspect"</label><span>"TODO"</span></p>
                <p>
                    <label for="padding">"Padding"</label>
                    <StepInput id="padding" value=padding step="0.1" min="0.1" />
                </p>

                <p>
                    <label for="font_height">"Font"</label>
                    <span style="grid-column: 2 / -1">
                        <StepInput id="font_width" value=font_width step="0.1" min="0.1" />
                        <small>" width"</small>
                        <StepInput id="font_height" value=font_height step="0.1" min="0.1" />
                        <small>" height"</small>
                    </span>
                </p>
            </div>

            <div class="card data">
                <h2>"Data options"</h2>
                <p>
                    <label for="data">""</label>
                    <select id="data">
                        <option>"Sine & cosine"</option>
                        <option>"TODO"</option>
                    </select>
                </p>

                <h3>"Sine"</h3>
                <p>
                    <label for="sine_name">"Name"</label>
                    <input type="text" id="sine_name" value=sine_name
                        on:input=move |ev| set_sine_name.set(event_target_value(&ev)) />
                </p>
                <p><StepLabel id="sine_width" value=sine_width step="0.1" min="0.1">"Width"</StepLabel></p>

                <h3>"Cosine"</h3>
                <p>
                    <label for="cosine_name">"Name"</label>
                    <input type="text" value=cosine_name
                        on:input=move |ev| set_cosine_name.set(event_target_value(&ev)) />
                </p>
                <p><StepLabel id="cosine_width" value=cosine_width step="0.1" min="0.1">"Width"</StepLabel></p>
            </div>

            <div class="card tooltip">
                <h2>"Tooltip"</h2>
            </div>

            <OptionsCard title="Inner" options=inner labels=ALL_INNER_OPTIONS detail=inner_layout_opts />
            <OptionsCard title="Top" options=top labels=ALL_EDGE_OPTIONS detail=edge_layout_opts />
            <OptionsCard title="Bottom" options=bottom labels=ALL_EDGE_OPTIONS detail=edge_layout_opts />
            <OptionsCard title="Left" options=left labels=ALL_EDGE_OPTIONS detail=edge_layout_opts />
            <OptionsCard title="Right" options=right labels=ALL_EDGE_OPTIONS detail=edge_layout_opts />
        </div>
    }
}

#[component]
fn OptionsCard<Full, FullView, FullIV, Label>(
    title: &'static str,
    options: RwSignal<Options<Full>>,
    labels: &'static [Label],
    detail: FullView,
) -> impl IntoView
where
    Full: Clone + From<Label> + 'static,
    FullView: Fn(Full) -> FullIV + 'static,
    FullIV: IntoView,
    Label: Copy + Default + From<Full> + FromStr + PartialEq + ToString + 'static,
{
    let (option, set_option) = create_signal(Label::default());
    let on_label_change =
        move |ev| set_option.set(event_target_value(&ev).parse().unwrap_or_default());

    let on_move_up = move |index| move |_| options.set(options.get().move_up(index));
    let on_move_down = move |index| move |_| options.set(options.get().move_down(index));
    let on_remove = move |index| move |_| options.set(options.get().remove(index));
    let on_new_line = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        options.set(options.get().add(option.get()));
    };

    let existing_rows = Signal::derive(move || {
        let options = options.get().into_inner();
        let last = options.len().saturating_sub(1);
        options
            .into_iter()
            .enumerate()
            .map(|(i, opt)| {
                view! {
                    <p>
                        <span>{Label::from(opt.clone()).to_string()}</span>
                        <span>
                            {detail(opt)}
                            " "
                            {(i != 0).then_some(view!(<button on:click=on_move_up(i)>"↑"</button>))}
                            {(i != last).then_some(view!(<button on:click=on_move_down(i)>"↓"</button>))}
                            <button on:click=on_remove(i)>"x"</button>
                        </span>
                    </p>
                }
            })
            .collect_view()
    });

    view! {
        <div class=format!("card {}", title.to_lowercase())>
            <h2>{title}</h2>
            {move || existing_rows}
            <p>
                <span></span>
                <span>
                    <select on:change=on_label_change>
                        <For each=move || labels key=|label| label.to_string() let:label>
                            <option selected=move || option.get() == *label>{label.to_string()}</option>
                        </For>
                    </select>
                    " "
                    <button on:click=on_new_line>"Add option"</button>
                </span>
            </p>
        </div>
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

const ALL_EDGE_OPTIONS: &[EdgeOption] = &[
    EdgeOption::RotatedLabel,
    EdgeOption::Legend,
    EdgeOption::TickLabels,
];

impl std::fmt::Display for EdgeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EdgeOption::RotatedLabel => write!(f, "Label"),
            EdgeOption::Legend => write!(f, "Legend"),
            EdgeOption::TickLabels => write!(f, "Ticks"),
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

impl<Tick> From<EdgeLayout<Tick>> for EdgeOption {
    fn from(layout: EdgeLayout<Tick>) -> Self {
        // TODO
        (&layout).into()
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

const ALL_INNER_OPTIONS: &[InnerOption] = &[
    InnerOption::AxisMarker,
    InnerOption::XGridLine,
    InnerOption::YGridLine,
    InnerOption::XGuideLine,
    InnerOption::YGuideLine,
    InnerOption::Legend,
];

impl std::fmt::Display for InnerOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InnerOption::AxisMarker => write!(f, "Axis marker"),
            InnerOption::XGridLine => write!(f, "X grid line"),
            InnerOption::YGridLine => write!(f, "Y grid line"),
            InnerOption::XGuideLine => write!(f, "X guide line"),
            InnerOption::YGuideLine => write!(f, "Y guide line"),
            InnerOption::Legend => write!(f, "Legend"),
        }
    }
}

impl FromStr for InnerOption {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "axis marker" => Ok(InnerOption::AxisMarker),
            "x grid line" => Ok(InnerOption::XGridLine),
            "y grid line" => Ok(InnerOption::YGridLine),
            "x guide line" => Ok(InnerOption::XGuideLine),
            "y guide line" => Ok(InnerOption::YGuideLine),
            "legend" => Ok(InnerOption::Legend),
            _ => Err("unknown inner option"),
        }
    }
}

impl<X: Tick, Y: Tick> From<&InnerLayout<X, Y>> for InnerOption {
    fn from(layout: &InnerLayout<X, Y>) -> Self {
        match layout {
            InnerLayout::AxisMarker(_) => Self::AxisMarker,
            InnerLayout::XGridLine(_) => Self::XGridLine,
            InnerLayout::YGridLine(_) => Self::YGridLine,
            InnerLayout::XGuideLine(_) => Self::XGuideLine,
            InnerLayout::YGuideLine(_) => Self::YGuideLine,
            InnerLayout::Legend(_) => Self::Legend,
            _ => InnerOption::default(),
        }
    }
}

impl<X: Tick, Y: Tick> From<InnerLayout<X, Y>> for InnerOption {
    fn from(layout: InnerLayout<X, Y>) -> Self {
        // TODO
        (&layout).into()
    }
}

impl<X: Tick, Y: Tick> From<InnerOption> for InnerLayout<X, Y> {
    fn from(option: InnerOption) -> Self {
        match option {
            InnerOption::AxisMarker => AxisMarker::top_edge().into(),
            InnerOption::XGridLine => XGridLine::default().into(),
            InnerOption::YGridLine => YGridLine::default().into(),
            InnerOption::XGuideLine => XGuideLine::default().into(),
            InnerOption::YGuideLine => YGuideLine::default().into(),
            InnerOption::Legend => InsetLegend::top_left().into(),
        }
    }
}

fn edge_layout_opts<Tick: 'static>(option: EdgeLayout<Tick>) -> impl IntoView {
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

fn inner_layout_opts<X: Tick, Y: Tick>(option: InnerLayout<X, Y>) -> impl IntoView {
    match option {
        InnerLayout::AxisMarker(marker) => view! {
            <AxisMarkerOpts marker=marker />
        }
        .into_view(),
        InnerLayout::Legend(legend) => view! {
            <InsetLegendOpts legend=legend />
        }
        .into_view(),
        InnerLayout::XGridLine(line) => view! {
            <GridLineOpts width=line.width colour=line.colour />
        }
        .into_view(),
        InnerLayout::YGridLine(line) => view! {
            <GridLineOpts width=line.width colour=line.colour />
        }
        .into_view(),
        InnerLayout::XGuideLine(line) => view! {
            <GuideLineOpts align=line.align width=line.width colour=line.colour />
        }
        .into_view(),
        InnerLayout::YGuideLine(line) => view! {
            <GuideLineOpts align=line.align width=line.width colour=line.colour />
        }
        .into_view(),
        _ => ().into_view(),
    }
}

#[component]
fn StepLabel<T: Clone + Default + IntoAttribute + FromStr + 'static>(
    value: RwSignal<T>,
    #[prop(into, optional)] id: String, // TODO
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
        <label for=id.clone()>{children()}</label>
        <input
            type="number"
            id=id
            step=step
            min=min
            max=max
            value=value
            on:input=on_input />
    }
}

#[component]
fn StepInput<T: Clone + Default + IntoAttribute + FromStr + 'static>(
    value: RwSignal<T>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into)] step: String,
    #[prop(into, optional)] min: Option<String>,
    #[prop(into, optional)] max: Option<String>,
) -> impl IntoView {
    let on_input = move |ev| {
        let min = event_target_value(&ev).parse().unwrap_or_default();
        value.set(min)
    };
    view! {
        <span>
            <input
                type="number"
                id=id
                step=step
                min=min
                max=max
                value=value
                on:input=on_input />
        </span>
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

select_impl!(
    SelectAlignOver,
    "Align over",
    align,
    AlignOver,
    ALL_ALIGN_OVER
);
select_impl!(SelectAnchor, "Anchor", anchor, Anchor, ALL_ANCHORS);
select_impl!(
    SelectAxisPlacement,
    "Placement",
    placement,
    AxisPlacement,
    ALL_AXIS_PLACEMENTS
);
select_impl!(SelectEdge, "Edge", edge, Edge, ALL_EDGES);

#[component]
fn SelectColour(colour: RwSignal<Option<Colour>>) -> impl IntoView {
    let value = move || colour.get().map(|c| c.to_string()).unwrap_or_default();
    let on_change = move |ev| {
        //let new = event_target_value(&ev).parse().ok();
        //colour.set(new);
    };
    view! {
        <input type="color" value=value on:input=on_change />
    }
}

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
        // TODO
        <StepLabel value=ticks.min_chars step="1" min="0">"min chars:"</StepLabel>
    }
}

#[component]
fn AxisMarkerOpts(marker: AxisMarker) -> impl IntoView {
    let on_arrow = move |ev| marker.arrow.set(event_target_checked(&ev));
    view! {
        <SelectAxisPlacement placement=marker.placement />
        <SelectColour colour=marker.colour />
        " "
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

#[component]
fn GridLineOpts(width: RwSignal<f64>, colour: RwSignal<Option<Colour>>) -> impl IntoView {
    view! {
        <StepLabel value=width step="0.1" min="0.1">"width:"</StepLabel>
        <SelectColour colour=colour />
    }
}

#[component]
fn GuideLineOpts(
    align: RwSignal<AlignOver>,
    width: RwSignal<f64>,
    colour: RwSignal<Option<Colour>>,
) -> impl IntoView {
    view! {
        <SelectAlignOver align=align />
        <StepLabel value=width step="0.1" min="0.1">"width:"</StepLabel>
        <SelectColour colour=colour />
    }
}
