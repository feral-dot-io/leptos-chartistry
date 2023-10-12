use super::gen::{GeneratedTicks, Span, TickGen, TickState};

/// Generates a vector of aligned, "nice" floats. The vector will contain `count` values between `from` and `to` inclusive. Returned ticks will be aligned to "nice" values in powers of 10. e.g., gen_nice_floats(0.1, 0.3, 3) -> [0.1, 0.2, 0.3].
///
/// If count is 1 or 0, returns the midpoint of the range.
///
/// Given a range (from - to), produce a series of evenly spaced ticks whose steps are the smallest values possible that are still distinguishable from each other. This results in "nice" rounded values that are easy to read. This is done by determining the scale (powers of 10) of the range and count to calculate a step size. The step size is then used to produce a series of evenly spaced ticks which get formatted to our desired precision.
#[derive(Clone, Debug, PartialEq)]
pub struct AlignedFloatsGen;

#[derive(Clone, Debug, PartialEq)]
pub struct AlignedFloats {
    min_length: usize,
    scale: isize,
}

impl TickGen for AlignedFloatsGen {
    type Tick = f64;

    fn generate(
        &self,
        &first: &Self::Tick,
        &last: &Self::Tick,
        span: Box<dyn Span>,
    ) -> GeneratedTicks<Self::Tick> {
        let (scale, count) = Self::find_precision(first, last, span);
        let (scale, ticks) = Self::generate_count(first, last, scale, count);
        let state = AlignedFloats::new(scale, &ticks);
        GeneratedTicks {
            ticks,
            state: Box::new(state),
        }
    }
}

impl AlignedFloatsGen {
    pub fn new() -> Self {
        Self
    }

    /// Returns the scale and count to use for the given range and span
    fn find_precision(first: f64, last: f64, span: Box<dyn Span>) -> (isize, usize) {
        // Determine scale e.g., are we in the 100s, 10s, 0.1s, etc. Then display one more (-1)
        let mut scale = scale10(last - first) - 1;
        // Naively calculate our count i.e., how many ticks can we fit in the span. This is the lower bound for count
        let mock = Self::mock_value(first, last, scale);
        let lower_count = (span.length() / span.consumed(&[mock.as_str()])) as usize;
        // Lower the scale (increase precision) so that we can always distinguish between ticks e.g., a range of 0-10 with a count of 30 would result in runs of 0.1. Subtract 2 to account for first and last before the jump to a higher precision
        scale -= scale10(lower_count as f64 - 2.0);
        // Calculate the upper count bound. The max number of ticks we can fit in the span with the higher precision
        let mock = Self::mock_value(first, last, scale);
        let upper_count = (span.length() / span.consumed(&[mock.as_str()])) as usize;

        (scale, upper_count)
    }

    /// Finds the longest string that could be displayed between first and last inclusive
    fn mock_value(first: f64, last: f64, scale: isize) -> String {
        let first = format(scale, first);
        let last = format(scale, last);
        if first.len() > last.len() {
            first
        } else {
            last
        }
    }

    fn generate_count(first: f64, last: f64, scale: isize, count: usize) -> (isize, Vec<f64>) {
        let range = last - first;
        // If count is too small, return a single tick in the middle
        if count <= 1 {
            return (scale, vec![first + range / 2.0]);
        }
        // Determine step size: use one count fewer so that we include from and to
        let step = range / (count - 1) as f64;
        let ticks = (0..count)
            .into_iter()
            .map(|i| {
                // Avoid f64 accumulation errors
                first + i as f64 * step
            })
            .collect::<Vec<_>>();
        (scale, ticks)
    }
}

impl AlignedFloats {
    pub fn new(scale: isize, ticks: &[f64]) -> AlignedFloats {
        // Find longest tick
        // Note: this is inefficient as we call format multiple times. We could do this in one pass if we didn't return Vec<f64> in TickGen
        let min_length = ticks
            .iter()
            .map(|tick| format(scale, *tick).len())
            .max()
            .unwrap_or(0);
        Self { min_length, scale }
    }

    fn format(&self, value: f64) -> String {
        let mut label = format(self.scale, value);
        // Left pad with spaces to maintain minimum size
        let spaces = " ".repeat(self.min_length.saturating_sub(label.len()));
        label.insert_str(0, &spaces);
        label
    }
}

impl TickState for AlignedFloats {
    type Tick = f64;

    fn position(&self, value: &Self::Tick) -> f64 {
        *value
    }

