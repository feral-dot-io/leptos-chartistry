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

impl Span for VerticalSpan {
    fn length(&self) -> f64 {
        self.avail_height
    }

    fn consumed(&self, ticks: &[&str]) -> f64 {
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

impl Span for HorizontalSpan {
    fn length(&self) -> f64 {
        self.avail_width
    }

    fn consumed(&self, ticks: &[&str]) -> f64 {
        let max_chars = (ticks.into_iter())
            .map(|tick| tick.len())
            .max()
            .unwrap_or_default();
        let max_label_width = max_chars as f64 * self.font_width + self.padding_width * 2.0;
        max_label_width * ticks.len() as f64
    }
}
