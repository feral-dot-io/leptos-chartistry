//mod compose;
mod layout;
pub mod legend;
pub mod rotated_label;
pub mod snippet;
pub mod tick_labels;

//pub use compose::{HorizontalLayout, UnconstrainedLayout, VerticalLayout};
pub use layout::{HorizontalLayout, Layout, VerticalLayout};
