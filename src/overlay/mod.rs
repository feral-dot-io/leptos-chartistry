pub mod tooltip;

use crate::state::State;
use leptos::*;
use std::rc::Rc;

pub trait OverlayLayout<X, Y>: private::Sealed {
    fn render(self: Rc<Self>, state: State<X, Y>) -> View;
}

mod private {
    pub trait Sealed {}
    impl<X, Y> Sealed for super::tooltip::Tooltip<X, Y> {}
}
