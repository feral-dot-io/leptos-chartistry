use super::Range;
use crate::{
    series::{GetX, GetY},
    Tick,
};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Data<X, Y> {
    data_x: Vec<X>,
    data_y: Vec<HashMap<usize, Y>>,

    // Data index: X position to data
    x_to_data: Vec<f64>,
    // Rendering data
    coords: HashMap<usize, Vec<(f64, f64)>>,

    range_x: Range<X>,
    range_y: Range<Y>,
}

impl<X: Tick, Y: Tick> Data<X, Y> {
    pub fn new<T>(get_x: GetX<T, X>, get_ys: HashMap<usize, GetY<T, Y>>, data: &[T]) -> Self {
        let cap = data.len();
        let y_cap = get_ys.len();

        // Empty positions
        let mut built = Self {
            data_x: Vec::with_capacity(cap),
            data_y: Vec::with_capacity(cap),
            x_to_data: Vec::with_capacity(cap * y_cap),
            coords: HashMap::with_capacity(cap),
            range_x: Range::default(),
            range_y: Range::default(),
        };

        for datum in data {
            // X
            let x = (get_x)(datum);
            let x_position = x.position();
            built.range_x.update(&x);
            built.x_to_data.push(x_position);

            // Y
            let mut y_data = HashMap::with_capacity(y_cap);
            for (&id, get_y) in &get_ys {
                let y = get_y.value(datum);
                // Note: cumulative can differ from Y when stacked
                let y_stacked = get_y.stacked_value(datum);
                built.range_y.update(&y_stacked);

                // Insert
                y_data.insert(id, y);
                built
                    .coords
                    .entry(id)
                    .or_insert_with(|| Vec::with_capacity(cap))
                    .push((x_position, y_stacked.position()));
            }

            // Insert
            built.data_x.push(x);
            built.data_y.push(y_data);
        }

        built
    }

    pub fn len(&self) -> usize {
        self.data_x.len()
    }

    pub fn range_x(&self) -> Range<X> {
        self.range_x.clone()
    }

    pub fn range_y(&self) -> Range<Y> {
        self.range_y.clone()
    }

    /// Finds the index of the _nearest_ position to the given X. Returns None if no data.
    fn nearest_index(&self, pos_x: f64) -> Option<usize> {
        // No values
        if self.x_to_data.is_empty() {
            return None;
        }
        // Find index after pos
        let index = self.x_to_data.partition_point(|&v| v < pos_x);
        // No value before
        if index == 0 {
            return Some(0);
        }
        // No value ahead
        if index == self.x_to_data.len() {
            return Some(index - 1);
        }
        // Find closest index
        let ahead = self.x_to_data[index] - pos_x;
        let before = pos_x - self.x_to_data[index - 1];
        if ahead < before {
            Some(index)
        } else {
            Some(index - 1)
        }
    }

    pub fn nearest_data_x(&self, pos_x: f64) -> Option<X> {
        self.nearest_index(pos_x)
            .map(|index| self.data_x[index].clone())
    }

    pub fn nearest_data_y(&self, pos_x: f64) -> HashMap<usize, Y> {
        self.nearest_index(pos_x)
            .map(|index| self.data_y[index].clone())
            .unwrap_or_default()
    }

    /// Given an arbitrary (unaligned to data) X position, find the nearest X position aligned to data. Returns `f64::NAN` if no data. Note a position covers a range dependent on the chart width.
    pub fn nearest_position_x(&self, pos_x: f64) -> Option<f64> {
        self.nearest_index(pos_x).map(|index| self.x_to_data[index])
    }

    pub fn series_positions(&self, id: usize) -> Vec<(f64, f64)> {
        self.coords.get(&id).cloned().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[derive(Clone, Debug, PartialEq)]
    struct MyData {
        x: f64,
        y1: f64,
        y2: f64,
    }

    impl MyData {
        const fn new(x: f64, y1: f64, y2: f64) -> Self {
            Self { x, y1, y2 }
        }
    }

    const DATA: &[MyData] = &[
        MyData::new(1.0, 2.0, 3.0),
        MyData::new(4.0, 5.0, 6.0),
        MyData::new(7.0, 8.0, 9.0),
    ];

    fn test_data(data: &[MyData]) -> Data<f64, f64> {
        let mut get_ys = HashMap::<usize, GetY<_, _>>::new();
        get_ys.insert(66, Arc::new(|d: &MyData| d.y1));
        get_ys.insert(5, Arc::new(|d: &MyData| d.y2));

        Data::new(Arc::new(|d: &MyData| d.x), get_ys, data)
    }

    #[test]
    fn test_data_new() {
        let data = test_data(DATA);
        // Data
        assert_eq!(data.data_x, vec![1.0, 4.0, 7.0]);
        assert_eq!(
            data.data_y,
            vec![
                HashMap::from([(66, 2.0), (5, 3.0)]),
                HashMap::from([(66, 5.0), (5, 6.0)]),
                HashMap::from([(66, 8.0), (5, 9.0)]),
            ]
        );
        // Positions
        assert_eq!(data.x_to_data, vec![1.0, 4.0, 7.0]);
        assert_eq!(
            data.coords,
            HashMap::from([
                (66, vec![(1.0, 2.0), (4.0, 5.0), (7.0, 8.0)]),
                (5, vec![(1.0, 3.0), (4.0, 6.0), (7.0, 9.0)]),
            ])
        );
        // Ranges
        assert_eq!(data.range_x.range(), Some((&1.0, &7.0)));
        assert_eq!(data.range_x.positions(), Some((1.0, 7.0)));
        assert_eq!(data.range_y.range(), Some((&2.0, &9.0)));
        assert_eq!(data.range_y.positions(), Some((2.0, 9.0)));
    }

    #[test]
    fn test_nearest_index() {
        let data = test_data(DATA);
        // Before data
        assert_eq!(data.nearest_index(0.5), Some(0));
        // After data
        assert_eq!(data.nearest_index(8.0), Some(2));
        // Closest
        assert_eq!(data.nearest_index(3.0), Some(1));
        assert_eq!(data.nearest_index(4.0), Some(1));
        assert_eq!(data.nearest_index(5.0), Some(1));
        assert_eq!(data.nearest_index(2.0), Some(0));
        assert_eq!(data.nearest_index(6.5), Some(2));
    }

    #[test]
    fn test_nearest_index_empty() {
        let data = test_data(&[]);
        assert_eq!(data.nearest_index(0.5), None);
    }

    #[test]
    fn test_nearest_data_x() {
        let data = test_data(DATA);
        assert_eq!(data.nearest_data_x(0.5), Some(1.0));
        assert_eq!(data.nearest_data_x(8.0), Some(7.0));
        assert_eq!(data.nearest_data_x(3.0), Some(4.0));
        assert_eq!(data.nearest_data_x(4.0), Some(4.0));
    }

    #[test]
    fn test_nearest_aligned_position_x() {
        let data = test_data(DATA);
        assert_eq!(data.nearest_position_x(0.5), Some(1.0));
        assert_eq!(data.nearest_position_x(8.0), Some(7.0));
        assert_eq!(data.nearest_position_x(3.0), Some(4.0));
        assert_eq!(data.nearest_position_x(4.0), Some(4.0));
    }
}
