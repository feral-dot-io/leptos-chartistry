use super::{Format, GeneratedTicks, Generator, Span};
use chrono::{prelude::*, Duration, DurationRound, Months};
use std::{borrow::Borrow, fmt::Display, ops::Add, sync::Arc};

/// Generates timestamp ticks from a set of periods. Aligned to nice values (earlier periods).
#[derive(Clone)]
pub struct Timestamps<Tz> {
    format: Arc<dyn TimestampFormat<Tz> + Send + Sync>,
    periods: Vec<Period>,
    tz: std::marker::PhantomData<Tz>,
}

trait TimestampFormat<Tz: TimeZone> {
    fn format(&self, period: Period, at: &DateTime<Tz>) -> String;
}

/// Available periods for [timestamp tick generation](Timestamps::from_periods).
// Note: Quarter and Week would be useful but would need more formatting options e.g., strftime doesn't offer quarter formatting and we would need to specify when weeks start which would probably want to coincide with years using %G or %Y
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Period {
    /// Nanosecond. Warning this generates a lot of timestamps that the algorithm will have to sample. Skip unless needed.
    Nanosecond,
    /// Microsecond (us)
    Microsecond,
    /// Millisecond (ms)
    Millisecond,
    /// Second (s)
    Second,
    /// Minute (m)
    Minute,
    /// Hour (h)
    Hour,
    /// Day (d)
    Day,
    /// Month (M)
    Month,
    /// Year (Y)
    Year,
}

#[derive(Clone)]
struct State<Tz: TimeZone> {
    format: Arc<dyn TimestampFormat<Tz> + Send + Sync>,
    all_periods: Vec<Period>,
    period: Period,
    tz: std::marker::PhantomData<Tz>,
}

impl<Tz> Default for Timestamps<Tz>
where
    Tz: TimeZone,
    Tz::Offset: Display,
{
    fn default() -> Self {
        Self::from_periods(Period::all())
    }
}

impl<Tz> Timestamps<Tz>
where
    Tz: TimeZone,
    Tz::Offset: Display,
{
    /// Create a new timestamp generator from a set of periods. Periods are sorted and deduplicated.
    pub fn from_periods(periods: impl Borrow<[Period]>) -> Self {
        let mut periods = periods.borrow().to_vec();
        periods.sort_unstable();
        periods.dedup();
        periods.reverse();
        Self {
            format: Arc::new(ShortFormat),
            periods,
            tz: std::marker::PhantomData,
        }
    }

    /// Create a new timestamp generator from a single period.
    pub fn from_period(period: impl Into<Period>) -> Self {
        Self::from_periods([period.into()])
    }

    /// Sets the short format for timestamps. This is the default.
    ///
    /// A short format tries to use the smallest possible representation for a period while reducing ambiguity. and is intended to be used where space is constrained e.g., tick labels. For example, a second will be formatted as `HH:MM:SS` and a year as `YYYY`.
    pub fn with_short_format(mut self) -> Self {
        self.format = Arc::new(ShortFormat);
        self
    }

    /// Sets the long format for timestamps. Shows a full date with time zone while trying to minimise detail. For example if timestamps are shown to the day then "YYYY-MM-DD Z" will be used dropping "HH:MM" however if hours are then shown, they would be included.
    ///
    /// Unlike the short format, the long format is intended to be used where space is not constrained e.g., the tooltip.
    pub fn with_long_format(mut self) -> Self {
        self.format = Arc::new(LongFormat);
        self
    }

    /// Sets a fixed strftime format for timestamps. See [chrono::strftime](https://docs.rs/chrono/0.4.33/chrono/format/strftime/index.html).
    pub fn with_strftime(mut self, format: impl Into<String>) -> Self {
        self.format = Arc::new(StrftimeFormat(format.into()));
        self
    }

    /// Sets a custom format for timestamps. The given `Period` is the latest period that was selected. The `DateTime` is the timestamp to format.
    pub fn with_format(
        mut self,
        f: impl Fn(Period, &DateTime<Tz>) -> String + Send + Sync + 'static,
    ) -> Self {
        self.format = Arc::new(f);
        self
    }
}

