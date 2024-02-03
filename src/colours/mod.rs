mod colourmaps;

pub use colourmaps::*;

/*
Colours are an important part of charts. Our aim is to avoid less readable and misleading colour schemes. So we rely on the scientific colour maps developed by Fabio Crameri. These are perceptually uniform, colour blind friendly, and monochrome friendly.

Reading material:
- Summary poster: https://www.fabiocrameri.ch/ws/media-library/a17d02961b3a4544961416de2d7900a4/posterscientificcolourmaps_crameri.pdf
- Article "The misuse of colour in science communication" https://www.nature.com/articles/s41467-020-19160-7
- Homepage https://www.fabiocrameri.ch/colourmaps/
- Flow chart on picking a scheme: https://s-ink.org/colour-map-guideline
- Available colour schemes: https://s-ink.org/scientific-colour-maps
*/

/// Arbitrary colours for a brighter palette
pub const ARBITRARY: [Colour; 10] = [
    Colour::new(0x12, 0xA5, 0xED), // Blue
    Colour::new(0xF5, 0x32, 0x5B), // Red
    Colour::new(0x71, 0xc6, 0x14), // Green
    Colour::new(0xFF, 0x84, 0x00), // Orange
    Colour::new(0x7b, 0x4d, 0xff), // Purple
    Colour::new(0xdb, 0x4c, 0xb2), // Magenta
    Colour::new(0x92, 0xb4, 0x2c), // Darker green
    Colour::new(0xFF, 0xCA, 0x00), // Yellow
    Colour::new(0x22, 0xd2, 0xba), // Turquoise
    Colour::new(0xea, 0x60, 0xdf), // Pink
];

#[derive(Clone, Debug, PartialEq)]
pub struct ColourScheme {
    // Must have at least one colour
    swatches: Vec<Colour>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

impl ColourScheme {
    pub(crate) fn new(first: Colour, rest: &[Colour]) -> Self {
        Self {
            swatches: std::iter::once(first).chain(rest.iter().copied()).collect(),
        }
    }

    pub fn by_index(&self, index: usize) -> Colour {
        // Note: not using checked_rem_euclid as we're guaranteed to have at least one colour
        let index = index.rem_euclid(self.swatches.len());
        self.swatches[index]
    }
}

impl Colour {
    pub(crate) const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl From<[Colour; 3]> for ColourScheme {
    fn from(colours: [Colour; 3]) -> Self {
        Self::new(colours[0], &colours[1..])
    }
}

impl From<[Colour; 10]> for ColourScheme {
    fn from(colours: [Colour; 10]) -> Self {
        Self::new(colours[0], &colours[1..])
    }
}

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}
