use super::bounds::Bounds;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Padding {
    pub(crate) top: f64,
    pub(crate) right: f64,
    pub(crate) bottom: f64,
    pub(crate) left: f64,
}

/// Represents padding around a component. Note that the context matters on how it's applied. For example, padding applied to the whole chart will shrink the available space whereas padding applied to a label will increase the size used.
impl Padding {
    /// Creates a new zero / empty / none padding.
    pub fn zero() -> Self {
        Self::sides(0.0, 0.0, 0.0, 0.0)
    }

    /// Creates a new padding with the given top, right, bottom, and left values. This is CSS style: clockwise from the top.
    pub fn sides(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Creates a new padding with the given horizontal (top and bottom) and vertical (left and right) values.
    pub fn hv(h: f64, v: f64) -> Self {
        Self::sides(h, v, h, v)
    }

    /// Returns the total height of the padding.
    pub(crate) fn height(&self) -> f64 {
        self.top + self.bottom
    }

    /// Returns the total width of the padding.
    pub(crate) fn width(&self) -> f64 {
        self.left + self.right
    }

    /// Applies the padding to the given bounds. Shrinks the bounds by the padding.
    pub(crate) fn apply(self, outer: Bounds) -> Bounds {
        outer.shrink(self.top, self.right, self.bottom, self.left)
    }

    /// Converts the padding to a CSS style string.
    pub(crate) fn to_style_px(self) -> String {
        format!(
            "{}px {}px {}px {}px",
            self.top, self.right, self.bottom, self.left
        )
    }
}

impl From<f64> for Padding {
    fn from(v: f64) -> Self {
        Padding::sides(v, v, v, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padding() {
        let p = Padding::sides(1.1, 2.2, 3.3, 4.4);
        assert_eq!(p.top, 1.1);
        assert_eq!(p.right, 2.2);
        assert_eq!(p.bottom, 3.3);
        assert_eq!(p.left, 4.4);
        assert_eq!(p.height(), 1.1 + 3.3);
        assert_eq!(p.width(), 2.2 + 4.4);
        assert_eq!(
            p.apply(Bounds::new(100.0, 200.0)),
            Bounds::from_points(4.4, 1.1, 97.8, 196.7)
        );
        assert_eq!(p.to_style_px(), "1.1px 2.2px 3.3px 4.4px");
        assert_eq!(Padding::zero().to_style_px(), "0px 0px 0px 0px");
        assert_eq!(
            Padding::hv(1.1, 2.2).to_style_px(),
            "1.1px 2.2px 1.1px 2.2px"
        );
        assert_eq!(Padding::from(1.1).to_style_px(), "1.1px 1.1px 1.1px 1.1px");
    }
}