impl<Tz> Generator for Timestamps<Tz>
where
    Tz: TimeZone + Send + Sync + 'static,
    Tz::Offset: Send + Sync,
{
    type Tick = DateTime<Tz>;

    fn generate(
        &self,
        first: &Self::Tick,
        last: &Self::Tick,
        span: &dyn Span<Self::Tick>,
    ) -> GeneratedTicks<Self::Tick> {
        // No periods?
        if self.periods.is_empty() {
            return GeneratedTicks::none();
        }

        let mut ticks = Vec::new();
        let mut state = State::from_period(self, self.periods[0]);

        'outer: for &period in &self.periods {
            // Fetch all ticks for this period
            let candidate = period
                .iter_aligned_range(first.clone(), last.clone())
                .collect::<Vec<_>>();
            // Try to fit candidate ticks into previous ticks, sampling if necessary
            for sample in 1..(candidate.len() + 1) {
                let sampled = Self::merge_ticks(&ticks, &candidate, sample);
                state = State::from_period(self, period);
                let used_width = span.consumed(&state, &sampled);
                // Our sampled ticks fit
                if used_width <= span.length() {
                    ticks = sampled;
                    // Stop entirely if we've had to use sampling at all
                    if sample != 1 {
                        break 'outer;
                    }
                    // We want a period to "dominate" i.e., take up most of the space. Stop if we've used more than half the space as that can no longer happen
                    if used_width > span.length() / 2.0 {
                        break 'outer;
                    }
                    break;
                } else if sampled.len() == 1 {
                    // Our ticks are small (a few chars). If we can't fit a single one then we can assume later periods won't either, so bail out now.
                    // This is particularly important when our span is small and won't hold any ticks at all. If we don't bail we end up generating all ticks from all periods -- incl. the ns range -- which then gets thrown away.
                    break 'outer;
                }
            }
        }

        GeneratedTicks::new(state, ticks)
    }
}

impl<Tz: TimeZone> Timestamps<Tz> {
    fn merge_ticks<T: Clone + Ord>(existing: &[T], candidate: &[T], sample: usize) -> Vec<T> {
        assert!(sample > 0);
        let candidate = candidate.to_owned();
        // Find a common index between existing and candidate to align samples
        let common_index = existing
            .iter()
            .filter_map(|t| candidate.binary_search(t).ok())
            .take(1)
            .next()
            .unwrap_or(sample - 1);
        // Sample candidate ticks
        let candidate = Self::sample_ticks(candidate, common_index, sample);
        let mut ticks = existing
            .iter()
            .cloned()
            .chain(candidate)
            .collect::<Vec<_>>();
        // Stable sort by time
        ticks.sort();
        // Remove duplicate ticks. These will be from lower periods e.g., if we have both seconds and minutes then we will have duplicate ticks at the minute boundary.
        ticks.dedup();
        ticks
    }

    /// Reduces ticks by sampling. Keeps the nth specified by `keep_every`, drops the rest. Picks a tick from a range of `keep_every` such that `ticks[align_index]` is included in the results. For example, if `keep_every` is 2 then every other tick is kept. If `keep_every` is 1 then all ticks are kept. If `keep_every` is 0 then no ticks are kept. Panics if `keep_every` is zero and if `align_index >= ticks.len()`.
    fn sample_ticks<T>(ticks: Vec<T>, align_index: usize, keep_every: usize) -> Vec<T> {
        assert!(keep_every > 0);
        // Ensure we keep the tick at align_index
        let mod_result = align_index % keep_every;
        ticks
            .into_iter()
            .enumerate()
            .filter(|(i, _)| i % keep_every == mod_result)
            .map(|(_, t)| t)
            .collect()
    }
}

