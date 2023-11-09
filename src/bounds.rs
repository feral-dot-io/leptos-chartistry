/// Describes a bounding area in 2D space. Asserts that the bounds are not negative, known as a "negative bounds".
///
/// Warning: when calculating new dimentions try to use [`Bounds::shrink`] instead of [`Bounds::from_points`] as it better avoids a panic on negative bounds.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Bounds {
    top: f64,
    right: f64,
    bottom: f64,
    left: f64,
}

impl Bounds {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            left: 0.0,
            top: 0.0,
            right: width,
            bottom: height,
        }
    }

    /// Creates a new bounds from the given top-left (x1, y1) and bottom-right (x2, y2) points. Note that this differs from SVG which tends to use (x, y, width, height) and CSS which tends to use (top, right, bottom, left). The CSS pattern is preferred elsewhere.
    ///
    /// Panics on negative bounds (i.e., x1 > x2 or y1 > y2). Use [`Bounds::shrink`] where possible for creating layouts.
    pub fn from_points(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        assert!(x1 <= x2);
        assert!(y1 <= y2);
        Self {
            left: x1,
            top: y1,
            right: x2,
            bottom: y2,
        }
    }

    /// Creates an inner bounds from an outer bounds. Shrinks each edge simultenously by the size given i.e., produces a rectangle inside another rectangular bounds.
    ///
    /// If size is too large (i.e., would extend beyond the opposite edge to create a negative bounds), then the size is clamped to the maximum to produce a zero area bounds. Preferences left and bottom edges e.g., if left + right > width, then left will be as large as possible.
    ///
    /// May produce a zero bounds. If size is negative, clamps to zero.
    pub fn shrink(self, top: f64, right: f64, bottom: f64, left: f64) -> Self {
        let top = top.max(0.0);
        let right = right.max(0.0);
        let bottom = bottom.max(0.0);
        let left = left.max(0.0);
        let left = f64::min(self.left + left, self.right);
        let bottom = f64::max(self.bottom - bottom, self.top);
        let right = f64::max(self.right - right, left);
        let top = f64::min(self.top + top, bottom);
        Self::from_points(left, top, right, bottom)
    }

    /// Returns true if the bounds are empty i.e., zero area. Must be absolute zero, does not test for a small float delta.
    pub fn is_zero(&self) -> bool {
        self.left == self.right || self.top == self.bottom
    }

    pub fn left_x(&self) -> f64 {
        self.left
    }

    pub fn right_x(&self) -> f64 {
        self.right
    }

    pub fn top_y(&self) -> f64 {
        self.top
    }

    pub fn bottom_y(&self) -> f64 {
        self.bottom
    }

    pub fn centre_x(&self) -> f64 {
        self.left_x() + (self.width() / 2.0)
    }

    pub fn centre_y(&self) -> f64 {
        self.top_y() + (self.height() / 2.0)
    }

    pub fn width(&self) -> f64 {
        self.right - self.left
    }

    pub fn height(&self) -> f64 {
        self.bottom - self.top
    }

    /// Tests if the given point is within the bounds.
    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.left && x <= self.right && y >= self.top && y <= self.bottom
    }
}

impl From<web_sys::DomRectReadOnly> for Bounds {
    fn from(r: web_sys::DomRectReadOnly) -> Self {
        Self::from_points(r.left(), r.top(), r.right(), r.bottom())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounds() {
        let b = Bounds::from_points(1.1, 2.2, 3.3, 4.4);
        assert_eq!(b.left_x(), 1.1);
        assert_eq!(b.right_x(), 3.3);
        assert_eq!(b.top_y(), 2.2);
        assert_eq!(b.bottom_y(), 4.4);
        assert_eq!(b.width(), (3.3 - 1.1));
        assert_eq!(b.height(), (4.4 - 2.2));
        assert_eq!(b.centre_x(), (3.3 - 1.1) / 2.0 + 1.1);
        assert_eq!(b.centre_y(), (4.4 - 2.2) / 2.0 + 2.2);
        assert_eq!(b.is_zero(), false);
        assert_eq!(Bounds::from_points(0.0, 10.0, 1.0, 10.0).is_zero(), true);
        assert_eq!(b.contains(1.1, 2.2), true);
        assert_eq!(b.contains(3.3, 4.4), true);
        assert_eq!(b.contains(5.5, 6.6), false);
    }

    #[test]
    fn test_shrink() {
        let b = Bounds::new(100.0, 200.0);
        // Shrinks edges
        assert_eq!(
            b.shrink(10.0, 20.0, 30.0, 40.0),
            Bounds::from_points(40.0, 10.0, 80.0, 170.0)
        );
        // Zero size does nothing
        assert_eq!(b.shrink(0.0, 0.0, 0.0, 0.0), b);
        // Clamp if top is too large
        assert_eq!(
            b.shrink(1000.0, 0.0, 0.0, 0.0),
            Bounds::from_points(0.0, 200.0, 100.0, 200.0)
        );
        // Clamp if right is too large
        assert_eq!(
            b.shrink(0.0, 1000.0, 0.0, 0.0),
            Bounds::from_points(0.0, 0.0, 0.0, 200.0)
        );
        // Clamp if bottom is too large
        assert_eq!(
            b.shrink(0.0, 0.0, 1000.0, 0.0),
            Bounds::from_points(0.0, 0.0, 100.0, 0.0)
        );
        // Clamp if left is too large
        assert_eq!(
            b.shrink(0.0, 0.0, 0.0, 1000.0),
            Bounds::from_points(100.0, 0.0, 100.0, 200.0)
        );
        // If clamping, preference left and bottom
        assert_eq!(
            b.shrink(1000.0, 1000.0, 1000.0, 1000.0),
            Bounds::from_points(100.0, 0.0, 100.0, 0.0)
        );
    }
}
