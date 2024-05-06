use super::Colour;
use leptos::*;

/// A gradient of colours. Maps to a [ColourScheme]
pub type SequentialGradient = (Colour, &'static [Colour]);
/// A diverging gradient of colours i.e., a gradient that tends to a central value then a second gradient away. Maps to a [ColourScheme]. Use with [Line::with_gradient](crate::Line::with_gradient).
pub type DivergingGradient = (SequentialGradient, SequentialGradient);

/// A colour scheme with at least one colour.
#[derive(Clone, Debug, PartialEq)]
pub struct ColourScheme {
    // Must have at least one colour
    swatches: Vec<Colour>,
    // Index of the zero value in a diverging gradient. If None, the zero value is not used.
    // TODO: collect more colour scheme uses and convert schemes into an enum / trait
    zero: Option<usize>,
}

impl ColourScheme {
    /// Create a new colour scheme with the given colours. Must have at least one colour.
    pub fn new(first: Colour, rest: impl IntoIterator<Item = Colour>) -> Self {
        Self {
            swatches: std::iter::once(first).chain(rest).collect(),
            zero: None,
        }
    }

    /// Creates a diverging colour scheme value from two sequential gradients. For use with [Line::with_gradient](crate::Line::with_gradient).
    ///
    /// A diverging colour scheme is useful for data that has a central value. For example, a temperature scale with a central value of 0°C. Assuming a light background the `before` scheme could then be blue to a black while `after` could be black to red.
    ///
    /// Special care should be taken about passing before and after parameters. The `before` scheme should be ordered from a colour to a centric value and vice versa for `after` with centric value to a colour. The centric value should be a dark colour on a light background.
    pub fn diverging_gradient(below_zero: Self, above_zero: Self) -> Self {
        let zero = below_zero.swatches.len();
        Self {
            swatches: below_zero
                .swatches
                .into_iter()
                .chain(above_zero.swatches)
                .collect(),
            zero: Some(zero),
        }
    }

    fn get_index(&self, index: usize) -> usize {
        // Note: not using checked_rem_euclid as we're guaranteed to have at least one colour
        index.rem_euclid(self.swatches.len())
    }

    /// Get the colour at the given index. Indexes are wrapped around the number of swatches.
    pub fn by_index(&self, index: usize) -> Colour {
        let index = self.get_index(index);
        self.swatches[index]
    }

    /// Set the colour at the given index. Indexes are wrapped around the number of swatches.
    pub fn set_by_index(&mut self, index: usize, colour: Colour) {
        let index = self.get_index(index);
        self.swatches[index] = colour;
    }

    /// Invert the colour scheme. Useful for changing the direction of a gradient. All Chartistry's default colour palettes assume a light background.
    ///
    /// On a light background you should aim to have the lightest colour first and the darkest last. Vice versa for a dark background.
    pub fn invert(self) -> Self {
        let mut swatches = self.swatches.clone();
        swatches.reverse();
        Self {
            swatches,
            zero: self.zero,
        }
    }

    fn line_to_prior_swatch_index(&self, line: usize, total: usize) -> usize {
        // Avoid divide by zero and preference first
        if total <= 1 {
            return 0;
        }
        let line = line.clamp(0, total - 1);
        let swatches = self.swatches.len();
        // Spread lines over swatches such that first and last line correspond
        // We deduct one from line total as otherwise skip the last swatch
        // The last line is special as it should always be the last swatch
        if line == total - 1 {
            return swatches - 1;
        }
        (line as f64 / (total as f64 - 1.0) * swatches as f64) as usize
    }

    /// Interpolate between the colours in the scheme. The line is the current line and the total is the total number of lines. Picks the two colours before and after the line and interpolates between them.
    pub fn interpolate(&self, line: usize, total: usize) -> Colour {
        let before_i = self.line_to_prior_swatch_index(line, total);
        // Last swatch? Can't interpolate so return it
        if before_i == self.swatches.len() - 1 {
            return self.swatches[before_i];
        }
        // Look up colours before and after
        let before = self.swatches[before_i];
        let after = self.swatches[before_i + 1];
        // Find ratio between the two
        let lines_per_swatch = total as f64 / self.swatches.len() as f64;
        let before_line = (before_i as f64 * lines_per_swatch) as usize;
        let ratio = ((line - before_line) as f64) / lines_per_swatch;

        before.interpolate(after, ratio)
    }
}

#[component]
pub fn LinearGradientSvg(
    #[prop(into)] id: AttributeValue,
    scheme: Signal<ColourScheme>,
    range_y: Signal<Option<(f64, f64)>>,
) -> impl IntoView {
    view! {
        <linearGradient id=Some(id) x1="0%" y1="100%" x2="0%" y2="0%">
            {move || scheme.get().stops(range_y.get().unwrap_or_default())}
        </linearGradient>
    }
}

impl ColourScheme {
    fn stops(&self, range_y: (f64, f64)) -> impl IntoView {
        // TODO: collect more colour scheme uses and convert schemes into an enum / trait
        if self.zero.is_some() {
            self.diverging_stops(range_y).into_view()
        } else {
            self.sequential_stops().into_view()
        }
    }

    // Stops for a sequential gradient. Evenly spreads the swatches over 0% to 100%.
    fn sequential_stops(&self) -> impl IntoView {
        let step = 1.0 / self.swatches.len().saturating_sub(1) as f64;
        generate_stops(&self.swatches, 0.0, step)
    }

    // Stops for a diverging gradient. Finds the zero value and spreads the swatches over 0% to zero and zero to 100%.
    fn diverging_stops(&self, (bottom_y, top_y): (f64, f64)) -> impl IntoView {
        // Find zero value as a % of the range (0.0 to 1.0)
        let zero = (1.0 - (-bottom_y) / (top_y - bottom_y)).clamp(0.0, 1.0);
        // Separate swatches
        let (below_zero, above_zero) = self.diverging_swatches();
        // Determine step size
        //let step = 1.0 / self.swatches.len() as f64;
        let below_step = zero * 1.0 / below_zero.len() as f64;
        let above_step = (1.0 - zero) * 1.0 / above_zero.len() as f64;
        // Start at the midpoint of first step so that offset is in the middle of the step
        let below_start = below_step / 2.0;
        let above_start = above_step / 2.0 + below_zero.len() as f64 * below_step;
        view! {
            {generate_stops(below_zero, below_start, below_step)}
            {generate_stops(above_zero, above_start, above_step)}
        }
    }

    // Separate the swatches into two halves at the zero value. The first half is below zero and the second half is the rest (zero and above). If not a diverging gradient, all swatches will be seen as above zero.
    fn diverging_swatches(&self) -> (&[Colour], &[Colour]) {
        if let Some(zero_index) = self.zero {
            self.swatches.split_at(zero_index)
        } else {
            (&[], &self.swatches)
        }
    }
}

// Generates a <stop> for each swatch. Offset is generated using `from + i * step` where i is the index of the swatch. The offset is formatted as a percentage (0% to 100%). `from` and `step` must be 0.0 to 1.0.
fn generate_stops(swatches: &[Colour], from: f64, step: f64) -> impl IntoView {
    swatches
        .iter()
        .enumerate()
        // % of the index (0.0 - 1.0)
        .map(|(i, colour)| (from + i as f64 * step, colour))
        // Keep percentages in range
        .filter(|&(percent, _)| percent > 0.0 && percent < 1.0)
        .map(|(percent, colour)| {
            // Format as a percentage (0% - 100%)
            let offset = format!("{:.2}%", percent * 100.0);
            view! {
                <stop offset=offset stop-color=colour />
            }
        })
        .collect_view()
}

impl From<SequentialGradient> for ColourScheme {
    fn from((first, rest): (Colour, &[Colour])) -> Self {
        Self::new(first, rest.to_vec())
    }
}

impl From<DivergingGradient> for ColourScheme {
    fn from((below_zero, above_zero): DivergingGradient) -> Self {
        let below_zero = below_zero.into();
        let above_zero = above_zero.into();
        Self::diverging_gradient(below_zero, above_zero)
    }
}

macro_rules! from_array_to_colour_scheme {
    ($($n:literal),*) => {
        $(
            impl From<[Colour; $n]> for ColourScheme {
                fn from(colours: [Colour; $n]) -> Self {
                    Self::new(colours[0], (&colours[1..]).to_vec())
                }
            }
        )*
    };
}
from_array_to_colour_scheme!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);

