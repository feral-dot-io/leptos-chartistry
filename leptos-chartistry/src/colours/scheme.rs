use super::Colour;
use leptos::*;

/// A colour scheme with at least one colour.
#[derive(Clone, Debug, PartialEq)]
pub struct ColourScheme {
    // Must have at least one colour
    swatches: Vec<Colour>,
}

impl ColourScheme {
    /// Create a new colour scheme with the given colours. Must have at least one colour.
    pub fn new(first: Colour, rest: impl IntoIterator<Item = Colour>) -> Self {
        Self {
            swatches: std::iter::once(first).chain(rest).collect(),
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
        Self { swatches }
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
pub fn LinearGradient(
    #[prop(into)] id: AttributeValue,
    #[prop(into)] colour: Signal<ColourScheme>,
) -> impl IntoView {
    let stops = move || {
        let swatches = colour.get().swatches;
        // Spread <stop> over swatches so first is 0% and last is 100%. Use 0% if only one swatch
        let spread = (swatches.len() - 1).max(1) as f64;
        colour
            .get()
            .swatches
            .into_iter()
            .enumerate()
            .map(|(i, colour)| {
                let percent = (i as f64 / spread) * 100.0;
                let offset = format!("{percent:.2}%");
                view! {
                    <stop offset=offset stop-color=colour />
                }
            })
            .collect_view()
    };
    view! {
        <linearGradient id=Some(id) gradientTransform="rotate(90)">
            {stops}
        </linearGradient>
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
            Colour::new(0, 0, 0),
            Colour::new(255, 255, 255),
            Colour::new(0, 0, 0),
        ])
    }

    const SCHEME10: [Colour; 10] = [
        Colour::new(0, 0, 0),
        Colour::new(255, 255, 255),
        Colour::new(0, 0, 0),
        Colour::new(255, 255, 255),
        Colour::new(0, 0, 0),
        Colour::new(255, 255, 255),
        Colour::new(0, 0, 0),
        Colour::new(255, 255, 255),
        Colour::new(0, 0, 0),
        Colour::new(255, 255, 255),
    ];
    fn scheme10() -> ColourScheme {
        ColourScheme::from(SCHEME10)
    }

    #[test]
    fn test_colour_scheme() {
        let scheme = scheme3();
        assert_eq!(scheme.by_index(0), Colour::new(0, 0, 0));
        assert_eq!(scheme.by_index(1), Colour::new(255, 255, 255));
        assert_eq!(scheme.by_index(2), Colour::new(0, 0, 0));
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
        assert_eq!(scheme3.interpolate(0, 3), Colour::new(0, 0, 0));
        assert_eq!(scheme3.interpolate(1, 3), Colour::new(255, 255, 255));
        assert_eq!(scheme3.interpolate(2, 3), Colour::new(0, 0, 0));
        // More lines than swatches
        assert_eq!(scheme3.interpolate(0, 9), Colour::new(0, 0, 0));
        assert_eq!(scheme3.interpolate(1, 9), Colour::new(85, 85, 85));
        assert_eq!(scheme3.interpolate(2, 9), Colour::new(170, 170, 170));
        assert_eq!(scheme3.interpolate(3, 9), Colour::new(255, 255, 255));
        assert_eq!(scheme3.interpolate(4, 9), Colour::new(170, 170, 170));
        assert_eq!(scheme3.interpolate(5, 9), Colour::new(85, 85, 85));
        assert_eq!(scheme3.interpolate(6, 9), Colour::new(0, 0, 0));
        assert_eq!(scheme3.interpolate(7, 9), Colour::new(0, 0, 0));
        assert_eq!(scheme3.interpolate(8, 9), Colour::new(0, 0, 0));
        // More swatches than lines
        assert_eq!(scheme10.interpolate(0, 1), Colour::new(0, 0, 0));
        assert_eq!(scheme10.interpolate(0, 2), Colour::new(0, 0, 0));
        assert_eq!(scheme10.interpolate(1, 2), Colour::new(255, 255, 255));
        assert_eq!(scheme10.interpolate(0, 3), Colour::new(0, 0, 0));
        assert_eq!(scheme10.interpolate(1, 3), Colour::new(255, 255, 255));
        assert_eq!(scheme10.interpolate(2, 3), Colour::new(255, 255, 255));
        assert_eq!(scheme10.interpolate(2, 5), Colour::new(255, 255, 255));
        assert_eq!(scheme10.interpolate(2, 8), Colour::new(255, 255, 255));
    }

    #[test]
    fn test_colour_interpolation() {
        let black = Colour::new(0, 0, 0);
        let white = Colour::new(255, 255, 255);
        assert_eq!(black.interpolate(white, 1.0), white);
        assert_eq!(black.interpolate(white, 0.0), black);
        assert_eq!(white.interpolate(black, 1.0), black);
        assert_eq!(white.interpolate(black, 0.0), white);
        assert_eq!(black.interpolate(white, 0.2), Colour::new(51, 51, 51));
        assert_eq!(white.interpolate(black, 0.2), Colour::new(204, 204, 204));
        let other = Colour::new(34, 202, 117);
        assert_eq!(black.interpolate(other, 0.4), Colour::new(14, 81, 47));
        assert_eq!(white.interpolate(other, 0.2), Colour::new(211, 244, 227));
        assert_eq!(white.interpolate(other, 0.8), Colour::new(78, 213, 145));
    }
}
