use chrono::prelude::*;
use leptos::*;
use leptos_chartistry::*;
use leptos_meta::{provide_meta_context, Style};
use std::str::FromStr;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 400.0;

const FONT_HEIGHT: f64 = 16.0;
const FONT_WIDTH: f64 = 10.0;

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
const ALL_TOOLTIP_PLACEMENTS: &[TooltipPlacement] =
    &[TooltipPlacement::Hide, TooltipPlacement::LeftCursor];
const ALL_SORT_BYS: &[TooltipSortBy] = &[
    TooltipSortBy::Lines,
    TooltipSortBy::Ascending,
    TooltipSortBy::Descending,
];

const JS_TIMESTAMP_FMT: &str = "%FT%R";

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
    Outer,
    #[default]
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
    x: DateTime<Local>,
    sine: f64,
    cosine: f64,
}

const Y2K: f64 = 946_684_800f64;
const ONE_DAY: f64 = 86_400f64;

fn load_data() -> Vec<Wave> {
    let mut data = Vec::new();
    for deg in 0..(360 * 3) {
        let rad = deg as f64 * std::f64::consts::PI / 180.0;
        data.push(Wave {
            x: f64_to_dt(Y2K + ONE_DAY * deg as f64),
            sine: rad.sin(),
            cosine: rad.cos(),
        });
    }
    data
}

