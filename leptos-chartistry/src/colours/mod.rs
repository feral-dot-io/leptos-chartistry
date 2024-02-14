mod colourmaps;
mod scheme;

pub use colourmaps::*;
pub use scheme::{ColourScheme, LinearGradient};

use leptos::*;
use std::str::FromStr;

/// A colour in RGB format.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

impl Colour {
    /// Create a new colour with the given red, green, and blue values.
    #[deprecated(since = "0.1.1", note = "renamed to `from_rgb`")]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self::from_rgb(red, green, blue)
    }

    /// Create a new colour with the given red, green, and blue values.
    pub const fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    fn interpolate(self, rhs: Self, ratio: f64) -> Self {
        let ratio = ratio.clamp(0.0, 1.0);
        let interpolate = |pre: u8, post: u8| {
            let pre = pre as f64;
            let post = post as f64;
            //let max = pre.max(post);
            //let min = pre.min(post);
            let diff = post - pre;
            println!(
                "pre={pre:?} post={post:?} diff={diff:?} ratio={ratio:?} => {:?}",
                (pre + (diff * ratio)).round()
            );
            (pre + (diff * ratio)).round() as u8
        };
        Colour {
            red: interpolate(self.red, rhs.red),
            green: interpolate(self.green, rhs.green),
            blue: interpolate(self.blue, rhs.blue),
        }
    }
}

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

impl FromStr for Colour {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('#');
        let len = s.len();
        if len != 6 {
            return Err(format!("expected 6 characters, got {}", len));
        }
        let red = u8::from_str_radix(&s[0..2], 16).map_err(|e| e.to_string())?;
        let green = u8::from_str_radix(&s[2..4], 16).map_err(|e| e.to_string())?;
        let blue = u8::from_str_radix(&s[4..6], 16).map_err(|e| e.to_string())?;
        Ok(Colour { red, green, blue })
    }
}

impl IntoAttribute for Colour {
    fn into_attribute(self) -> Attribute {
        self.to_string().into_attribute()
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        self.to_string().into_attribute()
    }
}