impl<Tz: TimeZone> State<Tz> {
    fn from_period(gen: &Timestamps<Tz>, period: Period) -> Self {
        Self {
            format: gen.format.clone(),
            all_periods: gen.periods.clone(),
            period,
            tz: std::marker::PhantomData,
        }
    }
}

impl<Tz: TimeZone> Format for State<Tz> {
    type Tick = DateTime<Tz>;

    fn format(&self, at: &Self::Tick) -> String {
        let mut period = self.period;
        // If tick falls exactly on an earlier period, use that representation instead
        for earlier in &self.all_periods {
            if earlier.truncate_at(at.clone()) == Some(at.clone()) {
                period = *earlier;
                break;
            }
        }
        // Use local formatter
        self.format.format(period, at)
    }
}

struct ShortFormat;
impl<Tz> TimestampFormat<Tz> for ShortFormat
where
    Tz: TimeZone,
    Tz::Offset: Display,
{
    fn format(&self, period: Period, at: &DateTime<Tz>) -> String {
        let fmt = period.short_format();
        at.format(fmt).to_string()
    }
}

struct LongFormat;
impl<Tz> TimestampFormat<Tz> for LongFormat
where
    Tz: TimeZone,
    Tz::Offset: Display,
{
    fn format(&self, period: Period, at: &DateTime<Tz>) -> String {
        let fmt = period.long_format();
        at.format(fmt).to_string()
    }
}

struct StrftimeFormat(String);
impl<Tz> TimestampFormat<Tz> for StrftimeFormat
where
    Tz: TimeZone,
    Tz::Offset: Display,
{
    fn format(&self, _: Period, at: &DateTime<Tz>) -> String {
        at.format(&self.0).to_string()
    }
}

impl<F, Tz> TimestampFormat<Tz> for F
where
    F: Fn(Period, &DateTime<Tz>) -> String,
    Tz: TimeZone,
{
    fn format(&self, period: Period, at: &DateTime<Tz>) -> String {
        (self)(period, at)
    }
}

impl Period {
    fn short_format(self) -> &'static str {
        match self {
            Period::Nanosecond => "%H:%M:%S.%f",
            Period::Microsecond => "%H:%M:%S.%6f",
            Period::Millisecond => "%H:%M:%S.%3f",
            Period::Second => "%H:%M:%S",
            Period::Hour | Period::Minute => "%H:%M",
            Period::Day => "%a",
            Period::Month => "%b",
            Period::Year => "%Y",
        }
    }

    fn long_format(self) -> &'static str {
        match self {
            Period::Nanosecond => "%Y-%m-%d %H:%M:%S.%9f %Z",
            Period::Microsecond => "%Y-%m-%d %H:%M:%S.%6f %Z",
            Period::Millisecond => "%Y-%m-%d %H:%M:%S.%3f %Z",
            Period::Second => "%Y-%m-%d %H:%M:%S %Z",
            Period::Day | Period::Hour | Period::Minute => "%Y-%m-%d %H:%M %Z",
            Period::Month => "%B %Y %Z",
            Period::Year => "%Y %Z",
        }
    }

    /// All available periods.
    pub const fn all() -> [Period; 9] {
        [
            Period::Year,
            Period::Month,
            Period::Day,
            Period::Hour,
            Period::Minute,
            Period::Second,
            Period::Millisecond,
            Period::Microsecond,
            Period::Nanosecond,
        ]
    }
}

#[derive(Clone, Debug, PartialEq)]
struct AlignedPeriodRange<Tz: TimeZone> {
    next: DateTime<Tz>,
    advance: Period,
    not_after: DateTime<Tz>,
}

impl<Tz: TimeZone> Iterator for AlignedPeriodRange<Tz> {
    type Item = DateTime<Tz>;

