use crate::bounds::Bounds;
use std::str::FromStr;

/// Identifies a rectangle edge.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Edge {
    /// The top edge.
    Top,
    /// The right edge.
    Right,
    /// The bottom edge.
    Bottom,
    /// The left edge.
    Left,
}

impl Edge {
    /// Returns true if the edge is horizontal (top and bottom).
    pub fn is_horizontal(&self) -> bool {
        match self {
            Self::Top | Self::Bottom => true,
            Self::Right | Self::Left => false,
        }
    }

    /// Returns true if the edge is vertical (left and right).
    pub fn is_vertical(&self) -> bool {
        !self.is_horizontal()
    }
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Top => write!(f, "top"),
            Self::Right => write!(f, "right"),
            Self::Bottom => write!(f, "bottom"),
            Self::Left => write!(f, "left"),
        }
    }
}

impl FromStr for Edge {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "top" => Ok(Self::Top),
            "right" => Ok(Self::Right),
            "bottom" => Ok(Self::Bottom),
            "left" => Ok(Self::Left),
            _ => Err(format!("unknown edge: `{}`", s)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EdgeBoundsIter<I> {
    iter: I,

    top: Bounds,
    right: Bounds,
    bottom: Bounds,
    left: Bounds,

    top_size: f64,
    right_size: f64,
    bottom_size: f64,
    left_size: f64,
}

pub trait IntoEdgeBounds<I> {
    fn into_edge_bounds(self, outer: Bounds, inner: Bounds) -> EdgeBoundsIter<I>;
}

impl<I: IntoIterator<Item = (C, Edge, f64)>, C> IntoEdgeBounds<I> for I {
    fn into_edge_bounds(self, outer: Bounds, inner: Bounds) -> EdgeBoundsIter<I> {
        EdgeBoundsIter::new(self, outer, inner)
    }
}

impl<I> EdgeBoundsIter<I> {
    pub fn new(iter: I, outer: Bounds, inner: Bounds) -> Self {
        Self {
            iter,

            top: Bounds::from_points(
                inner.left_x(),
                outer.top_y(),
                inner.right_x(),
                inner.top_y(),
            ),
            right: Bounds::from_points(
                inner.right_x(),
                inner.top_y(),
                outer.right_x(),
                inner.bottom_y(),
            ),
            bottom: Bounds::from_points(
                inner.left_x(),
                inner.bottom_y(),
                inner.right_x(),
                outer.bottom_y(),
            ),
            left: Bounds::from_points(
                outer.left_x(),
                inner.top_y(),
                inner.left_x(),
                inner.bottom_y(),
            ),

            top_size: 0.0,
            right_size: 0.0,
            bottom_size: 0.0,
            left_size: 0.0,
        }
    }
}

impl<I, C> Iterator for EdgeBoundsIter<I>
where
    I: Iterator<Item = (C, Edge, f64)>,
{
    type Item = (C, Edge, Bounds);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(c, edge, size)| {
            let bounds = match edge {
                Edge::Top => self.top.shrink(
                    self.top_size,
                    0.0,
                    self.top.height() - self.top_size - size,
                    0.0,
                ),
                Edge::Right => self.right.shrink(
                    0.0,
                    self.right_size,
                    0.0,
                    self.right.width() - self.right_size - size,
                ),
                Edge::Bottom => self.bottom.shrink(
                    self.bottom.height() - self.bottom_size - size,
                    0.0,
                    self.bottom_size,
                    0.0,
                ),
                Edge::Left => self.left.shrink(
                    0.0,
                    self.left.width() - self.left_size - size,
                    0.0,
                    self.left_size,
                ),
            };
            // Update running totals
            match edge {
                Edge::Top => self.top_size += size,
                Edge::Right => self.right_size += size,
                Edge::Bottom => self.bottom_size += size,
                Edge::Left => self.left_size += size,
            };
            (c, edge, bounds)
        })
    }
}
