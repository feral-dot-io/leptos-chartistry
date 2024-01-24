use chrono::prelude::*;
use leptos::*;
use leptos_chartistry::*;

const DEFAULT_FONT_HEIGHT: f64 = 16.0;
const DEFAULT_FONT_WIDTH: f64 = 10.0;

#[derive(Clone)]
struct LayoutOptions(Vec<LayoutOption>);

#[derive(Clone)]
enum LayoutOption {
    Legend(Legend),
    RotatedLabel(RotatedLabel),
    TickLabels(TickLabels),
}

#[derive(Clone)]
struct Legend {
    anchor: RwSignal<Anchor>,
}

#[derive(Clone)]
struct RotatedLabel {
    anchor: RwSignal<Anchor>,
    text: RwSignal<String>,
}

#[derive(Clone)]
struct TickLabels {
    min_chars: RwSignal<usize>,
    format: RwSignal<TickFormat>,
}

#[derive(Clone, Default, PartialEq, Eq, Hash)]
enum TickFormat {
    #[default]
    Short,
    Long,
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
    let top = LayoutOptions::create_signal(vec![LayoutOption::rotated_label(
        "Hello and welcome to Chartistry!",
    )]);
    let right = LayoutOptions::create_signal(vec![LayoutOption::Legend(Legend::default())]);
    let bottom =
        LayoutOptions::create_signal(vec![LayoutOption::TickLabels(TickLabels::default())]);
    let left = LayoutOptions::create_signal(vec![LayoutOption::TickLabels(TickLabels::default())]);

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

        <ViewLayoutOptions title="Top" options=top />
        <ViewLayoutOptions title="Right" options=right />
        <ViewLayoutOptions title="Bottom" options=bottom />
        <ViewLayoutOptions title="Left" options=left />

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
                //inner=inner
                //tooltip=tooltip
                series=series.clone()
                data=data
            />
        }}
    }
}

#[component]
fn ViewLayoutOptions(title: &'static str, options: RwSignal<LayoutOptions>) -> impl IntoView {
    let (option, set_option) = create_signal(LayoutOption::default().as_label().to_string());
    let on_label_change = move |ev| set_option.set(event_target_value(&ev));

    let on_move_up = move |index| move |_| options.set(options.get().move_up(index));
    let on_move_down = move |index| move |_| options.set(options.get().move_down(index));
    let on_remove = move |index| move |_| options.set(options.get().remove(index));
    let on_new_line = move |_| {
        let opt = option.get().try_into().unwrap_or_default();
        options.set(options.get().add(opt));
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
                        <td>{opt.as_label()}</td>
                        <td>{opt.render_options()}</td>
                        <td>{(i != 0).then_some(view!(<button on:click=on_move_up(i)>"↑"</button>))}</td>
                        <td>{(i != last).then_some(view!(<button on:click=on_move_down(i)>"↓"</button>))}</td>
                        <td><button on:click=on_remove(i)>"x"</button></td>
                    </tr>
                }
            })
            .collect_view()
    });

    view! {
        <h2>{title}</h2>
        <table>
            <tbody>
                {move || existing_tr}
                <tr>
                    <td>
                        <select on:change=on_label_change>
                            <option>"Label"</option>
                            <option>"Legend"</option>
                            <option>"Ticks"</option>
                        </select>
                    </td>
                    <td colspan="4"><button on:click=on_new_line>"Add option"</button></td>
                </tr>
            </tbody>
        </table>
    }
}

impl LayoutOptions {
    pub fn new(mut opts: Vec<LayoutOption>) -> Self {
        if opts.is_empty() {
            opts.push(LayoutOption::default());
        }
        Self(opts)
    }

    pub fn create_signal(opts: Vec<LayoutOption>) -> RwSignal<LayoutOptions> {
        create_rw_signal(Self::new(opts))
    }

    pub fn add(mut self, opt: LayoutOption) -> Self {
        self.0.push(opt);
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

    pub fn into_inner(self) -> Vec<LayoutOption> {
        self.0
    }
}

impl LayoutOption {
    fn rotated_label(text: &'static str) -> Self {
        let label = RotatedLabel::default();
        label.text.set(text.to_string());
        Self::RotatedLabel(label)
    }

    fn as_label(&self) -> &'static str {
        match self {
            Self::RotatedLabel(_) => "Label",
            Self::Legend(_) => "Legend",
            Self::TickLabels(_) => "Ticks",
        }
    }

