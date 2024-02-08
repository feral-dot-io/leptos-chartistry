pub mod edge_legend;
pub mod edge_tick_labels;

use leptos::signal_prelude::*;
use leptos_chartistry::*;

const EXAMPLE_ASPECT_RATIO: AspectRatio = AspectRatio::inner_ratio(300.0, 300.0);

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
            MyData::new(3.0, 6.0, 3.0),
            MyData::new(4.0, 5.0, 3.0),
            MyData::new(5.0, 3.0, 4.0),
            MyData::new(6.0, 2.5, 8.0),
            MyData::new(7.0, 4.0, 6.0),
            MyData::new(8.0, 7.0, 4.5),
            MyData::new(10.0, 9.0, 3.0),
        ]
    })
}
