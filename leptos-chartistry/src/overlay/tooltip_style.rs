use std::str::FromStr;

use crate::Colour;

const DEFAULT_STYLE: &str = "position: absolute; z-index: 1; width: max-content; height: max-content; transform: translateY(-50%); border: 1px solid lightgrey; white-space: pre; font-family: monospace;";

/// Styling properties of the tooltip.
#[derive(Clone, Debug, PartialEq)]
pub struct TooltipStyle {
    /// CSS styling class.
    pub class: String,
    /// Background colour.
    pub background: Option<Colour>,
    /// Text colour.
    pub text_colour: Option<Colour>,
}

impl Default for TooltipStyle {
    /// Get the default style with white background and default text colour.
    fn default() -> Self {
        Self {
            class: String::new(),
            background: Some(Colour::new(255, 255, 255)),
            text_colour: None,
        }
    }
}

impl TooltipStyle {
    /// Create a new TooltipStyle.
    pub fn new() -> Self {
        Self {
            background: None,
            ..Default::default()
        }
    }

    /// Sets the tooltip class.
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = class.into();
        self
    }

    /// Sets the background colour.
    pub fn background(mut self, background: impl Into<String>) -> Self {
        self.background = Colour::from_str(&background.into()).ok();
        self
    }

    /// Sets the text colour.
    pub fn text_colour(mut self, text_colour: impl Into<String>) -> Self {
        self.text_colour = Colour::from_str(&text_colour.into()).ok();
        self
    }

    /// Get the CSS style for the tooltip.
    pub fn css_style(&self) -> String {
        let text = self
            .text_colour
            .map_or(String::new(), |text| format!(" color: {text};"));
        let background = self
            .background
            .map_or(String::new(), |bg| format!(" background: {bg};"));

        format!("{};{}{}", DEFAULT_STYLE, text, background)
    }
}
