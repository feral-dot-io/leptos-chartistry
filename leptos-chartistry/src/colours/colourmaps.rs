// The Scientific colour maps are licensed under a MIT License
//
// Copyright (c) 2023, Fabio Crameri
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// The Scientific colour maps by Fabio Crameri (Crameri 2018) prevent visual distortion of the data and exclusion of readers with colour­-vision deficiencies (Crameri et al., 2020) and the overview graphic is available via the open-access s-ink.org repository.

/*
Notes:

Colours are an important part of charts. Our aim is to avoid less readable and misleading colour schemes. So we rely on the scientific colour maps developed by Fabio Crameri. These are perceptually uniform, colour blind friendly, and monochrome friendly.

Reading material:
- Summary poster: https://www.fabiocrameri.ch/ws/media-library/a17d02961b3a4544961416de2d7900a4/posterscientificcolourmaps_crameri.pdf
- Article "The misuse of colour in science communication" https://www.nature.com/articles/s41467-020-19160-7
- Homepage https://www.fabiocrameri.ch/colourmaps/
- Flow chart on picking a scheme: https://s-ink.org/colour-map-guideline
- Available colour schemes: https://s-ink.org/scientific-colour-maps
*/

use super::Colour;

pub const LAJOLLA: [Colour; 10] = [
    Colour::from_rgb(0x19, 0x19, 0x00),
    Colour::from_rgb(0x33, 0x22, 0x0F),
    Colour::from_rgb(0x5B, 0x30, 0x23),
    Colour::from_rgb(0x8F, 0x40, 0x3D),
    Colour::from_rgb(0xC7, 0x50, 0x4B),
    Colour::from_rgb(0xE0, 0x72, 0x4F),
    Colour::from_rgb(0xE7, 0x94, 0x52),
    Colour::from_rgb(0xEE, 0xB5, 0x55),
    Colour::from_rgb(0xF8, 0xDF, 0x7C),
    Colour::from_rgb(0xFF, 0xFE, 0xCB),
];

pub const LIPARI: [Colour; 10] = [
    Colour::from_rgb(0x03, 0x13, 0x26),
    Colour::from_rgb(0x13, 0x38, 0x5A),
    Colour::from_rgb(0x47, 0x58, 0x7A),
    Colour::from_rgb(0x6B, 0x5F, 0x76),
    Colour::from_rgb(0x8E, 0x61, 0x6C),
    Colour::from_rgb(0xBC, 0x64, 0x61),
    Colour::from_rgb(0xE5, 0x7B, 0x62),
    Colour::from_rgb(0xE7, 0xA2, 0x79),
    Colour::from_rgb(0xE9, 0xC9, 0x9F),
    Colour::from_rgb(0xFD, 0xF5, 0xDA),
];
