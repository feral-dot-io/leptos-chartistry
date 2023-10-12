use super::axis_marker::AxisMarker;
use crate::projection::Projection;
use leptos::*;

pub enum InnerOption {
    AxisMarker(AxisMarker),
}

impl InnerOption {
    pub fn render(self, proj: Signal<Projection>) -> View {
        match self {
            Self::AxisMarker(marker) => view! {
                <AxisMarker marker={marker} projection=proj />
            },
        }
    }
}
