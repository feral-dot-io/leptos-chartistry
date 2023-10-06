use leptos::*;

#[derive(Clone, Debug)]
pub struct Line {
    pub(crate) name: MaybeSignal<String>,
    pub(crate) width: MaybeSignal<f64>,
}

impl Line {
    pub fn new(name: impl Into<MaybeSignal<String>>) -> Self {
        Self {
            name: name.into(),
            width: 1.0.into(),
        }
    }

    pub fn set_width(mut self, width: impl Into<MaybeSignal<f64>>) -> Self {
        self.width = width.into();
        self
    }
}

#[component]
pub fn Line<'a>(line: &'a Line, positions: Vec<(f64, f64)>) -> impl IntoView {
    let mut first = true;
    let path = positions
        .into_iter()
        .map(|(x, y)| {
            if first {
                first = false;
                format!("M {} {} ", x, y)
            } else {
                format!("L {} {} ", x, y)
            }
        })
        .collect::<String>();
    view! {
        <path d=path fill="none" stroke="red" stroke-width=line.width />
    }
}
