use chrono::prelude::*;
use leptos::*;
use leptos_chartistry::{colours::Colour, *};
use leptos_meta::{provide_meta_context, Style};
use std::str::FromStr;

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
const ALL_ASPECT_OPTIONS: &[AspectOption] = &[
    AspectOption::Outer,
    AspectOption::Inner,
    AspectOption::Environment,
];
const ALL_ASPECT_CALCS: &[AspectCalc] = &[AspectCalc::Ratio, AspectCalc::Width, AspectCalc::Height];
const ALL_HOVER_PLACEMENTS: &[HoverPlacement] = &[HoverPlacement::Hide, HoverPlacement::LeftCursor];
const ALL_SORT_BYS: &[SortBy] = &[SortBy::Lines, SortBy::Ascending, SortBy::Descending];

const CUSTOM_TS_FORMAT: &str = "🌟⭐🌟%+🌟⭐🌟";
const ALL_TS_FORMATS: &[TimestampFormat] = &[
    TimestampFormat::Short,
    TimestampFormat::Long,
    TimestampFormat::Strftime(CUSTOM_TS_FORMAT),
];
const ALL_PERIODS: &[Period] = &[
    Period::Year,
    Period::Month,
    Period::Day,
    Period::Hour,
    Period::Minute,
    Period::Second,
    Period::Millisecond,
    Period::Microsecond,
    Period::Nanosecond,
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
    XGridLine,
    YGridLine,
    XGuideLine,
    YGuideLine,
    Legend,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
enum AspectOption {
    #[default]
    Outer,
    Inner,
    Environment,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
enum AspectCalc {
    #[default]
    Ratio,
    Width,
    Height,
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
    provide_meta_context();

    // General options
    let (debug, set_debug) = create_signal(false);
    let padding = create_rw_signal(DEFAULT_FONT_WIDTH);
    let font_height = create_rw_signal(DEFAULT_FONT_HEIGHT);
    let font_width = create_rw_signal(DEFAULT_FONT_WIDTH);

    // Aspect ratio
    let aspect = create_rw_signal(AspectOption::default());
    let calc = create_rw_signal(AspectCalc::default());
    let width = create_rw_signal(800.0);
    let height = create_rw_signal(600.0);
    let ratio = create_rw_signal(1.0);

    // Data
    let (data, _) = create_signal(load_data());

    let (sine_name, set_sine_name) = create_signal("sine".to_string());
    let sine_width = create_rw_signal(1.0);
    let (cosine_name, set_cosine_name) = create_signal("cosine".to_string());
    let cosine_width = create_rw_signal(1.0);

    // X axis
    let x_ticks = TickLabels::aligned_floats();
    // Y axis
    let y_format = create_rw_signal(TimestampFormat::default());
    let mk_y_gen = move || {
        PeriodicTimestamps::from_periods(Period::all()).with_format(y_format.get_untracked())
    };
    let y_ticks = TickLabels::from_generator(mk_y_gen());
    let on_ts_format = {
        let y_ticks = y_ticks.clone();
        move |ev| {
            let format = parse_timestamp_format(&event_target_value(&ev));
            y_format.set(format);
            y_ticks.set_generator(mk_y_gen());
        }
    };

    // Series
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
    let bottom = Options::create_signal(vec![y_ticks]);
    let left = Options::create_signal(vec![x_ticks]);
    let inner: RwSignal<Options<InnerLayout<DateTime<Utc>, f64>>> = Options::create_signal(vec![
        AxisMarker::top_edge().into_inner_layout(),
        XGridLine::default().into_inner_layout(),
        YGridLine::default().into_inner_layout(),
        XGuideLine::default().into_inner_layout(),
        YGuideLine::default().into_inner_layout(),
    ]);
    let tooltip = Tooltip::default();
    let tooltip_card = tooltip.clone();

    view! {
        <Style>"
            ._chartistry {
                margin: 2em auto;
            }

            .outer {
                margin: 2em auto;
                display: flex;
                gap: 2em;
                flex-wrap: wrap;
                justify-content: center;
                align-items: flex-start;
            }

            fieldset {
                border: 1px solid #333;
                border-radius: 0.5em;
                padding: 1em;
                display: grid;
                grid-template-columns: max-content 1fr repeat(3, min-content);
                align-items: baseline;
            }

            fieldset > h3 {
                grid-column: 2 / -1;
                font-size: 100%;
                font-weight: normal;
                margin: 0;
                align-self: end;
                padding: 0.2em 0.5em;
            }

            fieldset > p {
                display: contents;
            }

            fieldset > p > :nth-child(1) { 
                grid-column: 1; 
                text-align: right; }
            fieldset > p > :nth-child(2) { 
                grid-column: 2;
                padding: 0.2em 0.5em; }
            fieldset > p > :nth-child(3) { grid-column: 3; }
            fieldset > p > :nth-child(4) { grid-column: 4; }
            fieldset > p > :nth-child(5) { grid-column: 5; }

            fieldset input[type=number] {
                width: 8ch;
            }

            fieldset input[type=color] {
                width: 6ch;
                height: 1.6em;
            }
        "</Style>

        {move || view!{
            <Chart
                aspect_ratio=derive_aspect_ratio(aspect, calc, width, height, ratio)
                font=Signal::derive(move || Font::new(font_height.get(), font_width.get()))
                debug=debug
                padding=Signal::derive(move || Padding::from(padding.get()))
                top=top.get().into_inner()
                right=right.get().into_inner()
                bottom=bottom.get().into_inner()
                left=left.get().into_inner()
                inner=inner.get().into_inner()
                tooltip=tooltip.clone()
                series=series.clone()
                data=data
            />
        }}

        <div class="outer">
            <fieldset class="options">
                <legend>"Chart options"</legend>
                <p>
                    <span>
                        <input type="checkbox" id="debug" checked=debug
                            on:input=move |ev| set_debug.set(event_target_checked(&ev)) />
                    </span>
                    <label for="debug">"Debug"</label>
                </p>
                <p>
                    <label for="aspect">"Aspect ratio"</label>
                    <AspectRatio aspect=aspect calc=calc width=width height=height ratio=ratio />
                </p>
                <p>
                    <label for="padding">"Padding"</label>
                    <span><StepInput id="padding" value=padding step="0.1" min="0.1" /></span>
                </p>

                <p>
                    <label for="font_height">"Font"</label>
                    <span style="grid-column: 2 / -1">
                        <StepInput id="font_width" value=font_width step="0.1" min="0.1" />
                        <small>" width"</small>
                        <br />
                        <StepInput id="font_height" value=font_height step="0.1" min="0.1" />
                        <small>" height"</small>
                    </span>
                </p>
            </fieldset>

            <fieldset class="data">
                <legend>"Data options"</legend>
                <p>
                    <label for="data"></label>
                    <span>
                        <select id="data">
                            <option>"Sine & cosine"</option>
                            <option>"TODO"</option>
                        </select>
                    </span>
                </p>
                <p>
                    <span>"X axis"</span>
                    <span>"Aligned floats"</span>
                </p>
                <p>
                    <span>"Y axis"</span>
                    <span>
                        <select on:change=on_ts_format>
                            <optgroup label="Timestamp format">
                                <For each=move || ALL_TS_FORMATS key=|opt| opt.to_string() let:format>
                                    <option selected=move || y_format.get() == *format>{format.to_string()}</option>
                                </For>
                            </optgroup>
                        </select>
                    </span>
                </p>

                <h3>"Sine"</h3>
                <p>
                    <label for="sine_name">"Name"</label>
                    <span>
                        <input type="text" id="sine_name" value=sine_name
                            on:input=move |ev| set_sine_name.set(event_target_value(&ev)) />
                    </span>
                </p>
                <p>
                    <label for="sine_width">"Width:"</label>
                    <span><StepInput id="sine_width" value=sine_width step="0.1" min="0.1" /></span>
                </p>

                <h3>"Cosine"</h3>
                <p>
                    <label for="cosine_name">"Name"</label>
                    <span>
                        <input type="text" value=cosine_name
                            on:input=move |ev| set_cosine_name.set(event_target_value(&ev)) />
                    </span>
                </p>
                <p>
                    <label for="cosine_width">"Width:"</label>
                    <span><StepInput id="cosine_width" value=cosine_width step="0.1" min="0.1" /></span>
                </p>
            </fieldset>

            <TooltipCard tooltip=tooltip_card />

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
                        <span>{detail(opt)}</span>
                        <span>{(i != 0).then_some(view!(<button on:click=on_move_up(i)>"↑"</button>))}</span>
                        <span>{(i != last).then_some(view!(<button on:click=on_move_down(i)>"↓"</button>))}</span>
                        <span><button on:click=on_remove(i)>"x"</button></span>
                    </p>
                }
            })
            .collect_view()
    });

    view! {
        <fieldset class=title.to_lowercase()>
            <legend>{title}</legend>
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
        </fieldset>
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

impl std::fmt::Display for AspectOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AspectOption::Outer => write!(f, "Outer"),
            AspectOption::Inner => write!(f, "Inner"),
            AspectOption::Environment => write!(f, "Environment"),
        }
    }
}