    fn short_format(&self, &value: &Self::Tick) -> String {
        self.format(value)
    }

    fn long_format(&self, &value: &Self::Tick) -> String {
        self.format(value)
    }
}

fn format(scale: isize, value: f64) -> String {
    let precision = if scale < 0 { -scale as usize } else { 0 };
    let mut val = format!("{value:.precision$}");
    // The format! macro doesn't handle negative precision. For us, this means zero pad to the left of the decimal point
    if scale > 0 {
        // Clamp scale to leave leftmost digit if it's too large
        let neg_offset = if val.starts_with("-") { 1 } else { 0 };
        let scale = (scale as usize).min(val.len() - 1 - neg_offset);
        // Truncate from offset to the end with zeros
        val.len()
            .checked_sub(scale as usize)
            .map(|offset| val.replace_range(offset.., &"0".repeat(scale as usize)));
    }
    val
}

/// Determines the scale e.g. are we in the 10s, 100s, 0.1s, etc.
fn scale10(range: f64) -> isize {
    let scale = range.abs().log10().floor();
    if scale.is_infinite() {
        0
    } else {
        scale as isize
    }
}

#[cfg(test)]
mod tests {
    use super::super::use_ticks::HorizontalSpan;
    use super::*;

    fn mk_span(width: f64) -> Box<dyn Span> {
        Box::new(HorizontalSpan::new(1.0, 0.0, width + 0.1))
    }

    fn assert_precision(first: f64, last: f64, width: f64, scale: isize, count: usize) {
        let span = mk_span(width + 1.0);
        let precision = AlignedFloatsGen::find_precision(first, last, span);
        assert_eq!(precision, (scale, count));
    }

    fn assert_ticks(
        first: f64,
        last: f64,
        scale: isize,
        count: usize,
        expected: Vec<&'static str>,
    ) {
        let (scale, ticks) = AlignedFloatsGen::generate_count(first, last, scale, count);
        let state = AlignedFloats::new(scale, &ticks);
        let ticks = (ticks.into_iter())
            .map(|tick| state.short_format(&tick))
            .collect::<Vec<_>>();
        assert_eq!(ticks, expected);
    }

    #[test]
    fn test_find_precision() {
        assert_precision(0.0, 1.0, 3.0 * 3.0, -1, 3);
        assert_precision(0.0, 1.0, 6.0 * 3.0, -1, 6);
        // Max spread before we need a higher scale (e.g., 0.x to 0.0x)
        assert_precision(0.0, 1.0, 11.0 * 3.0, -1, 11);
        assert_precision(0.0, 1.0, 12.0 * 4.0, -2, 12);
        // Larger spread on a higher scale
        assert_precision(0.0, 1.0, 21.0 * 4.0, -2, 21);
        // Larger whole numbers
        assert_precision(0.0, 1000.0, 3.0 * 4.0, 2, 3);
        // Larger spread with negative numbers
        assert_precision(-100_000.0, 100_000.0, 3.0 * 7.0, 4, 3);

        // Count of 1
        assert_precision(0.0, 1.0, 3.0, -1, 1);
        assert_precision(123.0, 133.0, 3.0, 0, 1);
        assert_precision(0.0, 100_000.0, 6.0, 4, 1);
        assert_precision(1.234_567, 2.987_654, 3.0, -1, 1);
    }

    #[test]
    fn test_generate() {
        // Basic spread
        assert_ticks(0.0, 1.0, -1, 3, vec!["0.0", "0.5", "1.0"]);
        // Larger spread
        let exp = vec!["0.0", "0.2", "0.4", "0.6", "0.8", "1.0"];
        assert_ticks(0.0, 1.0, -1, 6, exp);
        // Max spread before we need a higher scale (e.g., 0.x to 0.0x)
        let exp = vec![
            "0.0", "0.1", "0.2", "0.3", "0.4", "0.5", "0.6", "0.7", "0.8", "0.9", "1.0",
        ];
        assert_ticks(0.0, 1.0, -1, 11, exp);
        // Add one to induce a higher scale
        let exp = vec![
            "0.00", "0.09", "0.18", "0.27", "0.36", "0.45", "0.55", "0.64", "0.73", "0.82", "0.91",
            "1.00",
        ];
        assert_ticks(0.0, 1.0, -2, 12, exp);
        // Larger spread on a higher scale
        let exp = vec![
            "0.00", "0.05", "0.10", "0.15", "0.20", "0.25", "0.30", "0.35", "0.40", "0.45", "0.50",
            "0.55", "0.60", "0.65", "0.70", "0.75", "0.80", "0.85", "0.90", "0.95", "1.00",
        ];
        assert_ticks(0.0, 1.0, -2, 21, exp);

        // Larger whole numbers
        assert_ticks(0.0, 1000.0, 2, 3, vec!["   0", " 500", "1000"]);
        // Larger spread with negative numbers
        assert_ticks(
            -100_000.0,
            100_000.0,
            4,
            3,
            vec!["-100000", "      0", " 100000"],
        );

        // Count of 1
        assert_ticks(0.0, 1.0, -1, 1, vec!["0.5"]);
        assert_ticks(123.0, 133.0, 0, 1, vec!["128"]);
        assert_ticks(0.0, 100_000.0, 4, 1, vec!["50000"]);
        assert_ticks(1.234_567, 2.987_654, -1, 1, vec!["2.1"]);
    }