#[cfg(test)]
mod tests {
    use super::*;

    fn scheme3() -> ColourScheme {
        ColourScheme::from([
            Colour::from_rgb(0, 0, 0),
            Colour::from_rgb(255, 255, 255),
            Colour::from_rgb(0, 0, 0),
        ])
    }

    const SCHEME10: [Colour; 10] = [
        Colour::from_rgb(0, 0, 0),
        Colour::from_rgb(255, 255, 255),
        Colour::from_rgb(0, 0, 0),
        Colour::from_rgb(255, 255, 255),
        Colour::from_rgb(0, 0, 0),
        Colour::from_rgb(255, 255, 255),
        Colour::from_rgb(0, 0, 0),
        Colour::from_rgb(255, 255, 255),
        Colour::from_rgb(0, 0, 0),
        Colour::from_rgb(255, 255, 255),
    ];
    fn scheme10() -> ColourScheme {
        ColourScheme::from(SCHEME10)
    }

    #[test]
    fn test_colour_scheme() {
        let scheme = scheme3();
        assert_eq!(scheme.by_index(0), Colour::from_rgb(0, 0, 0));
        assert_eq!(scheme.by_index(1), Colour::from_rgb(255, 255, 255));
        assert_eq!(scheme.by_index(2), Colour::from_rgb(0, 0, 0));
    }