    fn next(&mut self) -> Option<Self::Item> {
        // Stop once we reach to (note: could be starting condition)
        if self.next >= self.not_after {
            None
        } else {
            let next = self.next.clone();
            self.next = self.next.clone() + self.advance;
            Some(next)
        }
    }
}

impl Period {
    fn iter_aligned_range<Tz: TimeZone>(
        self,
        from: DateTime<Tz>,
        to: DateTime<Tz>,
    ) -> AlignedPeriodRange<Tz> {
        // Truncate `from` by the period. If from can't be aligned then iterate over nothing.
        let mut aligned = self.truncate_at(from.clone()).unwrap_or_else(|| to.clone());
        // Advance to the first aligned value >= `from`
        while aligned < from {
            aligned = aligned + self;
        }
        AlignedPeriodRange {
            next: aligned,
            advance: self,
            not_after: to,
        }
    }

    fn truncate_at<Tz: TimeZone>(self, at: DateTime<Tz>) -> Option<DateTime<Tz>> {
        let duration = match self {
            Period::Nanosecond => Duration::nanoseconds(1),
            Period::Microsecond => Duration::microseconds(1),
            Period::Millisecond => Duration::milliseconds(1),
            Period::Second => Duration::seconds(1),
            Period::Minute => Duration::minutes(1),
            Period::Hour => Duration::hours(1),
            Period::Day => Duration::days(1),

            // Variable periods. Can't use duration_trunc
            Period::Month => {
                return at
                    .timezone()
                    .with_ymd_and_hms(at.year(), at.month(), 1, 0, 0, 0)
                    .latest();
            }
            Period::Year => {
                return at
                    .timezone()
                    .with_ymd_and_hms(at.year(), 1, 1, 0, 0, 0)
                    .latest();
            }
        };

        // If at is zero (1970) then duration_trunc will fail but it's already aligned, so do nothing
        if Some(0) == at.timestamp_nanos_opt() {
            Some(at)
        } else {
            // Truncate non-variable periods
            at.duration_trunc(duration).ok()
        }
    }
}

