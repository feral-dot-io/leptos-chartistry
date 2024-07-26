use crate::{
    debug::DebugRect,
    series::{Snippet, UseY},
    state::State,
    Tick, TickLabels, AXIS_MARKER_COLOUR,
};
use leptos::prelude::*;
use std::cmp::{Ordering, Reverse};

/// Default gap distance from cursor to tooltip when shown.
pub const TOOLTIP_CURSOR_DISTANCE: f64 = 10.0;

/// Builds a mouse tooltip that shows X and Y values for the nearest data. Drawn in HTML as an overlay.
#[derive(Clone)]
pub struct Tooltip<X: 'static, Y: 'static> {
    /// Where the tooltip is placed when shown.
    pub placement: RwSignal<TooltipPlacement>,
    /// How the tooltip Y value table is sorted.
    pub sort_by: RwSignal<TooltipSortBy>,
    /// Gap distance from cursor to tooltip when shown.
    pub cursor_distance: RwSignal<f64>,
    /// If true, skips Y values that are `f64::NAN`.
    pub skip_missing: RwSignal<bool>,
    /// Whether to show X ticks. Default is true.
    // TODO: move to TickLabels
    pub show_x_ticks: RwSignal<bool>,
    /// X axis formatter.
    pub x_ticks: TickLabels<X>,
    /// Y axis formatter.
    pub y_ticks: TickLabels<Y>,
}

/// Where the tooltip is place when shown.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum TooltipPlacement {
    /// Does not show a tooltip.
    #[default]
    Hide,
    /// Shows the tooltip to the left of the cursor.
    LeftCursor,
}

/// How the tooltip Y value table is sorted.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum TooltipSortBy {
    /// Sorts by line name.
    #[default]
    Lines,
    /// Sorts by Y value in ascending order.
    Ascending,
    /// Sorts by Y value in descending order.
    Descending,
}

impl<X: Tick, Y: Tick> Tooltip<X, Y> {
    /// Creates a new tooltip with the given placement, X ticks, and Y ticks.
    pub fn new(
        placement: impl Into<TooltipPlacement>,
        x_ticks: impl Into<TickLabels<X>>,
        y_ticks: impl Into<TickLabels<Y>>,
    ) -> Self {
        Self {
            placement: RwSignal::new(placement.into()),
            x_ticks: x_ticks.into(),
            y_ticks: y_ticks.into(),
            ..Default::default()
        }
    }

    /// Creates a new tooltip with the given placement. Uses default X and Y ticks.
    pub fn from_placement(placement: impl Into<TooltipPlacement>) -> Self {
        Self::new(
            placement,
            TickLabels::from_generator(X::tooltip_generator()),
            TickLabels::from_generator(Y::tooltip_generator()),
        )
    }

    /// Creates a new tooltip left of the cursor. Uses default X and Y ticks.
    pub fn left_cursor() -> Self {
        Self::from_placement(TooltipPlacement::LeftCursor)
    }

    /// Sets the sort order of the Y value table.
    pub fn with_sort_by(self, sort_by: impl Into<TooltipSortBy>) -> Self {
        self.sort_by.set(sort_by.into());
        self
    }

    /// Sets the gap distance from cursor to tooltip when shown.
    pub fn with_cursor_distance(self, distance: impl Into<f64>) -> Self {
        self.cursor_distance.set(distance.into());
        self
    }

    /// Sets whether the tooltip should skip Y values that are `f64::NAN`.
    pub fn skip_missing(self, skip_missing: impl Into<bool>) -> Self {
        self.skip_missing.set(skip_missing.into());
        self
    }

    /// Sets whether to show X ticks.
    pub fn show_x_ticks(self, show_x_ticks: impl Into<bool>) -> Self {
        self.show_x_ticks.set(show_x_ticks.into());
        self
    }
}

impl<X: Tick, Y: Tick> Default for Tooltip<X, Y> {
    fn default() -> Self {
        Self {
            placement: RwSignal::default(),
            sort_by: RwSignal::default(),
            cursor_distance: create_rw_signal(TOOLTIP_CURSOR_DISTANCE),
            skip_missing: create_rw_signal(false),
            show_x_ticks: create_rw_signal(true),
            x_ticks: TickLabels::default(),
            y_ticks: TickLabels::default(),
        }
    }
}

impl TooltipSortBy {
    fn to_ord<Y: Tick>(y: &Option<Y>) -> Option<F64Ord> {
        y.as_ref().map(|y| F64Ord(y.position()))
    }

    fn sort_values<Y: Tick>(&self, values: &mut [(UseY, Option<Y>)]) {
        match self {
            TooltipSortBy::Lines => values.sort_by_key(|(line, _)| line.name.get()),
            TooltipSortBy::Ascending => values.sort_by_key(|(_, y)| Self::to_ord(y)),
            TooltipSortBy::Descending => values.sort_by_key(|(_, y)| Reverse(Self::to_ord(y))),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
struct F64Ord(f64);

impl PartialOrd for F64Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for F64Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Eq for F64Ord {}

impl std::fmt::Display for TooltipPlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TooltipPlacement::Hide => write!(f, "Hide"),
            TooltipPlacement::LeftCursor => write!(f, "Left cursor"),
        }
    }
}

