mod aligned_floats;
mod gen;
mod timestamps;

pub use aligned_floats::AlignedFloatsGen;
pub use gen::{
    GenState as TickState, GeneratedTicks, Generator as TickGen, HorizontalSpan, VerticalSpan,
};
pub use timestamps::{Gen as TimestampGen, Period};

pub type TickFormatFn<Tick> = std::rc::Rc<dyn Fn(&dyn gen::GenState<Tick = Tick>, &Tick) -> String>;
