pub mod edge_layout;
pub mod edge_legend;
pub mod edge_rotated_label;
pub mod edge_tick_labels;
pub mod feature_colours;
pub mod feature_tooltip;
pub mod inner_axis_marker;
pub mod inner_grid_line;
pub mod inner_guide_line;
pub mod inner_layout;
pub mod inner_legend;
pub mod series_line;
pub mod series_line_stack;

use leptos::signal_prelude::*;

pub struct MyData {
    x: f64,
    y1: f64,
    y2: f64,
}

impl MyData {
    fn new(x: f64, y1: f64, y2: f64) -> Self {
        Self { x, y1, y2 }
    }
}

pub fn load_data() -> Signal<Vec<MyData>> {
    Signal::derive(|| {
        vec![
            MyData::new(0.0, 1.0, 0.0),
            MyData::new(1.0, 3.0, 1.0),
            MyData::new(2.0, 5.0, 2.5),
            MyData::new(3.0, 5.5, 3.0),
            MyData::new(4.0, 5.0, 3.0),
            MyData::new(5.0, 2.5, 4.0),
            MyData::new(6.0, 2.25, 9.0),
            MyData::new(7.0, 3.0, 5.0),
            MyData::new(8.0, 7.0, 3.5),
            MyData::new(10.0, 10.0, 3.0),
        ]
    })
}