impl std::str::FromStr for TooltipPlacement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hide" => Ok(TooltipPlacement::Hide),
            "left cursor" => Ok(TooltipPlacement::LeftCursor),
            _ => Err(format!("invalid TooltipPlacement: `{}`", s)),
        }
    }
}

impl std::fmt::Display for TooltipSortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TooltipSortBy::Lines => write!(f, "Lines"),
            TooltipSortBy::Ascending => write!(f, "Ascending"),
            TooltipSortBy::Descending => write!(f, "Descending"),
        }
    }
}

impl std::str::FromStr for TooltipSortBy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "lines" => Ok(TooltipSortBy::Lines),
            "ascending" => Ok(TooltipSortBy::Ascending),
            "descending" => Ok(TooltipSortBy::Descending),
            _ => Err(format!("invalid SortBy: `{}`", s)),
        }
    }
}

#[component]
pub(crate) fn Tooltip<X: Tick, Y: Tick>(
    tooltip: Tooltip<X, Y>,
    state: State<X, Y>,
) -> impl IntoView {
    let Tooltip {
        placement,
        sort_by,
        skip_missing,
        cursor_distance,
        show_x_ticks,
        x_ticks,
        y_ticks,
    } = tooltip;
    let debug = state.pre.debug;
    let font_height = state.pre.font_height;
    let font_width = state.pre.font_width;
    let padding = state.pre.padding;
    let inner = state.layout.inner;

    let x_body = {
        let nearest_data_x = state.pre.data.nearest_data_x(state.hover_position_x);
        let x_format = x_ticks.format;
        let avail_width = Signal::derive(move || with!(|inner| inner.width()));
        let x_ticks = x_ticks.generate_x(&state.pre, avail_width);
        move || {
            // Hide ticks?
            if !show_x_ticks.get() {
                return "".to_string();
            }
            let x_format = x_format.get();
            with!(|nearest_data_x, x_ticks| {
                nearest_data_x.as_ref().map_or_else(
                    || "no data".to_string(),
                    |x_value| (x_format)(x_value, x_ticks.state.as_ref()),
                )
            })
        }
    };

    let format_y_value = {
        let avail_height = Signal::derive(move || with!(|inner| inner.height()));
        let y_format = y_ticks.format;
        let y_ticks = y_ticks.generate_y(&state.pre, avail_height);
        move |y_value: Option<Y>| {
            let y_format = y_format.get();
            y_ticks.with(|y_ticks| {
                y_value.as_ref().map_or_else(
                    || "-".to_string(),
                    |y_value| (y_format)(y_value, y_ticks.state.as_ref()),
                )
            })
        }
    };

    let nearest_y_values = {
        let nearest_data_y = state.pre.data.nearest_data_y(state.hover_position_x);
        create_memo(move |_| {
            let mut y_values = nearest_data_y.get();
            // Skip missing?
            if skip_missing.get() {
                y_values = y_values
                    .into_iter()
                    .filter(|(_, y_value)| y_value.is_some())
                    .collect::<Vec<_>>()
            }
            // Sort values
            sort_by.get().sort_values(&mut y_values);
            y_values
        })
    };

    let nearest_data_y = move || {
        nearest_y_values
            .get()
            .into_iter()
            .map(|(line, y_value)| {
                let y_value = format_y_value(y_value);
                (line, y_value)
            })
            .collect::<Vec<_>>()
    };

    let series_tr = {
        let state = state.clone();
        move |(series, y_value): (UseY, String)| {
            view! {
                <tr>
                    <td><Snippet series=series state=state.clone() /></td>
                    <td
                        style="white-space: pre; font-family: monospace; text-align: right;"
                        style:padding-top=move || format!("{}px", font_height.get() / 4.0)
                        style:padding-left=move || format!("{}px", font_width.get())>
                        {y_value}
                    </td>
                </tr>
            }
        }
    };

    view! {
        <Show when=move || state.hover_inner.get() && placement.get() != TooltipPlacement::Hide>
            <DebugRect label="tooltip" debug=debug />
            <aside
                class="_chartistry_tooltip"
                style="position: absolute; z-index: 1; width: max-content; height: max-content; transform: translateY(-50%); background-color: #fff; white-space: pre; font-family: monospace;"
                style:border=format!("1px solid {}", AXIS_MARKER_COLOUR)
                style:top=move || format!("calc({}px)", state.mouse_page.get().1)
                style:right=move || format!("calc(100% - {}px + {}px)", state.mouse_page.get().0, cursor_distance.get())
                style:padding=move || padding.get().to_css_style()>
                <h2
                    style="margin: 0; text-align: center;"
                    style:font-size=move || format!("{}px", font_height.get())>
                    {x_body}
                </h2>
                <table
                    style="border-collapse: collapse; border-spacing: 0; margin: 0 0 0 auto; padding: 0;"
                    style:font-size=move || format!("{}px", font_height.get())>
                    <tbody>
                        <For
                            each=nearest_data_y
                            key=|(series, y_value)| (series.id, y_value.to_owned())
                            children=series_tr.clone()
                        />
                    </tbody>
                </table>
            </aside>
        </Show>
    }
}
