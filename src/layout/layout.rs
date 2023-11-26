use crate::bounds::Bounds;
use leptos::signal_prelude::*;
use std::rc::Rc;

/*
Compose:
    - Start with some form of bounds (either inner or outer)
    - List of top / bottom sizes.
    - Constrains the layout.
    - Calculate the left / right sizes using contraints.
    - Return a composed layout.
*/

#[derive(Clone, Debug, PartialEq)]
pub struct Layout {
    pub outer: Signal<Bounds>,
    pub top: Vec<Signal<f64>>,
    pub right: Vec<Signal<f64>>,
    pub bottom: Vec<Signal<f64>>,
    pub left: Vec<Signal<f64>>,
    pub inner: Signal<Bounds>,
}

pub trait HorizontalOption {
    fn height(&self) -> Signal<f64>;
}

pub trait VerticalOption {
    fn width(&self) -> Signal<f64>;
}

pub struct ConstrainedLayout {
    pub top: Vec<Signal<f64>>,
    pub top_height: Signal<f64>,
    pub bottom: Vec<Signal<f64>>,
    pub bottom_height: Signal<f64>,
}

impl ConstrainedLayout {
    // TODO: pass vec of signals?
    pub fn new(top: Vec<&dyn HorizontalOption>, bottom: Vec<&dyn HorizontalOption>) -> Self {
        // TODO: ensure bottom is reversed prior to this point.

        let top = top.into_iter().map(|opt| opt.height()).collect::<Vec<_>>();
        let bottom = bottom
            .into_iter()
            .map(|opt| opt.height())
            .collect::<Vec<_>>();

        let top_height = {
            let top = top.to_owned();
            Signal::derive(move || top.iter().map(|opt| opt.get()).sum::<f64>())
        };
        let bottom_height = {
            let bottom = bottom.to_owned();
            Signal::derive(move || bottom.iter().map(|opt| opt.get()).sum::<f64>())
        };

        Self {
            top,
            top_height,
            bottom,
            bottom_height,
        }
    }

    pub fn compose(left: Vec<&dyn VerticalOption>, right: Vec<&dyn VerticalOption>) -> Layout {
        //
    }
}
