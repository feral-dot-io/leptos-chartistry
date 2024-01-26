mod gen;

pub use gen::{
    AlignedFloats, GenState as TickState, GeneratedTicks, Generator as TickGen, HorizontalSpan,
    Period, PeriodicTimestamps, VerticalSpan,
};

use chrono::prelude::*;

pub trait Tick: Clone + PartialEq + PartialOrd + 'static {
    fn position(&self) -> f64;
}

impl Tick for f64 {
    fn position(&self) -> f64 {
        *self
    }
}

impl<Tz> Tick for DateTime<Tz>
where
    Tz: TimeZone + 'static,
    Tz::Offset: std::fmt::Display,
{
    fn position(&self) -> f64 {
        self.timestamp() as f64 + (self.timestamp_subsec_nanos() as f64 / 1e9)
    }
}
