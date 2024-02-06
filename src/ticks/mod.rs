mod gen;

pub use gen::{
    AlignedFloats, Format as TickFormat, GeneratedTicks, Generator as TickGen, HorizontalSpan,
    Period, TickFormatFn, Timestamps, VerticalSpan,
};

use chrono::prelude::*;

/// A type that can be used as a tick on an axis. Try to rely on provided implementations.
pub trait Tick: Clone + PartialEq + PartialOrd + std::fmt::Debug + 'static {
    /// Default fallback tick generator for when one is not provided.
    fn default_generator() -> impl TickGen<Tick = Self>;

    /// Maps the tick to a position on the axis. Must be uniform.
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
        Timestamps::default()
    }

    fn position(&self) -> f64 {
        self.timestamp() as f64 + (self.timestamp_subsec_nanos() as f64 / 1e9)
    }
}
