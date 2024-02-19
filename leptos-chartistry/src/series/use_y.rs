use super::{UseData, UseLine};
use crate::series::line::RenderLine;
use leptos::*;

#[derive(Clone, Debug, PartialEq)]
pub struct UseY {
    pub id: usize,
    pub name: RwSignal<String>,
    pub desc: UseYDesc,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UseYDesc {
    Line(UseLine),
}

impl UseY {
    pub(crate) fn render<X: 'static, Y: 'static>(
        &self,
        data: UseData<X, Y>,
        positions: Signal<Vec<(f64, f64)>>,
    ) -> View {
        match &self.desc {
            UseYDesc::Line(line) => view! {
                <RenderLine
                    use_y=self.clone()
                    line=line.clone()
                    data=data
                    positions=positions
                    markers=positions />
            },
        }
    }
}