impl<Tz: TimeZone> Add<Period> for DateTime<Tz> {
    type Output = Self;
    fn add(self, rhs: Period) -> Self::Output {
        match rhs {
            Period::Nanosecond => self + Duration::nanoseconds(1),
            Period::Microsecond => self + Duration::microseconds(1),
            Period::Millisecond => self + Duration::milliseconds(1),
            Period::Second => self + Duration::seconds(1),
            Period::Minute => self + Duration::minutes(1),
            Period::Hour => self + Duration::hours(1),
            Period::Day => self + Duration::days(1),
            //Period::Week => self + Duration::weeks(1),
            Period::Month => self + Months::new(1),
            //Period::Quarter => self + Months::new(3),
            Period::Year => self + Months::new(12),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::HorizontalSpan;
    use super::*;
    use crate::Tick;

    fn assert_ticks<Tick>(ticks: GeneratedTicks<Tick>, expected: Vec<&'static str>) {
        let GeneratedTicks { ticks, state } = ticks;
        let check = (ticks.into_iter())
            .map(|tick| state.format(&tick))
            .collect::<Vec<_>>();
        assert_eq!(check, expected);
    }

    fn mk_span<XY: Tick>(width: f64) -> impl Span<XY> {
        HorizontalSpan::new(6.0, 0, 2.0, width, HorizontalSpan::identity_format())
    }

    #[test]
    fn test_timestamp_generator() {
        let gen = Timestamps::from_periods(Period::all());
        let first = Utc.with_ymd_and_hms(2014, 3, 1, 0, 0, 0).unwrap();
        let last = Utc.with_ymd_and_hms(2018, 7, 5, 0, 0, 0).unwrap();
        // Just years
        assert_ticks(
            gen.generate(&first, &last, &mk_span((4.0 * 6.0 + 4.0) * 4.0)),
            vec!["2015", "2016", "2017", "2018"],
        );
        // One year: shows mid-year and filled with months
        let last = Utc.with_ymd_and_hms(2015, 3, 5, 0, 0, 0).unwrap();
        assert_ticks(
            gen.generate(&first, &last, &mk_span((4.0 * 6.0 + 4.0) * 14.0)),
            vec![
                "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec", "2015",
                "Feb", "Mar",
            ],
        );
        // Drill down on the edge
        let first = Utc.with_ymd_and_hms(2015, 1, 1, 0, 0, 0).unwrap();
        let last = Utc
            .with_ymd_and_hms(2015, 1, 1, 0, 0, 0)
            .unwrap()
            .with_nanosecond(1)
            .unwrap();
        assert_ticks(
            gen.generate(&first, &last, &mk_span(1000000.0)),
            vec!["2015"],
        );
        // Again but show ns
        let last = Utc
            .with_ymd_and_hms(2015, 1, 1, 0, 0, 0)
            .unwrap()
            .with_nanosecond(3)
            .unwrap();
        assert_ticks(
            gen.generate(&first, &last, &mk_span(1000000.0)),
            vec!["2015", "00:00:00.000000001", "00:00:00.000000002"],
        );
        // Sampling
        let first = Utc.with_ymd_and_hms(2005, 3, 5, 6, 0, 0).unwrap();
        let last = Utc.with_ymd_and_hms(2005, 3, 5, 12, 0, 0).unwrap();
        assert_ticks(
            gen.generate(&first, &last, &mk_span((5.0 * 6.0 + 4.0) * 3.0)),
            vec!["07:00", "09:00", "11:00"],
        );
    }

    #[test]
    fn test_timestamp_generator_zero() {
        let gen = Timestamps::from_periods(Period::all());
        let first = DateTime::<Utc>::from_timestamp(0, 0).unwrap();
        let last = DateTime::<Utc>::from_timestamp(12, 6_000_000).unwrap();
        assert_ticks(
            gen.generate(&first, &last, &mk_span(1000.0)),
            vec![
                "1970", "00:00:01", "00:00:02", "00:00:03", "00:00:04", "00:00:05", "00:00:06",
                "00:00:07", "00:00:08", "00:00:09", "00:00:10", "00:00:11", "00:00:12",
            ],
        );
    }

    #[test]
    fn test_timestamp_generator_no_range() {
        let gen = Timestamps::from_periods(Period::all());
        let dt = Utc.with_ymd_and_hms(2015, 1, 1, 0, 0, 0).unwrap();
        assert_ticks(gen.generate(&dt, &dt, &mk_span(1000.0)), vec![])
    }

    #[test]
    fn test_gen_small_space() {
        let gen = Timestamps::from_periods(Period::all());
        let first = DateTime::<Utc>::from_timestamp(0, 0).unwrap();
        let last = DateTime::<Utc>::from_timestamp(0, 3_000_000).unwrap();
        assert_ticks(gen.generate(&first, &last, &mk_span(10.0)), vec![]);
    }

    #[test]
    fn test_timestamps_empty_periods() {
        let gen = Timestamps::from_periods([]);
        let first = Utc.with_ymd_and_hms(2014, 3, 1, 0, 0, 0).unwrap();
        let last = Utc.with_ymd_and_hms(2018, 7, 5, 0, 0, 0).unwrap();
        assert_ticks(gen.generate(&first, &last, &mk_span(1000.0)), vec![]);
    }

    #[test]
    fn test_sample_ticks() {
        let f = Timestamps::<Utc>::sample_ticks::<u32>;
        assert_eq!(f(vec![0, 1, 2, 3, 4, 5], 0, 1), vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(f(vec![0, 1, 2, 3, 4, 5], 1, 2), vec![1, 3, 5]);
        assert_eq!(f(vec![0, 1, 2, 3, 4, 5], 2, 3), vec![2, 5]);
        assert_eq!(f(vec![0, 1, 2, 3, 4, 5], 3, 4), vec![3]);
        assert_eq!(f(vec![0, 1, 2, 3, 4, 5], 4, 5), vec![4]);
        assert_eq!(f(vec![0, 1, 2, 3, 4, 5], 5, 6), vec![5]);
        assert_eq!(f(vec![0, 1, 2, 3, 4, 5], 6, 7), Vec::<u32>::new());
        assert_eq!(f((0..60).collect(), 0, 1), (0..60).collect::<Vec<_>>());
        for i in 1..100 {
            assert_eq!(f((0..60).collect(), i - 1, i).len(), 60 / i, "i={i}");
        }
    }

    #[test]
    fn test_period_iter_range() {
        assert_eq!(
            Period::Year
                .iter_aligned_range(
                    Utc.with_ymd_and_hms(2014, 3, 1, 0, 0, 0).unwrap(),
                    Utc.with_ymd_and_hms(2018, 7, 5, 0, 0, 0).unwrap()
                )
                .map(|dt| {
                    assert_eq!(dt.month(), 1);
                    assert_eq!(dt.day(), 1);
                    assert_eq!(dt.hour(), 0);
                    assert_eq!(dt.minute(), 0);
                    assert_eq!(dt.second(), 0);
                    assert_eq!(dt.nanosecond(), 0);
                    dt.year()
                })
                .collect::<Vec<_>>(),
            vec![2015, 2016, 2017, 2018]
        );
        assert!(Period::Month
            .iter_aligned_range(
                Utc.with_ymd_and_hms(2014, 3, 5, 0, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2014, 3, 30, 0, 0, 0).unwrap()
            )
            .map(|dt| dt.month())
            .collect::<Vec<_>>()
            .is_empty(),);
        assert_eq!(
            Period::Second
                .iter_aligned_range(
                    Utc.with_ymd_and_hms(2027, 4, 5, 1, 2, 57).unwrap(),
                    Utc.with_ymd_and_hms(2027, 4, 5, 1, 3, 7).unwrap()
                )
                .map(|dt| dt.second())
                .collect::<Vec<_>>(),
            vec![57, 58, 59, 0, 1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn test_period_truncate_at() {
        let at = Utc
            .with_ymd_and_hms(2014, 2, 3, 4, 5, 6)
            .unwrap()
            .with_nanosecond(7)
            .unwrap();
        assert_eq!(
            Period::Year.truncate_at(at).unwrap(),
            Utc.with_ymd_and_hms(2014, 1, 1, 0, 0, 0).unwrap()
        );
        assert_eq!(
            Period::Month.truncate_at(at).unwrap(),
            Utc.with_ymd_and_hms(2014, 2, 1, 0, 0, 0).unwrap()
        );
        assert_eq!(
            Period::Day.truncate_at(at).unwrap(),
            Utc.with_ymd_and_hms(2014, 2, 3, 0, 0, 0).unwrap()
        );
        assert_eq!(
            Period::Hour.truncate_at(at).unwrap(),
            Utc.with_ymd_and_hms(2014, 2, 3, 4, 0, 0).unwrap()
        );
        assert_eq!(
            Period::Minute.truncate_at(at).unwrap(),
            Utc.with_ymd_and_hms(2014, 2, 3, 4, 5, 0).unwrap()
        );
        let no_nanos = Utc.with_ymd_and_hms(2014, 2, 3, 4, 5, 6).unwrap();
        assert_eq!(Period::Second.truncate_at(at).unwrap(), no_nanos);
        assert_eq!(Period::Millisecond.truncate_at(at).unwrap(), no_nanos);
        assert_eq!(Period::Microsecond.truncate_at(at).unwrap(), no_nanos);
        assert_eq!(
            Period::Nanosecond.truncate_at(at).unwrap(),
            Utc.with_ymd_and_hms(2014, 2, 3, 4, 5, 6,)
                .unwrap()
                .with_nanosecond(7)
                .unwrap()
        );
    }
}
