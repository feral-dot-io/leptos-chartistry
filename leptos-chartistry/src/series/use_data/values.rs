use crate::{
    series::{GetX, GetY},
    Tick,
};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Values<X, Y> {
    pub data_x: Vec<X>,
    pub data_y: Vec<HashMap<usize, Y>>,

    pub positions_x: Vec<f64>,
    pub positions_y: Vec<HashMap<usize, f64>>,

    pub range_x: Range<X>,
    pub range_y: Range<Y>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Range<T>(Option<InnerRange<T>>);

#[derive(Clone, Debug, PartialEq)]
pub struct InnerRange<T> {
    pub min: T,
    pub max: T,

    pub min_position: f64,
    pub max_position: f64,
}

impl<X: Tick, Y: Tick> Values<X, Y> {
    pub fn new<T>(get_x: GetX<T, X>, get_ys: HashMap<usize, GetY<T, Y>>, data: &[T]) -> Self {
        let cap = data.len();
        let y_cap = get_ys.len();

        // Empty positions
        let mut built = Self {
            data_x: Vec::with_capacity(cap),
            data_y: Vec::with_capacity(cap),
            positions_x: Vec::with_capacity(cap),
            positions_y: Vec::with_capacity(cap),
            range_x: Range::default(),
            range_y: Range::default(),
        };

        for datum in data {
            // X
            let x = (get_x)(datum);
            let x_position = x.position();
            built.range_x.update(&x, x_position);

            built.data_x.push(x.clone());
            built.positions_x.push(x_position);

            // Y
            let mut y_data = HashMap::with_capacity(y_cap);
            let mut y_positions = HashMap::with_capacity(y_cap);
            for (&id, get_y) in get_ys.iter() {
                let y = get_y.value(datum);
                let y_cumulative = get_y.cumulative_value(datum);
                // Note: cumulative can differ from Y when stacked
                let y_position = y_cumulative.position();
                built.range_y.update(&y, y_position);

                y_data.insert(id, y);
                y_positions.insert(id, y_position);
            }

            built.data_y.push(y_data);
            built.positions_y.push(y_positions);
        }
        built
    }
}

impl<T> Default for Range<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T> Range<T> {
    pub fn update(&mut self, t: &T, pos: f64)
    where
        T: Tick,
    {
        if let Some(range) = self.0.as_mut() {
            range.update(t, pos);
        } else {
            *self = Range(Some(InnerRange::new(t, pos)));
        }
    }

    pub fn maybe_update(mut self, ts: Vec<Option<T>>) -> Self
    where
        T: Tick,
    {
        ts.into_iter().flatten().for_each(|t| {
            self.update(&t, t.position());
        });
        self
    }

    // Returns the (min, max) of T if it exists
    pub fn range(&self) -> Option<(&T, &T)> {
        self.0.as_ref().map(|r| (&r.min, &r.max))
    }

    // Returns the (min, max) of T's position if it exists
    pub fn positions(&self) -> Option<(f64, f64)> {
        self.0.as_ref().map(|r| (r.min_position, r.max_position))
    }
}

impl<T: Tick> InnerRange<T> {
    pub fn new(t: &T, pos: f64) -> Self {
        Self {
            min: t.clone(),
            max: t.clone(),
            min_position: pos,
            max_position: pos,
        }
    }

    pub fn update(&mut self, t: &T, pos: f64) {
        if *t < self.min {
            self.min = t.clone();
        } else if *t > self.max {
            self.max = t.clone();
        }
        if pos < self.min_position {
            self.min_position = pos;
        } else if pos > self.max_position {
            self.max_position = pos;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[derive(Clone, Debug, PartialEq)]
    struct MyData {
        x: f64,
        y1: f64,
        y2: f64,
    }

    impl MyData {
        pub fn new(x: f64, y1: f64, y2: f64) -> Self {
            Self { x, y1, y2 }
        }
    }

    #[test]
    fn test_positions_new() {
        let mut get_ys = HashMap::<usize, GetY<_, _>>::new();
        get_ys.insert(66, Rc::new(|d: &MyData| d.y1));
        get_ys.insert(5, Rc::new(|d: &MyData| d.y2));

        let pos = Values::new(
            Rc::new(|d: &MyData| d.x),
            get_ys,
            &[
                MyData::new(1.0, 2.0, 3.0),
                MyData::new(4.0, 5.0, 6.0),
                MyData::new(7.0, 8.0, 9.0),
            ],
        );
        // Data
        assert_eq!(pos.data_x, vec![1.0, 4.0, 7.0]);
        assert_eq!(
            pos.data_y,
            vec![
                HashMap::from([(66, 2.0), (5, 3.0)]),
                HashMap::from([(66, 5.0), (5, 6.0)]),
                HashMap::from([(66, 8.0), (5, 9.0)]),
            ]
        );
        // Positions
        assert_eq!(pos.positions_x, vec![1.0, 4.0, 7.0]);
        assert_eq!(
            pos.positions_y,
            vec![
                HashMap::from([(66, 2.0), (5, 3.0)]),
                HashMap::from([(66, 5.0), (5, 6.0)]),
                HashMap::from([(66, 8.0), (5, 9.0)]),
            ]
        );
        // Ranges
        assert_eq!(
            pos.range_x,
            Range(Some(InnerRange {
                min: 1.0,
                max: 7.0,
                min_position: 1.0,
                max_position: 7.0,
            }))
        );
        assert_eq!(
            pos.range_y,
            Range(Some(InnerRange {
                min: 2.0,
                max: 9.0,
                min_position: 2.0,
                max_position: 9.0,
            }))
        );
    }
}
