use super::{Format, Span};

pub struct VerticalSpan {
    avail_height: f64,
    line_height: f64,
}

impl VerticalSpan {
    pub fn new(line_height: f64, avail_height: f64) -> Self {
        Self {
            avail_height,
            line_height,
        }
    }
}

impl<Tick> Span<Tick> for VerticalSpan {
    fn length(&self) -> f64 {
        self.avail_height
    }

    fn consumed(&self, _: &dyn Format<Tick = Tick>, ticks: &[Tick]) -> f64 {
        self.line_height * ticks.len() as f64
    }
}

pub struct HorizontalSpan {
    font_width: f64,
    min_chars: usize,
    padding_width: f64,
    avail_width: f64,
}

impl HorizontalSpan {
    pub fn new(font_width: f64, min_chars: usize, padding_width: f64, avail_width: f64) -> Self {
        Self {
            font_width,
            min_chars,
            padding_width,
            avail_width,
        }
    }
}

impl<Tick> Span<Tick> for HorizontalSpan {
    fn length(&self) -> f64 {
        self.avail_width
    }

    fn consumed(&self, state: &dyn Format<Tick = Tick>, ticks: &[Tick]) -> f64 {
        let max_chars = ticks
            .iter()
            .map(|tick| state.format(tick).len().max(self.min_chars))
            .max()
            .unwrap_or_default();
        let max_label_width = max_chars as f64 * self.font_width + self.padding_width * 2.0;
        max_label_width * ticks.len() as f64
    }
}
