use super::{Format, Span};
use std::rc::Rc;

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

pub type TickFormatFn<Tick> = dyn Fn(&Tick, &dyn Format<Tick = Tick>) -> String;

pub struct HorizontalSpan<Tick: 'static> {
    font_width: f64,
    min_chars: usize,
    padding_width: f64,
    avail_width: f64,
    format: Rc<TickFormatFn<Tick>>,
}

impl<Tick> HorizontalSpan<Tick> {
    pub fn new(
        font_width: f64,
        min_chars: usize,
        padding_width: f64,
        avail_width: f64,
        format: Rc<TickFormatFn<Tick>>,
    ) -> Self {
        Self {
            font_width,
            min_chars,
            padding_width,
            avail_width,
            format,
        }
    }

    pub fn identity_format() -> Rc<TickFormatFn<Tick>> {
        Rc::new(|tick, state| state.format(tick))
    }
}

impl<Tick> Span<Tick> for HorizontalSpan<Tick> {
    fn length(&self) -> f64 {
        self.avail_width
    }

    fn consumed(&self, state: &dyn Format<Tick = Tick>, ticks: &[Tick]) -> f64 {
        let max_chars = ticks
            .iter()
            .map(|tick| (self.format)(tick, state).len().max(self.min_chars))
            .max()
            .unwrap_or_default();
        let max_label_width = max_chars as f64 * self.font_width + self.padding_width * 2.0;
        max_label_width * ticks.len() as f64
    }
}
