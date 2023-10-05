#[derive(Copy, Clone, Debug)]
pub struct Font {
    height: f64,
    width: f64,
}

impl Font {
    pub fn new(height: f64, width: f64) -> Self {
        Self { height, width }
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn svg_size(&self) -> String {
        format!("{}px", self.height)
    }

    pub fn svg_family(&self) -> String {
        "monospace".to_string()
    }
}
