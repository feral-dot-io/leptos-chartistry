use crate::colours::Colour;
use leptos::*;

#[derive(Clone, Debug)]
pub struct Line {
    pub(crate) name: MaybeSignal<String>,
    pub(crate) colour: Option<MaybeSignal<Colour>>,
    pub(crate) width: MaybeSignal<f64>,
}

#[derive(Clone, Debug)]
pub struct UseLine {
    pub(crate) name: MaybeSignal<String>,
    pub(crate) colour: MaybeSignal<Colour>,
    pub(crate) width: MaybeSignal<f64>,
}

impl Line {
    pub fn new(name: impl Into<MaybeSignal<String>>) -> Self {
        Self {
            name: name.into(),
            colour: None,
            width: 1.0.into(),
        }
    }

    pub fn set_colour(mut self, colour: impl Into<MaybeSignal<Colour>>) -> Self {
        self.colour = Some(colour.into());
        self
    }

    pub fn set_width(mut self, width: impl Into<MaybeSignal<f64>>) -> Self {
        self.width = width.into();
        self
    }

    pub(super) fn use_line(self, colour: Colour) -> UseLine {
        UseLine {
            name: self.name,
            colour: self.colour.unwrap_or_else(|| colour.into()),
            width: self.width,
        }
    }
}

impl From<&str> for Line {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

impl From<String> for Line {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

#[component]
pub fn Line<'a>(line: &'a UseLine, positions: Vec<(f64, f64)>) -> impl IntoView {
    let mut need_move = true;
    let path = positions
        .into_iter()
        .map(|(x, y)| {
            if x.is_nan() || y.is_nan() {
                need_move = true;
                "".to_string()
            } else if need_move {
                need_move = false;
                format!("M {} {} ", x, y)
            } else {
                format!("L {} {} ", x, y)
            }
        })
        .collect::<String>();
    let colour = line.colour;
    view! {
        <path
            d=path
            fill="none"
            stroke=move || colour.get().to_string()
            stroke-width=line.width
        />
    }
}
