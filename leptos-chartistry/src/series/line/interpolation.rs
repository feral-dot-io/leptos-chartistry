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
                    let mid_x = x / 2.0;
                    let mid_y = y / 2.0;
                    match self {
                        Self::Horizontal => format!("H {} V {} ", x, y),
                        Self::HorizontalMiddle => format!("H {} V {} H {} ", mid_x, y, mid_x),
                        Self::Vertical => format!("V {} H {} ", y, x),
                        Self::VerticalMiddle => format!("V {} H {} V {} ", mid_y, x, mid_y),
                    }
                }
            })
            .collect::<String>()
    }
}
