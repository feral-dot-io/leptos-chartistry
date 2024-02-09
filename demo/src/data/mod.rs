mod sunspots;
pub use sunspots::daily_sunspots;

use chrono::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Sunspots {
    pub year: DateTime<Utc>,
    pub sunspots: f64,
}

impl Sunspots {
    fn new(year: i32, sunspots: f64) -> Self {
        Self {
            year: Utc.with_ymd_and_hms(year, 7, 1, 0, 0, 0).unwrap(),
            sunspots,
        }
    }

    fn from_vec(data: Vec<(i32, f64)>) -> Vec<Self> {
        data.into_iter()
            .map(|(year, sunspots)| Self::new(year, sunspots))
            .collect()
    }
}
