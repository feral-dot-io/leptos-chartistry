mod aligned_floats;
mod gen;
mod timestamps;

pub use aligned_floats::AlignedFloatsGen;
pub use gen::{
    GenState as TickState, GeneratedTicks, Generator as TickGen, HorizontalSpan, VerticalSpan,
};
pub use timestamps::{Gen as TimestampGen, Period};

pub type TickFormatFn<Tick> = std::rc::Rc<dyn Fn(&dyn gen::GenState<Tick = Tick>, &Tick) -> String>;

use chrono::prelude::*;

pub trait Tick: Clone + PartialEq + PartialOrd + 'static {
    fn position(&self) -> f64;
}

impl Tick for f64 {
    fn position(&self) -> f64 {
        *self
    }
}

impl<Tz: TimeZone + 'static> Tick for DateTime<Tz> {
    fn position(&self) -> f64 {
        self.timestamp() as f64 + (self.timestamp_subsec_nanos() as f64 / 1e9)
    }
}
