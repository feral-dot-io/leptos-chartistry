use crate::{
    debug::DebugRect,
    layout::Layout,
    series::{Snippet, UseLine},
    state::{PreState, State},
    Tick, TickLabels,
};
use leptos::*;
use std::{
    borrow::Borrow,
    cmp::{Ordering, Reverse},
};

pub const TOOLTIP_CURSOR_DISTANCE: f64 = 10.0;

#[derive(Clone)]
pub struct Tooltip<X: 'static, Y: 'static> {
    pub placement: RwSignal<HoverPlacement>,
    pub skip_missing: RwSignal<bool>,
    pub cursor_distance: RwSignal<f64>,
    pub sort_by: RwSignal<SortBy>,
    pub x_ticks: TickLabels<X>,
    pub y_ticks: TickLabels<Y>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum HoverPlacement {
    Hide,
    #[default]
    LeftCursor,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum SortBy {
    #[default]
    Lines,
    Ascending,
    Descending,
}

impl<X: Tick, Y: Tick> Tooltip<X, Y> {
    pub fn new(
        placement: impl Into<HoverPlacement>,
        x_ticks: impl Borrow<TickLabels<X>>,
        y_ticks: impl Borrow<TickLabels<Y>>,
    ) -> Self {
        Self {
            placement: RwSignal::new(placement.into()),
            x_ticks: x_ticks.borrow().clone(),
            y_ticks: y_ticks.borrow().clone(),
            ..Default::default()
        }
    }

    pub fn from_placement(placement: impl Into<HoverPlacement>) -> Self {
        Self::new(placement, TickLabels::default(), TickLabels::default())
    }

    pub fn left_cursor() -> Self {
        Self::from_placement(HoverPlacement::LeftCursor)
    }

    pub fn with_cursor_distance(self, distance: impl Into<f64>) -> Self {
        self.cursor_distance.set(distance.into());
        self
    }
}

impl<X: Tick, Y: Tick> Default for Tooltip<X, Y> {
    fn default() -> Self {
        Self {
            placement: RwSignal::default(),
            skip_missing: false.into(),
            cursor_distance: create_rw_signal(TOOLTIP_CURSOR_DISTANCE),
            sort_by: RwSignal::default(),
            x_ticks: TickLabels::default(),
            y_ticks: TickLabels::default(),
        }
    }
}

impl SortBy {
    fn to_ord<Y: Tick>(y: &Option<Y>) -> Option<F64Ord> {
        y.as_ref().map(|y| F64Ord(y.position()))
    }

    fn sort_values<Y: Tick>(&self, values: &mut [(UseLine, Option<Y>)]) {
        match self {
            SortBy::Lines => values.sort_by_key(|(line, _)| line.name.get()),
            SortBy::Ascending => values.sort_by_key(|(_, y)| Self::to_ord(y)),
            SortBy::Descending => values.sort_by_key(|(_, y)| Reverse(Self::to_ord(y))),
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

impl std::fmt::Display for HoverPlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HoverPlacement::Hide => write!(f, "Hide"),
            HoverPlacement::LeftCursor => write!(f, "Left cursor"),
        }
    }
}

impl std::str::FromStr for HoverPlacement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hide" => Ok(HoverPlacement::Hide),
            "left cursor" => Ok(HoverPlacement::LeftCursor),
            _ => Err(format!("invalid HoverPlacement: `{}`", s)),
        }
    }
}

impl std::fmt::Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortBy::Lines => write!(f, "Lines"),
            SortBy::Ascending => write!(f, "Ascending"),
            SortBy::Descending => write!(f, "Descending"),
        }
    }
}

impl std::str::FromStr for SortBy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "lines" => Ok(SortBy::Lines),
            "ascending" => Ok(SortBy::Ascending),
            "descending" => Ok(SortBy::Descending),
            _ => Err(format!("invalid SortBy: `{}`", s)),
        }
    }
}

#[component]
pub fn Tooltip<X: Tick, Y: Tick>(tooltip: Tooltip<X, Y>, state: State<X, Y>) -> impl IntoView {
    let Tooltip {
        placement,
        sort_by,
        skip_missing,
        cursor_distance,
        x_ticks,
        y_ticks,
    } = tooltip;
    let PreState {
        debug,
        font,
        padding,
        ..
    } = state.pre;
    let State {
        layout: Layout { inner, .. },
        mouse_page,
        hover_inner,
        nearest_data_x,
        ..
    } = state;

    let avail_width = Signal::derive(move || with!(|inner| inner.width()));
    let avail_height = Signal::derive(move || with!(|inner| inner.height()));
    let x_ticks = x_ticks.generate_x(&state.pre, avail_width);
    let y_ticks = y_ticks.generate_y(&state.pre, avail_height);

    let x_body = move || {
        with!(|nearest_data_x, x_ticks| {
            nearest_data_x.as_ref().map_or_else(
                || "no data".to_string(),
                |x_value| x_ticks.state.format(x_value),
            )
        })
    };

    let format_y_value = move |y_value: Option<Y>| {
        y_ticks.with(|y_ticks| {
            y_value
                .as_ref()
                .map_or_else(|| "-".to_string(), |y_value| y_ticks.state.format(y_value))
        })
    };

    let nearest_y_values = {
        let nearest_data_y = state.nearest_data_y;
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
        move |(series, y_value): (UseLine, String)| {
            view! {
                <tr>
                    <td><Snippet series=series state=state.clone() /></td>
                    <td
                        style="white-space: pre; font-family: monospace; text-align: right;"
                        style:padding-top=move || format!("{}px", font.get().height() / 4.0)
                        style:padding-left=move || format!("{}px", font.get().width())>
                        {y_value}
                    </td>
                </tr>
            }
        }
    };

    view! {
        <Show when=move || hover_inner.get() && placement.get() != HoverPlacement::Hide >
            <DebugRect label="tooltip" debug=debug />
            <aside
                style="position: absolute; z-index: 1; width: max-content; height: max-content; transform: translateY(-50%); border: 1px solid lightgrey; background-color: #fff; white-space: pre; font-family: monospace;"
                style:top=move || format!("calc({}px)", mouse_page.get().1)
                style:right=move || format!("calc(100% - {}px + {}px)", mouse_page.get().0, cursor_distance.get())
                style:padding=move || padding.get().to_css_style()>
                <h2
                    style="margin: 0; text-align: center;"
                    style:font-size=move || format!("{}px", font.get().height())>
                    {x_body}
                </h2>
                <table
                    style="border-collapse: collapse; border-spacing: 0; margin: 0 auto; padding: 0;"
                    style:font-size=move || format!("{}px", font.get().height())>
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
