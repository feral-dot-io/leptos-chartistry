#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum Interpolation {
    #[default]
    Linear,
    //Step,
    //Monotone,
}

impl Interpolation {
    pub fn path(self, points: &[(f64, f64)]) -> String {
        match self {
            Self::Linear => linear(points),
            //Self::Step => step(points),
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