pub fn f64_to_dt(at: f64) -> DateTime<Local> {
    let nsecs = (at.fract() * 1_000_000_000.0).round() as u32;
    Local.timestamp_opt(at as i64, nsecs).unwrap()
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // General options
    let (debug, set_debug) = create_signal(false);
    let padding = create_rw_signal(FONT_WIDTH);
    let font_height = create_rw_signal(FONT_HEIGHT);
    let font_width = create_rw_signal(FONT_WIDTH);

    // Aspect ratio
    let aspect = create_rw_signal(AspectOption::default());
    let calc = create_rw_signal(AspectCalc::default());
    let width = create_rw_signal(WIDTH);
    let height = create_rw_signal(HEIGHT);
    let ratio = create_rw_signal(1.0);

    // Data
    let (data, _) = create_signal(load_data());
    let sine = Line::new(|w: &Wave| w.sine).with_name("sine");
    let cosine = Line::new(|w: &Wave| w.cosine).with_name("cosine");

    // Axis
    let x_periods = Timestamps::from_periods(Period::all());
    let x_ticks = TickLabels::new(x_periods.clone());
    let y_ticks = TickLabels::aligned_floats();

    // Series
    let series = Series::new(|w: &Wave| w.x)
        .line(sine.clone())
        .line(cosine.clone());
    let (min_x, max_x) = (series.min_x, series.max_x);
    let (min_y, max_y) = (series.min_y, series.max_y);
    let series_colours = series.colours;
    let series_len = series.len();

    // Tooltip
    let tooltip = Tooltip::new(
        TooltipPlacement::default(),
        x_periods.with_strftime("%c"),
        y_ticks.clone(),
    );
    let tooltip_card = tooltip.clone();

    // Range
    let on_datetime_change = move |sig: RwSignal<Option<DateTime<_>>>| {
        move |ev| {
            let new_value =
                NaiveDateTime::parse_from_str(&event_target_value(&ev), JS_TIMESTAMP_FMT)
                    .ok()
                    .and_then(|dt| dt.and_local_timezone(Local).latest());
            sig.set(new_value)
        }
    };
    let mk_range_ts = move |sig: RwSignal<Option<DateTime<_>>>| {
        move || {
            sig.get()
                .map(|v| v.format(JS_TIMESTAMP_FMT).to_string())
                .unwrap_or_default()
        }
    };

    // Layout options
    let top: RwSignal<Options<EdgeLayout<_>>> = Options::create_signal(vec![RotatedLabel::middle(
        "Hello and welcome to Chartistry!",
    )]);
    let right = Options::create_signal(vec![Legend::middle()]);
    let bottom = Options::create_signal(vec![
        x_ticks.clone().into_edge(),
        RotatedLabel::middle("This demo shows most of the available options. Edit things below...")
            .into_edge(),
    ]);
    let left = Options::create_signal(vec![y_ticks.clone().into_edge()]);
    let inner: RwSignal<Options<InnerLayout<DateTime<_>, f64>>> = Options::create_signal(vec![
        AxisMarker::horizontal_zero().into_inner(),
        AxisMarker::left_edge().into_inner(),
        XGridLine::new(x_ticks).into_inner(),
        YGridLine::new(y_ticks).into_inner(),
        XGuideLine::default().into_inner(),
        YGuideLine::default().into_inner(),
    ]);

    view! {
        <Style>"
            html { 
                min-height: 100%;
                background: repeating-radial-gradient(circle at top, #eee, #12a5ed);
            }

            ._chartistry {
                background-color: #fff;
                border: 1px solid #000;
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
                background-color: #fff;
                border: 1px solid #000;
                border-radius: 0.5em;
                padding: 0.5em 1em 1em 1em;
                display: grid;
                grid-template-columns: max-content 1fr repeat(3, min-content);
                align-items: baseline;
            }
            fieldset > legend {
                background-color: #fff;
                border-bottom: 0.2em solid #fc7089;
                padding: 0.2em;
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

            .periods {
                display: grid;
                grid-template-columns: 1fr 1fr 1fr;
            }
        "</Style>

        {move || view!{
            <Chart
                aspect_ratio=derive_aspect_ratio(aspect, calc, width, height, ratio)
                font_height=font_height
                font_width=font_width
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
                    <span><label for="debug">"Debug"</label></span>
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
                        <StepInput id="font_height" value=font_height step="0.1" min="0.1" />
                        <small>" height"</small>
                        <br />
                        <StepInput id="font_width" value=font_width step="0.1" min="0.1" />
                        <small>" width"</small>
                    </span>
                </p>
            </fieldset>

            <fieldset class="series">
                <legend>"Series options"</legend>
                <p>
                    <label for="sine_name">"Sine"</label>
                    <span>
                        <input type="text" id="sine_name" value=sine.name
                            on:input=move |ev| sine.name.set(event_target_value(&ev)) />
                    </span>
                </p>
                <p>
                    <label for="sine_width">"Width"</label>
                    <span><StepInput id="sine_width" value=sine.width step="0.1" min="0.1" /></span>
                </p>

                <p>
                    <label for="cosine_name">"Cosine"</label>
                    <span>
                        <input type="text" id="cosine_name" value=cosine.name
                            on:input=move |ev| cosine.name.set(event_target_value(&ev)) />
                    </span>
                </p>
                <p>
                    <label for="cosine_width">"Width"</label>
                    <span><StepInput id="cosine_width" value=cosine.width step="0.1" min="0.1" /></span>
                </p>
                <p>
                    <label for="series_colours">"Colours"</label>
                    <span>
                        <SelectColourScheme colours=series_colours lines=series_len />
                    </span>
                </p>
            </fieldset>

            <fieldset class="series">
                <legend>"Axis options"</legend>
                <p><span>"Y axis"</span><span>"Aligned floats"</span></p>
                <p>
                    <label for="min_y">"Range"</label>
                    <span>
                        <input type="number" id="min_y" step="0.1"
                            value=move || min_y.get().map(|v| v.to_string()).unwrap_or_default()
                            on:change=move |ev| min_y.set(event_target_value(&ev).parse().ok()) />
                        " to "
                        <input type="number" id="max_y" step="0.1"
                            value=move || max_y.get().map(|v| v.to_string()).unwrap_or_default()
                            on:change=move |ev| max_y.set(event_target_value(&ev).parse().ok()) />
                    </span>
                </p>
                <p>
                    <span>"X axis"</span>
                    <span class="periods">"Timestamps"</span>
                </p>
                <p>
                    <label for="min_x">"Range"</label>
                    <span>
                        <input type="datetime-local" id="min_x"
                            value=mk_range_ts(min_x)
                            on:change=on_datetime_change(min_x) />
                        " to "
                        <br />
                        <input type="datetime-local" id="max_x"
                            value=mk_range_ts(max_x)
                            on:change=on_datetime_change(max_x) />
                    </span>
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
    SelectTooltipPlacement,
    "Placement",
    hover,
    TooltipPlacement,
    ALL_TOOLTIP_PLACEMENTS
);
select_impl!(SelectSortBy, "Order", sort_by, TooltipSortBy, ALL_SORT_BYS);
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

#[component]
fn SelectColour(colour: RwSignal<Colour>) -> impl IntoView {
    let on_change = move |ev| {
        if let Ok(value) = event_target_value(&ev).parse() {
            colour.set(value);
        }
    };
    view! {
        <input type="color" value=move || colour.get().to_string() on:input=on_change />
    }
}

#[component]
fn SelectColourScheme(colours: RwSignal<ColourScheme>, lines: usize) -> impl IntoView {
    (0..lines)
        .map(|line| {
            let on_change = move |ev| {
                if let Ok(colour) = event_target_value(&ev).parse() {
                    let mut new_colours = colours.get();
                    new_colours.set_by_index(line, colour);
                    colours.set(new_colours);
                }
            };
            view! {
                <input type="color"
                    value=move || colours.get().by_index(line).to_string()
                    on:input=on_change />
            }
        })
        .collect_view()
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
fn GridLineOpts(width: RwSignal<f64>, colour: RwSignal<Colour>) -> impl IntoView {
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
    colour: RwSignal<Colour>,
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
        cursor_distance,
        ..
    } = tooltip;

    view! {
        <fieldset class="tooltip">
            <legend>"Tooltip"</legend>
            <p>
                <label for="tooltip_hover">"Hover"</label>
                <span><SelectTooltipPlacement id="tooltip_hover" hover=placement /></span>
            </p>
            <p>
                <label for="tooltip_sort">"Sort by"</label>
                <span><SelectSortBy id="tooltip_sort" sort_by=sort_by /></span>
            </p>
            <p>
                <label for="tooltip_distance">"Cursor distance"</label>
                <span><StepInput id="tooltip_distance" value=cursor_distance step="0.1" min="0" /></span>
            </p>
            <p>
                <span>
                    <input type="checkbox" id="skip_missing" checked=skip_missing
                        on:input=move |ev| skip_missing.set(event_target_checked(&ev)) />
                </span>
                <label for="skip_missing">"Skip missing?"</label>
            </p>
        </fieldset>
    }
}
