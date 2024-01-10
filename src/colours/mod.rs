mod colourmaps;

pub use colourmaps::*;

/*
Colours are an important part of charts. Our aim is to avoid less readable and misleading colour schemes. So we rely on the scientific colour maps developed by Fabio Crameri. These are perceptually uniform, colour blind friendly, and monochrome friendly.

Reading material:
- Summary poster: https://www.fabiocrameri.ch/ws/media-library/a17d02961b3a4544961416de2d7900a4/posterscientificcolourmaps_crameri.pdf
- Article "The misuse of colour in science communication" https://www.nature.com/articles/s41467-020-19160-7
- Homepage https://www.fabiocrameri.ch/colourmaps/
- Picking a colour scheme: https://s-ink.org/colour-map-guideline
*/

#[derive(Clone, Debug, PartialEq)]
pub struct ColourScheme {
    colours: Vec<Colour>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

impl ColourScheme {
    pub fn by_index(&self, index: usize) -> Colour {
        let index = index.rem_euclid(self.colours.len());
        self.colours[index]
    }
}

impl Colour {
    const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    const fn new_tuple((red, green, blue): (u8, u8, u8)) -> Self {
        Self::new(red, green, blue)
    }
}

impl Default for ColourScheme {
    fn default() -> Self {
        ARBITRARY.as_ref().into()
    }
}

impl From<&[(u8, u8, u8)]> for ColourScheme {
    fn from(colours: &[(u8, u8, u8)]) -> Self {
        let colours = colours
            .iter()
            .map(|&(red, green, blue)| Colour::new(red, green, blue))
            .collect();
        Self { colours }
    }
}

impl From<[(u8, u8, u8); 10]> for ColourScheme {
    fn from(colours: [(u8, u8, u8); 10]) -> Self {
        colours.as_ref().into()
    }
}

impl From<&[Colour]> for ColourScheme {
    fn from(colours: &[Colour]) -> Self {
        Self {
            colours: colours.to_vec(),
        }
    }
}

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

pub static LIGHTISH_GREY: Colour = Colour::new(0xD2, 0xD2, 0xD2);
pub static LIGHTER_GREY: Colour = Colour::new(0xEF, 0xF2, 0xFA);
pub static LIGHT_GREY: Colour = Colour::new_tuple(colourmaps::GRAYC[6]);

/// Arbitrary colours for a brighter palette
pub const ARBITRARY: [(u8, u8, u8); 10] = [
    (0x12, 0xA5, 0xED), // Blue
    (0xF5, 0x32, 0x5B), // Red
    (0x71, 0xc6, 0x14), // Green
    (0xFF, 0x84, 0x00), // Orange
    (0x7b, 0x4d, 0xff), // Purple
    (0xdb, 0x4c, 0xb2), // Magenta
    (0x92, 0xb4, 0x2c), // Darker green
    (0xFF, 0xCA, 0x00), // Yellow
    (0x22, 0xd2, 0xba), // Turquoise
    (0xea, 0x60, 0xdf), // Pink
];
