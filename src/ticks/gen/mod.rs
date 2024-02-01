mod aligned_floats;
mod span;
mod timestamps;

pub use aligned_floats::AlignedFloats;
pub use span::{HorizontalSpan, VerticalSpan};
pub use timestamps::{Format as TimestampFormat, Period, PeriodicTimestamps};

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
