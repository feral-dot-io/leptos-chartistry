mod gen;

pub use gen::{
    AlignedFloats, Format as TickFormat, GeneratedTicks, Generator as TickGen, HorizontalSpan,
    Period, TickFormatFn, Timestamps, VerticalSpan,
};

use chrono::prelude::*;

/// A type that can be used as a tick on an axis. Try to rely on provided implementations.
pub trait Tick: Clone + PartialEq + PartialOrd + Send + Sync + 'static {
    /// Default tick generator used in tick labels.
    fn tick_label_generator() -> impl TickGen<Tick = Self>;

    /// Default tick generator used in tooltips.
    fn tooltip_generator() -> impl TickGen<Tick = Self> {
        Self::tick_label_generator()
    }

    /// Maps the tick to a position on the axis. Must be uniform. May return `f64::NAN` for missing data.
    fn position(&self) -> f64;
}

impl Tick for f64 {
    fn tick_label_generator() -> impl TickGen<Tick = Self> {
        AlignedFloats::default()
    }

    fn position(&self) -> f64 {
        *self
    }
}

impl<Tz> Tick for DateTime<Tz>
where
    Tz: TimeZone + Send + Sync + 'static,
    Tz::Offset: std::fmt::Display + Send + Sync,
{
    fn tick_label_generator() -> impl TickGen<Tick = Self> {
        Timestamps::default()
    }

    fn tooltip_generator() -> impl TickGen<Tick = Self> {
        Timestamps::default().with_long_format()
    }

    fn position(&self) -> f64 {
        self.timestamp() as f64 + (self.timestamp_subsec_nanos() as f64 / 1e9)
    }
}
