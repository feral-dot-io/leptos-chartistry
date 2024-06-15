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
