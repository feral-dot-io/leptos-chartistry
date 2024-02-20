use crate::Tick;

#[derive(Clone, Debug, PartialEq)]
pub struct Range<T>(Option<InnerRange<T>>);

#[derive(Clone, Debug, PartialEq)]
pub struct InnerRange<T> {
    pub min: T,
    pub max: T,

    pub min_position: f64,
    pub max_position: f64,
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
