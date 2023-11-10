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

pub trait Span<Tick> {
    fn length(&self) -> f64;
    fn consumed(&self, state: &dyn TickState<Tick = Tick>, ticks: &[Tick]) -> f64;
}

pub trait TickState {
    type Tick;

    fn position(&self, value: &Self::Tick) -> f64;
    fn short_format(&self, value: &Self::Tick) -> String;
    fn long_format(&self, value: &Self::Tick) -> String;
}

pub type TickFormatFn<Tick> = Rc<dyn Fn(&dyn TickState<Tick = Tick>, &Tick) -> String>;

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

pub fn short_format_fn<Tick>() -> TickFormatFn<Tick> {
    Rc::new(|state, tick| state.short_format(tick))
}

pub fn long_format_fn<Tick>() -> TickFormatFn<Tick> {
    Rc::new(|state, tick| state.long_format(tick))
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

    fn consumed(&self, _: &dyn TickState<Tick = Tick>, ticks: &[Tick]) -> f64 {
        self.line_height * ticks.len() as f64
    }
}

pub struct HorizontalSpan<Tick> {
    format: TickFormatFn<Tick>,
    avail_width: f64,
    font_width: f64,
    padding_width: f64,
}

impl<Tick> HorizontalSpan<Tick> {
    pub fn new(
        format: TickFormatFn<Tick>,
        font_width: f64,
        padding_width: f64,
        avail_width: f64,
    ) -> Self {
        Self {
            format,
            avail_width,
            font_width,
            padding_width,
        }
    }
}

impl<Tick> Span<Tick> for HorizontalSpan<Tick> {
    fn length(&self) -> f64 {
        self.avail_width
    }

    fn consumed(&self, state: &dyn TickState<Tick = Tick>, ticks: &[Tick]) -> f64 {
        let max_chars = (ticks.into_iter())
            .map(|tick| (self.format)(state, tick).len())
            .max()
            .unwrap_or_default();
        let max_label_width = max_chars as f64 * self.font_width + self.padding_width * 2.0;
        max_label_width * ticks.len() as f64
    }
}
