/// Line interpolation. This is used to determine how to draw the line between points.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum Interpolation {
    /// Linear interpolation draws a straight line between points. The simplest of methods.
    Linear,
    /// Step interpolation only uses horizontal and vertical lines to connect two points.
    Step(Step),
    /// Cubic monotone interpolation smooths the line between points. Avoids spurious oscillations.[^Steffen]
    ///
    /// [^Steffen]: Steffen, M., “A simple method for monotonic interpolation in one dimension.”, Astronomy and Astrophysics, vol. 239, pp. 443–450, 1990.
    #[default]
    Monotone,
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
            "monotone" => Ok(Self::Monotone),
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
            Self::Monotone => write!(f, "monotone"),
        }
    }
}

impl From<Step> for Interpolation {
    fn from(step: Step) -> Self {
        Self::Step(step)
    }
}

impl Interpolation {
    pub(super) fn path(self, points: &[(f64, f64)]) -> String {
        match self {
            Self::Linear => linear(points),
            Self::Step(step) => step.path(points),
            Self::Monotone => monotone(points),
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

/*
    Implementation from "A simple method for monotonic interpolation in one dimension". [^Steffen]

    In Fortran:
        y1(i) = (sign(1.0, s[i-1]) + sign(1.0, s[i])) * min(abs(s[i-1]), 0.5 * abs(p[i]))
    Where:
        s[i] = (y[i+1] - y[i]) / (x[i+1] - x[i])
        p[i] = (s[i-1]h[i] + s[i]h[i-1]) / (h[i-1] + h[i])
        h[i] = x[i+1] - x[i]

    In Rust:
        y(i) = (s[i-1].signum() + s[i].signum()) * s[i-1].abs().min(0.5 * p[i].abs())
*/
fn monotone(points: &[(f64, f64)]) -> String {
    let mut path = String::with_capacity(points.len());
    for i in 0..points.len() {
        let (x_prev, y_prev) = get_or_nan(points, i.checked_sub(1));
        let (x, y) = points[i];
        let (x_next, y_next) = get_or_nan(points, i.checked_add(1));
        // Path command
        let cmd = if x.is_nan() || y.is_nan() {
            // Inbetween segments
            "".to_string()
        } else if x_prev.is_nan() || y_prev.is_nan() {
            // Start of a new segment
            format!("M {x},{y} ")
        } else if x_next.is_nan() || y_next.is_nan() {
            // End of a segment
            format!("L {x},{y} ")
        } else {
            let tangent = tangent(x_prev, x, x_next, y_prev, y, y_next);
            let dx = (x - x_prev) / 3.0;
            let x_c = x - dx;
            let y_c = y - dx * tangent;
            format!("S {x_c},{y_c} {x},{y} ")
        };
        path.push_str(&cmd);
    }
    path
}

fn get_or_nan(points: &[(f64, f64)], i: Option<usize>) -> (f64, f64) {
    i.and_then(|i| points.get(i).copied())
        .unwrap_or((f64::NAN, f64::NAN))
}

fn slope(x: f64, y: f64, x_next: f64, y_next: f64) -> f64 {
    (y_next - y) / (x_next - x)
}

fn tangent(x_prev: f64, x: f64, x_next: f64, y_prev: f64, y: f64, y_next: f64) -> f64 {
    let slope_prev = slope(x_prev, y_prev, x, y);
    let slope = slope(x, y, x_next, y_next);
    // Parabola
    let dist_prev = x - x_prev;
    let dist = x_next - x;
    let para = (slope_prev * dist + slope * dist_prev) / (dist_prev + dist);
    // Tangent
    (slope_prev.signum() + slope.signum()) * slope_prev.abs().min(0.5 * para.abs())
}
