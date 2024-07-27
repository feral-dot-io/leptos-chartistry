mod aligned_floats;
mod span;
mod timestamps;

pub use aligned_floats::AlignedFloats;
pub use span::{HorizontalSpan, TickFormatFn, VerticalSpan};
pub use timestamps::{Period, Timestamps};

use std::{rc::Rc, sync::Arc};

pub trait Generator {
    type Tick;

    fn generate(
        &self,
        first: &Self::Tick,
        last: &Self::Tick,
        span: &dyn Span<Self::Tick>,
    ) -> GeneratedTicks<Self::Tick>;
}

pub trait Span<Tick> {
    fn length(&self) -> f64;
    fn consumed(&self, state: &dyn Format<Tick = Tick>, ticks: &[Tick]) -> f64;
}

pub trait Format {
    type Tick;
    fn format(&self, value: &Self::Tick) -> String;
}

#[derive(Clone)]
pub struct GeneratedTicks<Tick> {
    pub state: Arc<dyn Format<Tick = Tick> + Send + Sync>,
    pub ticks: Vec<Tick>,
}

impl<Tick> GeneratedTicks<Tick> {
    pub fn new(state: impl Format<Tick = Tick> + 'static, ticks: Vec<Tick>) -> Self {
        GeneratedTicks {
            state: Rc::new(state),
            ticks,
        }
    }
}

impl<Tick: 'static> GeneratedTicks<Tick> {
    pub fn none() -> GeneratedTicks<Tick> {
        Self::new(NilState(std::marker::PhantomData), vec![])
    }
}

// Dummy TickState that should never be called. Used with no ticks.
struct NilState<Tick>(std::marker::PhantomData<Tick>);

impl<Tick> Format for NilState<Tick> {
    type Tick = Tick;

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
