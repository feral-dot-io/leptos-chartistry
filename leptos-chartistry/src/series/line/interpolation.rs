/// Line interpolation. This is used to determine how to draw the line between points.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum Interpolation {
    /// Linear interpolation draws a straight line between points. The simplest of methods.
    #[default]
    Linear,
    /// Step interpolation only uses horizontal and vertical lines to connect two points.
    Step(Step),
    //Monotone,
}

/// Step interpolation only uses horizontal and vertical lines to connect two points. We have a choice of where to put the "corner" of the step.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum Step {
    /// Moves across the horizontal plane first then the vertical.
    #[default]
    Horizontal,
    /// Moves midway across the horizontal plane first, then all of the vertical, then the rest of the horizontal. When chained with other steps, creates a single step but could make the data point less obvious.
    HorizontalMiddle,
    /// Moves across the vertical plane first then the horizontal.
    Vertical,
    /// Similar to [Step::HorizontalMiddle] but moves midway across the vertical plane first, then all of the horizontal, then the rest of the vertical.
    VerticalMiddle,
}

impl std::str::FromStr for Interpolation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "linear" => Ok(Self::Linear),
            "step-horizontal" => Ok(Self::Step(Step::Horizontal)),
            "step-horizontal-middle" => Ok(Self::Step(Step::HorizontalMiddle)),
            "step-vertical" => Ok(Self::Step(Step::Vertical)),
            "step-vertical-middle" => Ok(Self::Step(Step::VerticalMiddle)),
            _ => Err(format!("unknown line interpolation: `{}`", s)),
        }
    }
}

impl std::fmt::Display for Interpolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Linear => write!(f, "linear"),
            Self::Step(Step::Horizontal) => write!(f, "step-horizontal"),
            Self::Step(Step::HorizontalMiddle) => write!(f, "step-horizontal-middle"),
            Self::Step(Step::Vertical) => write!(f, "step-vertical"),
            Self::Step(Step::VerticalMiddle) => write!(f, "step-vertical-middle"),
        }
    }
}

impl Interpolation {
    pub(super) fn path(self, points: &[(f64, f64)]) -> String {
        match self {
            Self::Linear => linear(points),
            Self::Step(step) => step.path(points),
            //Self::Monotone => monotone(points),
        }
    }
}

fn linear(points: &[(f64, f64)]) -> String {
    let mut need_move = true;
    points
        .iter()
        .map(|(x, y)| {
            if x.is_nan() || y.is_nan() {
                need_move = true;
                "".to_string()
            } else if need_move {
                need_move = false;
                format!("M {} {} ", x, y)
            } else {
                format!("L {} {} ", x, y)
            }
        })
        .collect::<String>()
}

impl Step {
    fn path(self, points: &[(f64, f64)]) -> String {
        let mut prev: Option<(f64, f64)> = None;
        points
            .iter()
            .map(|&(x, y)| {
                if x.is_nan() || y.is_nan() {
                    prev = None;
                    "".to_string()
                } else if let Some((prev_x, prev_y)) = prev {
                    prev = Some((x, y));
                    match self {
                        Self::Horizontal => format!("H {} V {} ", x, y),
                        Self::HorizontalMiddle => {
                            format!("H {} V {} H {} ", (x + prev_x) / 2.0, y, x)
                        }
                        Self::Vertical => format!("V {} H {} ", y, x),
                        Self::VerticalMiddle => {
                            format!("V {} H {} V {} ", (y + prev_y) / 2.0, x, y)
                        }
                    }
                } else {
                    prev = Some((x, y));
                    format!("M {} {} ", x, y)
                }
            })
            .collect::<String>()
    }
}
