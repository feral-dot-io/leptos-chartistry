use std::rc::Rc;

pub trait TickGen {
    type Tick;

    fn generate(
        &self,
        first: &Self::Tick,
        last: &Self::Tick,
        span: Box<dyn Span<Self::Tick>>,
    ) -> GeneratedTicks<Self::Tick>;
}

#[derive(Clone)]
pub struct GeneratedTicks<Tick> {
    pub ticks: Vec<Tick>,
    pub state: Rc<dyn TickState<Tick = Tick>>,
}

impl<Tick> GeneratedTicks<Tick> {
    pub fn new(ticks: Vec<Tick>, state: impl TickState<Tick = Tick> + 'static) -> Self {
        GeneratedTicks {
            ticks: ticks,
            state: Rc::new(state),
        }
    }

    pub fn none(state: impl TickState<Tick = Tick> + 'static) -> GeneratedTicks<Tick> {
        Self::new(vec![], state)
    }
}

/// Note: PartialEq only compares the `ticks`. Meaning TickGen implementations must result in the same TickState when Ticks are equal.
impl<Tick: PartialEq> PartialEq for GeneratedTicks<Tick> {
    fn eq(&self, other: &Self) -> bool {
        self.ticks == other.ticks
    }
}

pub trait TickState {
    type Tick;

    fn position(&self, value: &Self::Tick) -> f64;
    fn short_format(&self, value: &Self::Tick) -> String;
    fn long_format(&self, value: &Self::Tick) -> String;
}

pub trait Span<Tick> {
    fn length(&self) -> f64;
    fn consumed(&self, state: &dyn TickState<Tick = Tick>, ticks: &[Tick]) -> f64;
}