    #[test]
    fn test_line_to_swatch_index() {
        let scheme3 = scheme3();
        let scheme10 = scheme10();
        // More swatches than lines
        assert_eq!(scheme10.line_to_prior_swatch_index(0, 2), 0);
        assert_eq!(scheme10.line_to_prior_swatch_index(1, 2), 9);
        assert_eq!(scheme10.line_to_prior_swatch_index(0, 1), 0); // Preference first
        assert_eq!(scheme10.line_to_prior_swatch_index(0, 5), 0);
        assert_eq!(scheme10.line_to_prior_swatch_index(1, 5), 2);
        assert_eq!(scheme10.line_to_prior_swatch_index(2, 5), 5);
        assert_eq!(scheme10.line_to_prior_swatch_index(3, 5), 7);
        assert_eq!(scheme10.line_to_prior_swatch_index(4, 5), 9);
        assert_eq!(scheme3.line_to_prior_swatch_index(0, 1), 0);
        assert_eq!(scheme3.line_to_prior_swatch_index(1, 2), 2);
        // Same number of swatches and lines
        assert_eq!(scheme3.line_to_prior_swatch_index(0, 3), 0);
        assert_eq!(scheme3.line_to_prior_swatch_index(1, 3), 1);
        assert_eq!(scheme3.line_to_prior_swatch_index(2, 3), 2);
        // More lines than swatches
        assert_eq!(scheme3.line_to_prior_swatch_index(0, 5), 0);
        assert_eq!(scheme3.line_to_prior_swatch_index(1, 5), 0);
        assert_eq!(scheme3.line_to_prior_swatch_index(2, 5), 1);
        assert_eq!(scheme3.line_to_prior_swatch_index(3, 5), 2);
        assert_eq!(scheme3.line_to_prior_swatch_index(4, 5), 2);
        assert_eq!(scheme3.line_to_prior_swatch_index(0, 9), 0);
        assert_eq!(scheme3.line_to_prior_swatch_index(1, 9), 0);
        assert_eq!(scheme3.line_to_prior_swatch_index(2, 9), 0);
        assert_eq!(scheme3.line_to_prior_swatch_index(3, 9), 1);
        assert_eq!(scheme3.line_to_prior_swatch_index(4, 9), 1);
        assert_eq!(scheme3.line_to_prior_swatch_index(5, 9), 1);
        assert_eq!(scheme3.line_to_prior_swatch_index(6, 9), 2);
        assert_eq!(scheme3.line_to_prior_swatch_index(7, 9), 2);
        assert_eq!(scheme3.line_to_prior_swatch_index(8, 9), 2);
        // Bigger
        assert_eq!(scheme10.line_to_prior_swatch_index(37, 100), 3);
        // Clamp if line is too big
        assert_eq!(scheme10.line_to_prior_swatch_index(3, 3), 9);
        assert_eq!(scheme10.line_to_prior_swatch_index(4, 3), 9);
        // First swatch if no lines
        assert_eq!(scheme10.line_to_prior_swatch_index(0, 0), 0);
        assert_eq!(scheme10.line_to_prior_swatch_index(1, 0), 0);
    }

    #[test]
    fn test_scheme_interpolation() {
        let scheme3 = scheme3();
        let scheme10 = scheme10();
        // One to one mapping of swatches to lines
        assert_eq!(scheme3.interpolate(0, 3), Colour::from_rgb(0, 0, 0));
        assert_eq!(scheme3.interpolate(1, 3), Colour::from_rgb(255, 255, 255));
        assert_eq!(scheme3.interpolate(2, 3), Colour::from_rgb(0, 0, 0));
        // More lines than swatches
        assert_eq!(scheme3.interpolate(0, 9), Colour::from_rgb(0, 0, 0));
        assert_eq!(scheme3.interpolate(1, 9), Colour::from_rgb(85, 85, 85));
        assert_eq!(scheme3.interpolate(2, 9), Colour::from_rgb(170, 170, 170));
        assert_eq!(scheme3.interpolate(3, 9), Colour::from_rgb(255, 255, 255));
        assert_eq!(scheme3.interpolate(4, 9), Colour::from_rgb(170, 170, 170));
        assert_eq!(scheme3.interpolate(5, 9), Colour::from_rgb(85, 85, 85));
        assert_eq!(scheme3.interpolate(6, 9), Colour::from_rgb(0, 0, 0));
        assert_eq!(scheme3.interpolate(7, 9), Colour::from_rgb(0, 0, 0));
        assert_eq!(scheme3.interpolate(8, 9), Colour::from_rgb(0, 0, 0));
        // More swatches than lines
        assert_eq!(scheme10.interpolate(0, 1), Colour::from_rgb(0, 0, 0));
        assert_eq!(scheme10.interpolate(0, 2), Colour::from_rgb(0, 0, 0));
        assert_eq!(scheme10.interpolate(1, 2), Colour::from_rgb(255, 255, 255));
        assert_eq!(scheme10.interpolate(0, 3), Colour::from_rgb(0, 0, 0));
        assert_eq!(scheme10.interpolate(1, 3), Colour::from_rgb(255, 255, 255));
        assert_eq!(scheme10.interpolate(2, 3), Colour::from_rgb(255, 255, 255));
        assert_eq!(scheme10.interpolate(2, 5), Colour::from_rgb(255, 255, 255));
        assert_eq!(scheme10.interpolate(2, 8), Colour::from_rgb(255, 255, 255));
    }
}
