use crate::bounds::Bounds;
use leptos::*;

/// A projection converts between data and SVG coordinates. SVG has zero in the top left corner. Data coordinates have zero in the bottom left.
#[derive(Clone, Debug, PartialEq)]
pub struct Projection {
    bounds: Bounds,
    range: Bounds,
    x_mult: f64,
    y_mult: f64,
}

impl Projection {
    pub fn new(bounds: Bounds, range: Bounds) -> Self {
        log::info!("range: {:?}: {} x {}", range, range.width(), range.height());
        Projection {
            bounds,
            range,
            x_mult: bounds.width() / range.width(),
            y_mult: bounds.height() / range.height(),
        }
    }

    /// Converts a data point to SVG view coordinates. View coordinates are in SVG space with zero at top left. Data coordinates are in chart space with zero at bottom left.
    pub fn data_to_svg(&self, x: f64, y: f64) -> (f64, f64) {
        let x = self.bounds.left_x() + (x - self.range.left_x()) * self.x_mult;
        let y = self.bounds.bottom_y() - (y - self.range.top_y()) * self.y_mult;
        (x, y)
    }

    /// Converts an SVG point to data coordinates. View coordinates are in SVG space with zero at top left. Data coordinates are in chart space with zero at bottom left.
    pub fn svg_to_data(&self, x: f64, y: f64) -> (f64, f64) {
        let x = self.range.left_x() + (x - self.bounds.left_x()) / self.x_mult;
        let y = self.range.top_y() - (y - self.bounds.bottom_y()) / self.y_mult;
        (x, y)
    }

    pub fn bounds(&self) -> Bounds {
        self.bounds
    }

    pub fn derive_width(proj: Signal<Projection>) -> Signal<f64> {
        Signal::derive(move || with!(|proj| proj.bounds().width()))
    }
    pub fn derive_height(proj: Signal<Projection>) -> Signal<f64> {
        Signal::derive(move || with!(|proj| proj.bounds().height()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_coords(p: &Projection, (data_x, data_y): (f64, f64), (svg_x, svg_y): (f64, f64)) {
        assert_eq!(p.data_to_svg(data_x, data_y), (svg_x, svg_y));
        assert_eq!(p.svg_to_data(svg_x, svg_y), (data_x, data_y));
    }

    #[test]
    fn test_projection() {
        let range = Bounds::from_points(0.0, 0.0, 100.0, 100.0);
        let bounds = Bounds::from_points(10.0, 10.0, 90.0, 90.0);
        let p = Projection::new(bounds, range);

        assert_eq!(p.bounds(), bounds);

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
        let range = Bounds::from_points(0.0, 0.0, 200.0, 200.0);
        let p = Projection::new(bounds, range);
        assert_eq!(p.bounds(), bounds);

        // Data range (0, 0) to (200, 200) -> view bounds
        assert_coords(&p, (0.0, 0.0), (10.0, 90.0)); // Bottom left
        assert_coords(&p, (200.0, 0.0), (90.0, 90.0)); // Bottom right
        assert_coords(&p, (0.0, 200.0), (10.0, 10.0)); // Top left
        assert_coords(&p, (200.0, 200.0), (90.0, 10.0)); // Top right
        assert_coords(&p, (100.0, 100.0), (50.0, 50.0)); // Centre
    }

    #[test]
    fn test_partial_eq() {
        let bounds = Bounds::from_points(10.0, 10.0, 90.0, 90.0);
        let p = Projection::new(bounds, bounds);
        assert_eq!(p, p.clone());
    }
}
