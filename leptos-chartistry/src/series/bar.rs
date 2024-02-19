use super::GetYValue;
use crate::Colour;
use leptos::signal_prelude::*;
use std::rc::Rc;

pub struct Bar<T, Y> {
    get_y: Rc<dyn GetYValue<T, Y>>,
    pub name: RwSignal<String>,
    pub colour: RwSignal<Option<Colour>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UseBar {
    pub id: usize,
    pub name: RwSignal<String>,
    colour: Signal<Colour>,
}

impl<T, Y> Bar<T, Y> {
    pub fn new(get_y: impl Fn(&T) -> Y + 'static) -> Self {
        Self {
            get_y: Rc::new(get_y),
            name: RwSignal::default(),
            colour: RwSignal::default(),
        }
    }
}

impl<T, Y> Clone for Bar<T, Y> {
    fn clone(&self) -> Self {
        Self {
            get_y: self.get_y.clone(),
            name: self.name,
            colour: self.colour,
        }
    }
}
