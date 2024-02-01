mod gen;

pub use gen::{
    AlignedFloats, GenState as TickState, GeneratedTicks, Generator as TickGen, HorizontalSpan,
    Period, PeriodicTimestamps, TimestampFormat, VerticalSpan,
};

use chrono::prelude::*;

pub trait Tick: Clone + PartialEq + PartialOrd + 'static {
    fn default_generator() -> impl TickGen<Tick = Self>;
    fn position(&self) -> f64;
}

impl Tick for f64 {
    fn default_generator() -> impl TickGen<Tick = Self> {
        AlignedFloats::default()
    }

    fn position(&self) -> f64 {
        *self
    }
}

impl<Tz> Tick for DateTime<Tz>
where
    Tz: TimeZone + 'static,
    Tz::Offset: std::fmt::Display,
{
    fn default_generator() -> impl TickGen<Tick = Self> {
        PeriodicTimestamps::default()
    }

    fn position(&self) -> f64 {
        self.timestamp() as f64 + (self.timestamp_subsec_nanos() as f64 / 1e9)
    }
}