impl FromStr for AspectOption {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "outer" => Ok(AspectOption::Outer),
            "inner" => Ok(AspectOption::Inner),
            "environment" => Ok(AspectOption::Environment),
            _ => Err("unknown aspect ratio option"),
        }
    }
}

impl std::fmt::Display for AspectCalc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AspectCalc::Ratio => write!(f, "width / height = ratio"),
            AspectCalc::Width => write!(f, "height * ratio = width"),
            AspectCalc::Height => write!(f, "width / ratio = height"),
        }
    }
}

impl FromStr for AspectCalc {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "width / height = ratio" => Ok(AspectCalc::Ratio),
            "height * ratio = width" => Ok(AspectCalc::Width),
            "width / ratio = height" => Ok(AspectCalc::Height),
            _ => Err("unknown aspect ratio calculation"),
        }
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

impl<Tick> From<EdgeLayout<Tick>> for EdgeOption {
    fn from(layout: EdgeLayout<Tick>) -> Self {
        match layout {
            EdgeLayout::RotatedLabel(_) => Self::RotatedLabel,
            EdgeLayout::Legend(_) => Self::Legend,
            EdgeLayout::TickLabels(_) => Self::TickLabels,
            _ => EdgeOption::default(),
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

impl<X: Tick, Y: Tick> From<InnerLayout<X, Y>> for InnerOption {
    fn from(layout: InnerLayout<X, Y>) -> Self {
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
fn WidthInput(width: RwSignal<f64>) -> impl IntoView {
    view! {
        <label>"width:"<StepInput value=width step="0.1" min="0" /></label>
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
    let on_change = move |ev| {
        let min = event_target_value(&ev).parse().unwrap_or_default();
        value.set(min)
    };
    view! {
        <input
            type="number"
            id=id
            step=step
            min=min
            max=max
            value=value
            on:change=on_change />
    }
}

#[component]
fn SelectOption<Opt>(
    #[prop(into)] label: String,
    #[prop(into, optional)] id: Option<AttributeValue>,
    value: RwSignal<Opt>,
    all: &'static [Opt],
) -> impl IntoView
where
    Opt: Clone + FromStr + PartialEq + ToString + 'static,
{
    let on_change = move |ev| value.set(event_target_value(&ev).parse().unwrap_or(all[0].clone()));
    view! {
        <select id=id on:change=on_change>
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
        fn $fn(#[prop(into, optional)] id: Option<AttributeValue>, $input: RwSignal<$signal>) -> impl IntoView {
            view!(<SelectOption id=id label=$label value=$input all=$all />)
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
select_impl!(
    SelectHoverPlacement,
    "Placement",
    hover,
    HoverPlacement,
    ALL_HOVER_PLACEMENTS
);
select_impl!(SelectSortBy, "Order", sort_by, SortBy, ALL_SORT_BYS);
select_impl!(
    SelectAspectOption,
    "Aspect ratio",
    aspect,
    AspectOption,
    ALL_ASPECT_OPTIONS
);
select_impl!(
    SelectAspectCalc,
    "Calculation",
    calc,
    AspectCalc,
    ALL_ASPECT_CALCS
);

// TODO remove?
//select_impl!(SelectTsFormat, "Format", format, TsFormat, ALL_TS_FORMATS);
#[derive(Clone, Debug, Default, PartialEq)]
struct TsFormat(TimestampFormat);

impl TsFormat {
    pub const fn new(f: TimestampFormat) -> Self {
        Self(f)
    }
}

impl std::fmt::Display for TsFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for TsFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "Short" => Ok(TsFormat::new(TimestampFormat::Short)),
            "Long" => Ok(TsFormat::new(TimestampFormat::Long)),
            _ => {
                if s.starts_with("Custom: ") {
                    Ok(TsFormat::new(TimestampFormat::Strftime(CUSTOM_TS_FORMAT)))
                } else {
                    Err(format!("unknown timestamp format: `{}`", s))
                }
            }
        }
    }
}

fn parse_timestamp_format(s: &str) -> TimestampFormat {
    match s.to_lowercase().as_str() {
        "short" => TimestampFormat::Short,
        "long" => TimestampFormat::Long,
        _ => TimestampFormat::Strftime(CUSTOM_TS_FORMAT),
    }
}

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
        " "
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
        <label>"min width:"<StepInput value=ticks.min_chars step="1" min="0" /></label>
    }
}

#[component]
fn AxisMarkerOpts(marker: AxisMarker) -> impl IntoView {
    let on_arrow = move |ev| marker.arrow.set(event_target_checked(&ev));
    view! {
        <SelectColour colour=marker.colour />
        " "
        <SelectAxisPlacement placement=marker.placement />
        " "
        <WidthInput width=marker.width />
        " "
        <label>
            <input type="checkbox" checked=marker.arrow on:input=on_arrow />
            "arrow"
        </label>
    }
}

#[component]
fn InsetLegendOpts(legend: InsetLegend) -> impl IntoView {
    view! {
        <SelectOption label="Edge" value=legend.edge all=ALL_EDGES />
        " "
        <LegendOpts legend=legend.legend />
    }
}

#[component]
fn GridLineOpts(width: RwSignal<f64>, colour: RwSignal<Option<Colour>>) -> impl IntoView {
    view! {
        <SelectColour colour=colour />
        " "
        <WidthInput width=width />
    }
}

#[component]
fn GuideLineOpts(
    align: RwSignal<AlignOver>,
    width: RwSignal<f64>,
    colour: RwSignal<Option<Colour>>,
) -> impl IntoView {
    view! {
        <SelectColour colour=colour />
        " "
        <SelectAlignOver align=align />
        " "
        <WidthInput width=width />
    }
}

fn derive_aspect_ratio(
    aspect: RwSignal<AspectOption>,
    calc: RwSignal<AspectCalc>,
    width: RwSignal<f64>,
    height: RwSignal<f64>,
    ratio: RwSignal<f64>,
) -> Signal<AspectRatio> {
    Signal::derive(move || {
        let calc = calc.get();
        let width = width.get();
        let height = height.get();
        let ratio = ratio.get();
        use AspectCalc as Calc;
        match aspect.get() {
            AspectOption::Outer => match calc {
                Calc::Width => AspectRatio::outer_width(height, ratio),
                Calc::Height => AspectRatio::outer_height(width, ratio),
                Calc::Ratio => AspectRatio::outer_ratio(width, height),
            },
            AspectOption::Inner => match calc {
                Calc::Width => AspectRatio::inner_width(height, ratio),
                Calc::Height => AspectRatio::inner_height(width, ratio),
                Calc::Ratio => AspectRatio::inner_ratio(width, height),
            },
            AspectOption::Environment => match calc {
                Calc::Width => AspectRatio::environment_width(ratio),
                Calc::Height => AspectRatio::environment_height(ratio),
                Calc::Ratio => AspectRatio::environment(),
            },
        }
    })
}

#[component]
fn AspectRatio(
    aspect: RwSignal<AspectOption>,
    calc: RwSignal<AspectCalc>,
    width: RwSignal<f64>,
    height: RwSignal<f64>,
    ratio: RwSignal<f64>,
) -> impl IntoView {
    let on_left = move |ev| {
        let value = event_target_value(&ev).parse().unwrap_or_default();
        match calc.get() {
            AspectCalc::Ratio => width.set(value),
            AspectCalc::Width => height.set(value),
            AspectCalc::Height => width.set(value),
        }
    };
    let on_right = move |ev| {
        let value = event_target_value(&ev).parse().unwrap_or_default();
        match calc.get() {
            AspectCalc::Ratio => height.set(value),
            AspectCalc::Width => ratio.set(value),
            AspectCalc::Height => ratio.set(value),
        }
    };

    let left_value = move || match calc.get() {
        AspectCalc::Ratio => format!("{:.0}", width.get()),
        AspectCalc::Width => format!("{:.0}", height.get()),
        AspectCalc::Height => format!("{:.0}", width.get()),
    };
    let right_value = move || match calc.get() {
        AspectCalc::Ratio => format!("{:.0}", height.get()),
        AspectCalc::Width => format!("{:.2}", ratio.get()),
        AspectCalc::Height => format!("{:.2}", ratio.get()),
    };

    let calc_formula = move || match calc.get() {
        AspectCalc::Ratio => view! { " / " },
        AspectCalc::Width => view! { " * " },
        AspectCalc::Height => view! { " / " },
    };
    let result_value = move || match calc.get() {
        AspectCalc::Ratio => format!("{:.2} ratio", ratio.get()),
        AspectCalc::Width => format!("{:.0} width", width.get()),
        AspectCalc::Height => format!("{:.0} height", height.get()),
    };

    // When not used, our third var is just for show. Update it when the other two change.
    create_effect(move |_| match calc.get() {
        AspectCalc::Ratio => ratio.set(width.get() / height.get()),
        AspectCalc::Width => width.set(height.get() * ratio.get()),
        AspectCalc::Height => height.set(width.get() / ratio.get()),
    });

    view! {
        <span>
            <SelectAspectOption aspect=aspect />
            <br />
            <SelectAspectCalc calc=calc />
            <br />
            <input type="number" step=1 min=1 value=left_value on:change=on_left />
            {calc_formula}
            <input type="number" step=0.1 min=0.1 value=right_value on:change=on_right />
            " = " {result_value}
        </span>
    }
}

#[component]
fn TooltipCard<X: Tick, Y: Tick>(tooltip: Tooltip<X, Y>) -> impl IntoView {
    let Tooltip {
        placement,
        sort_by,
        skip_missing,
        table_margin,
        ..
    } = tooltip;

    view! {
        <fieldset class="tooltip">
            <legend>"Tooltip"</legend>
            <p>
                <label for="tooltip_hover">"Hover"</label>
                <span><SelectHoverPlacement id="tooltip_hover" hover=placement /></span>
            </p>
            <p>
                <label for="tooltip_sort">"Sort by"</label>
                <span><SelectSortBy id="tooltip_sort" sort_by=sort_by /></span>
            </p>
            <p>
                <span>
                    <input type="checkbox" id="skip_missing" checked=skip_missing
                        on:input=move |ev| skip_missing.set(event_target_checked(&ev)) />
                </span>
                <label for="skip_missing">"Skip missing?"</label>
            </p>
            <p>
                <span>
                    <input type="checkbox" id="table_margin" checked=table_margin
                        on:input=move |ev| table_margin.set(event_target_checked(&ev).then_some(DEFAULT_FONT_WIDTH)) />
                </span>
                <span>
                    <label for="table_margin">"Table margin?"</label>
                    {move || table_margin.get().map(move |margin| {
                        let on_change = move |ev| {
                            let value = event_target_value(&ev).parse().unwrap_or_default();
                            table_margin.set(Some(value))
                        };
                        view! {
                            <br />
                            <input type="number" step="0.1" min="0" value=margin
                                on:change=on_change />
                        }
                    })}
                </span>
            </p>
        </fieldset>
    }
}