    #[test]
    fn test_generate_large_range() {
        assert_ticks(
            0.0,
            212801815895.28098,
            9,
            16,
            vec![
                "           0",
                " 14000000000",
                " 28000000000",
                " 42000000000",
                " 56000000000",
                " 70000000000",
                " 85000000000",
                " 99000000000",
                "113000000000",
                "127000000000",
                "141000000000",
                "156000000000",
                "170000000000",
                "184000000000",
                "198000000000",
                "212000000000",
            ],
        );
    }

    #[test]
    fn test_format() {
        // Significant digits
        assert_eq!(format(0, 1.0), "1");
        assert_eq!(format(0, 1.23456789), "1");
        assert_eq!(format(0, 123_456.123), "123456");
        assert_eq!(format(1, 12.345_678), "10");
        assert_eq!(format(1, 123_456.123_456), "123450");
        assert_eq!(format(3, 123_456.123_456), "123000");
        assert_eq!(format(8, 123_456_789.0), "100000000");
        assert_eq!(format(1, 0.123_456_789), "0");
        assert_eq!(format(10, 0.123_456_789), "0");
        // Decimal places
        assert_eq!(format(-1, 0.123_456_789), "0.1");
        assert_eq!(format(-5, 0.123_456_789), "0.12346"); // Rounding
        assert_eq!(format(-5, 0.123_44), "0.12344");
        // Negative
        assert_eq!(format(0, -1.0), "-1");
        assert_eq!(format(3, -123_456.789), "-123000");
        assert_eq!(format(-3, -123_456.789123), "-123456.789");
        assert_eq!(format(1, -0.123_456_789), "-0");

        // Scale too large? Clamp to just leave first digit
        assert_eq!(format(6, 123_456.123_456), "100000");
        assert_eq!(format(6, -123_456.123_456), "-100000");
        assert_eq!(format(5, 123_456.123_456), "100000");
        assert_eq!(format(9, 123_456.123_456), "100000");

        // Extremes
        let v = format(3, f64::MAX);
        assert!(v.starts_with("1797"));
        assert!(v.ends_with("858000"));
        assert_eq!(v.len(), f64::MAX_10_EXP as usize + 1);
        let v = format(3, f64::MIN);
        assert!(v.starts_with("-1797"));
        assert!(v.ends_with("858000"));
        assert_eq!(v.len(), f64::MAX_10_EXP as usize + 2);
        assert_eq!(format(-3, f64::MIN_POSITIVE), "0.000");
        assert_eq!(format(3, f64::MIN_POSITIVE), "0");
    }

    #[test]
    fn test_scale() {
        assert_eq!(scale10(1.0), 0);
        assert_eq!(scale10(-1.0), 0);
        assert_eq!(scale10(0.0), 0);
        assert_eq!(scale10(10.0), 1);
        assert_eq!(scale10(100.0), 2);
        assert_eq!(scale10(55.0), 1);
        assert_eq!(scale10(1_123_567.789_654_321), 6);
        assert_eq!(scale10(-1_123_567.789_654_321), 6);
        assert_eq!(scale10(0.1), -1);
        assert_eq!(scale10(0.01), -2);
        assert_eq!(scale10(0.55), -1);
        assert_eq!(scale10(0.000_000_009_123), -9);
        assert_eq!(scale10(f64::MAX), 308);
        assert_eq!(scale10(f64::MIN), 308);
        assert_eq!(scale10(f64::MIN_POSITIVE), -308);
    }

    #[test]
    fn test_derived() {
        // TODO
        //format!("{:?}", AlignedFloatTicks::new(123, vec![123.456]).clone());
    }
}
