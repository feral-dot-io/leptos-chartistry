pub mod data;
pub mod examples;
pub mod pages {
    pub mod demo;
    pub mod examples {
        mod all_examples;
        pub use all_examples::*;
    }
}
