use crate::bounds::Bounds;

/// A projection converts between data and SVG coordinates. SVG has zero in the top left corner. Data coordinates have zero in the bottom left.
#[derive(Clone, Debug, PartialEq)]
pub struct Projection {
    // SVG bounds
    bounds: Bounds,
    // Data offset
    left_x: f64,
    bottom_y: f64,

    x_mult: f64,
    y_mult: f64,
}

impl Projection {
    pub fn new(bounds: Bounds, range_x: Option<(f64, f64)>, range_y: Option<(f64, f64)>) -> Self {
        let (left_x, right_x) = range_x.unwrap_or_default();
        let (bottom_y, top_y) = range_y.unwrap_or_default();
        // If the range is zero, skip projection
        let width = right_x - left_x;
        let x_mult = bounds.width() / if width == 0.0 { 0.5 } else { width };
        let height = top_y - bottom_y;
        let y_mult = bounds.height() / if height == 0.0 { 0.5 } else { height };
        Projection {
            bounds,
            left_x,
            bottom_y,
            x_mult,
            y_mult,
        }
    }

    /// Converts a data point to SVG view coordinates. View coordinates are in SVG space with zero at top left. Data coordinates are in chart space with zero at bottom left.
    pub fn position_to_svg(&self, x: f64, y: f64) -> (f64, f64) {
        let x = self.bounds.left_x() + (x - self.left_x) * self.x_mult;
        let y = self.bounds.bottom_y() - (y - self.bottom_y) * self.y_mult;
        (x, y)
    }

    /// Converts an SVG point to data coordinates. View coordinates are in SVG space with zero at top left. Data coordinates are in chart space with zero at bottom left.
    pub fn svg_to_position(&self, x: f64, y: f64) -> (f64, f64) {
        let x = self.left_x + (x - self.bounds.left_x()) / self.x_mult;
        let y = self.bottom_y - (y - self.bounds.bottom_y()) / self.y_mult;
        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_coords(p: &Projection, pos: (f64, f64), svg: (f64, f64)) {
        assert_eq!(p.position_to_svg(pos.0, pos.1), (svg.0, svg.1), "to svg");
        assert_eq!(p.svg_to_position(svg.0, svg.1), (pos.0, pos.1), "to pos");
    }

    #[test]
    fn test_projection() {
        let bounds = Bounds::from_points(10.0, 10.0, 90.0, 90.0);
        let p = Projection::new(bounds, Some((0.0, 100.0)), Some((0.0, 100.0)));

        // Data range -> view bounds
        assert_coords(&p, (0.0, 0.0), (10.0, 90.0)); // Bottom left
        assert_coords(&p, (100.0, 0.0), (90.0, 90.0)); // Bottom right
        assert_coords(&p, (0.0, 100.0), (10.0, 10.0)); // Top left
        assert_coords(&p, (100.0, 100.0), (90.0, 10.0)); // Top right
        assert_coords(&p, (50.0, 50.0), (50.0, 50.0)); // Centre
    }

    #[test]
    fn test_incl_zero() {
        let bounds = Bounds::from_points(10.0, 10.0, 90.0, 90.0);
        let p = Projection::new(bounds, Some((0.0, 200.0)), Some((0.0, 200.0)));
        // Data range (0, 0) to (200, 200) -> view bounds
        assert_coords(&p, (0.0, 0.0), (10.0, 90.0)); // Bottom left
        assert_coords(&p, (200.0, 0.0), (90.0, 90.0)); // Bottom right
        assert_coords(&p, (0.0, 200.0), (10.0, 10.0)); // Top left
        assert_coords(&p, (200.0, 200.0), (90.0, 10.0)); // Top right
        assert_coords(&p, (100.0, 100.0), (50.0, 50.0)); // Centre
    }

    #[test]
    fn test_projection_zero_range() {
        let bounds = Bounds::from_points(10.0, 10.0, 90.0, 90.0);
        Projection::new(bounds, None, None);
    }

    #[test]
    fn test_partial_eq() {
        let bounds = Bounds::from_points(10.0, 10.0, 90.0, 90.0);
        let p = Projection::new(
            bounds,
            Some((bounds.left_x(), bounds.right_x())),
            Some((bounds.bottom_y(), bounds.top_y())),
        );
        assert_eq!(p, p.clone());
    }
}
