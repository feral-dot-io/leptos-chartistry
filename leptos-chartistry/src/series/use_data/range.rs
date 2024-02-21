use crate::Tick;

#[derive(Clone, Debug, PartialEq)]
pub struct Range<T>(Option<InnerRange<T>>);

#[derive(Clone, Debug, PartialEq)]
pub struct InnerRange<T> {
    pub min: (T, f64),
    pub max: (T, f64),
}

impl<T> Default for Range<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T> Range<T> {
    pub fn update(&mut self, t: &T)
    where
        T: Tick,
    {
        if let Some(range) = self.0.as_mut() {
            range.update(t);
        } else {
            *self = Range(Some(InnerRange::new(t)));
        }
    }

    pub fn maybe_update(mut self, ts: Vec<Option<T>>) -> Self
    where
        T: Tick,
    {
        ts.into_iter().flatten().for_each(|t| {
            self.update(&t);
        });
        self
    }

    // Returns the (min, max) of T if it exists
    pub fn range(&self) -> Option<(&T, &T)> {
        self.0.as_ref().map(|r| (&r.min.0, &r.max.0))
    }

    // Returns the (min, max) of T's position if it exists
    pub fn positions(&self) -> Option<(f64, f64)> {
        self.0.as_ref().map(|r| (r.min.1, r.max.1))
    }
}

impl<T: Tick> InnerRange<T> {
    pub fn new(t: &T) -> Self {
        let pos = t.position();
        Self {
            min: (t.clone(), pos),
            max: (t.clone(), pos),
        }
    }

    pub fn update(&mut self, t: &T) {
        let pos = t.position();
        if *t < self.min.0 {
            self.min = (t.clone(), pos);
        } else if *t > self.max.0 {
            self.max = (t.clone(), pos);
        }
    }
}