    fn render_options(self) -> impl IntoView {
        match self {
            Self::RotatedLabel(label) => view! {
                <RotatedLabelOpts label=label />
            }
            .into_view(),
            Self::Legend(legend) => view! {
                <LegendOpts legend=legend />
            }
            .into_view(),
            Self::TickLabels(labels) => view! {
                <TickLabelsOpts labels=labels />
            }
            .into_view(),
        }
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
fn TickLabelsOpts(labels: TickLabels) -> impl IntoView {
    let format = labels.format;
    let on_change = move |ev| {
        let new_value = event_target_value(&ev).try_into().unwrap_or_default();
        format.set(new_value);
    };
    view! {
        <select on:change=on_change>
            <optgroup label="Format">
                <option selected=move || format.get() == TickFormat::Short>"Short"</option>
                <option selected=move || format.get() == TickFormat::Long>"Long"</option>
            </optgroup>
        </select>
        " "
        <label>
            "Min chars: "
            <input
                type="number" step="1" min="0" value=labels.min_chars
                style="width: 8ch;"
                on:input=move |ev| labels.min_chars.set(event_target_value(&ev).parse().unwrap_or(0))
            />
        </label>
    }
}

#[component]
fn SelectAnchor(anchor: RwSignal<Anchor>) -> impl IntoView {
    let on_change = move |ev| anchor.set(event_target_value(&ev).into());
    view! {
        <select on:change=on_change>
            <optgroup label="Anchor">
                <option selected=move || anchor.get() == Anchor::Start>"Start"</option>
                <option selected=move || anchor.get() == Anchor::Middle>"Middle"</option>
                <option selected=move || anchor.get() == Anchor::End>"End"</option>
            </optgroup>
        </select>
    }
}

impl TryFrom<String> for LayoutOption {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "label" => Ok(LayoutOption::RotatedLabel(RotatedLabel::default())),
            "legend" => Ok(LayoutOption::Legend(Legend::default())),
            "ticks" => Ok(LayoutOption::TickLabels(TickLabels::default())),
            _ => Err("unknown layout option"),
        }
    }
}

impl Default for LayoutOption {
    fn default() -> Self {
        // Empty label has zero size so suitable for default
        Self::RotatedLabel(RotatedLabel::default())
    }
}

impl Default for Legend {
    fn default() -> Self {
        Self {
            anchor: create_rw_signal(Anchor::Middle),
        }
    }
}

impl Default for RotatedLabel {
    fn default() -> Self {
        Self {
            anchor: create_rw_signal(Anchor::Middle),
            text: create_rw_signal(String::new()),
        }
    }
}

impl Default for TickLabels {
    fn default() -> Self {
        Self {
            min_chars: create_rw_signal(0),
            format: create_rw_signal(TickFormat::Short),
        }
    }
}

impl TryFrom<String> for TickFormat {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "short" => Ok(Self::Short),
            "long" => Ok(Self::Long),
            _ => Err("unknown tick format"),
        }
    }
}

macro_rules! impl_to_axis {
    ($trait:ty, $fn:ident, $result:ty, $ticks:ident) => {
        impl $trait for LayoutOption {
            fn $fn(&self) -> $result {
                match self.clone() {
                    Self::Legend(legend) => leptos_chartistry::Legend::new(legend.anchor).into(),
                    Self::RotatedLabel(label) => {
                        leptos_chartistry::RotatedLabel::new(label.anchor, label.text).into()
                    }
                    Self::TickLabels(ticks) => leptos_chartistry::TickLabels::$ticks()
                        .set_min_chars(ticks.min_chars)
                        .set_format(move |s, t| match ticks.format.get() {
                            TickFormat::Short => s.short_format(t),
                            TickFormat::Long => s.long_format(t),
                        })
                        .into(),
                }
            }
        }
    };
}

impl_to_axis!(
    ToHorizontal<f64>,
    to_horizontal,
    HorizontalLayout<f64>,
    aligned_floats
);
impl_to_axis!(
    ToHorizontal<DateTime<Utc>>,
    to_horizontal,
    HorizontalLayout<DateTime<Utc>>,
    timestamps
);
impl_to_axis!(
    ToVertical<f64>,
    to_vertical,
    VerticalLayout<f64>,
    aligned_floats
);
impl_to_axis!(
    ToVertical<DateTime<Utc>>,
    to_vertical,
    VerticalLayout<DateTime<Utc>>,
    timestamps
);
