pub trait TickGen {
    type Tick;

    fn generate(
        &self,
        first: &Self::Tick,
        last: &Self::Tick,
        span: Box<dyn Span>,
    ) -> GeneratedTicks<Self::Tick>;
}

pub struct GeneratedTicks<Tick> {
    pub ticks: Vec<Tick>,
    pub state: Box<dyn TickState<Tick = Tick>>,
}

impl<Tick> GeneratedTicks<Tick> {
    pub fn none(state: impl TickState<Tick = Tick> + 'static) -> GeneratedTicks<Tick> {
        GeneratedTicks {
            ticks: vec![],
            state: Box::new(state),
        }
    }
}

pub trait TickState {
    type Tick;

    fn position(&self, value: &Self::Tick) -> f64;
    fn short_format(&self, value: &Self::Tick) -> String;
    fn long_format(&self, value: &Self::Tick) -> String;
}

pub trait Span {
    fn length(&self) -> f64;
    fn consumed(&self, ticks: &[&str]) -> f64;
}
