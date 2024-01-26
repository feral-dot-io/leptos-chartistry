mod aligned_floats;
mod timestamps;

pub use aligned_floats::AlignedFloats;
pub use timestamps::{Period, PeriodicTimestamps};

use std::rc::Rc;

pub trait Generator {
    type Tick;

    fn generate(
        &self,
        first: &Self::Tick,
        last: &Self::Tick,
        span: Box<dyn Span<Self::Tick>>,
    ) -> GeneratedTicks<Self::Tick>;
}

pub trait Span<Tick> {
    fn length(&self) -> f64;
    fn consumed(&self, state: &dyn GenState<Tick = Tick>, ticks: &[Tick]) -> f64;
}

pub trait GenState {
    type Tick;

    fn position(&self, value: &Self::Tick) -> f64;
    fn format(&self, value: &Self::Tick) -> String;
}

#[derive(Clone)]
pub struct GeneratedTicks<Tick> {
    pub state: Rc<dyn GenState<Tick = Tick>>,
    pub ticks: Vec<Tick>,
}

impl<Tick: 'static> GeneratedTicks<Tick> {
    pub fn new(ticks: Vec<Tick>, state: impl GenState<Tick = Tick> + 'static) -> Self {
        let state = Rc::new(state);
        GeneratedTicks { state, ticks }
    }

    pub fn none() -> GeneratedTicks<Tick> {
        Self::new(vec![], NilState(std::marker::PhantomData))
    }
}

// Dummy TickState that should never be called. Used with no ticks.
struct NilState<Tick>(std::marker::PhantomData<Tick>);

impl<Tick> GenState for NilState<Tick> {
    type Tick = Tick;

    fn position(&self, _: &Self::Tick) -> f64 {
        0.0
    }

    fn format(&self, _: &Self::Tick) -> String {
        "-".to_string()
    }
}

/// Note: PartialEq only compares the `ticks`. Meaning TickGen implementations must result in the same TickState when Ticks are equal.
impl<Tick: PartialEq> PartialEq for GeneratedTicks<Tick> {
    fn eq(&self, other: &Self) -> bool {
        self.ticks == other.ticks
    }
}

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

    fn consumed(&self, _: &dyn GenState<Tick = Tick>, ticks: &[Tick]) -> f64 {
        self.line_height * ticks.len() as f64
    }
}

pub struct HorizontalSpan {
    avail_width: f64,
    font_width: f64,
    padding_width: f64,
}

impl HorizontalSpan {
    pub fn new(font_width: f64, padding_width: f64, avail_width: f64) -> Self {
        Self {
            avail_width,
            font_width,
            padding_width,
        }
    }
}

impl<Tick> Span<Tick> for HorizontalSpan {
    fn length(&self) -> f64 {
        self.avail_width
    }

    fn consumed(&self, state: &dyn GenState<Tick = Tick>, ticks: &[Tick]) -> f64 {
        let max_chars = ticks
            .iter()
            .map(|tick| state.format(tick).len())
            .max()
            .unwrap_or_default();
        let max_label_width = max_chars as f64 * self.font_width + self.padding_width * 2.0;
        max_label_width * ticks.len() as f64
    }
}
