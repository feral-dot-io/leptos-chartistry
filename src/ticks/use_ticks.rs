use super::{gen::Span, GeneratedTicks, TickGen, TickState};
use crate::{series::Data, Font, Padding};
use leptos::*;
use std::rc::Rc;

pub type TickFormatFn<Tick> = Rc<dyn Fn(&dyn TickState<Tick = Tick>, &Tick) -> String>;

#[derive(Clone)]
pub struct Ticks<Tick> {
    pub(crate) font: MaybeSignal<Font>,
    pub(crate) padding: MaybeSignal<Padding>,
    pub(crate) debug: MaybeSignal<bool>,
    pub(crate) generator: Rc<dyn TickGen<Tick = Tick>>,
    pub(crate) format: TickFormatFn<Tick>,
}

impl<X: PartialEq> Ticks<X> {
    pub fn generate_x<Y>(
        self,
        data: Signal<Data<X, Y>>,
        avail_width: Signal<f64>,
    ) -> Signal<GeneratedTicks<X>> {
        let (font, padding) = (self.font, self.padding);
        create_memo(move |_| {
            let format = self.format.clone();
            data.with(|data| {
                let (first, last) = data.x_range();
                let font_width = font.get().width();
                let padding_width = padding.get().width();
                let span: HorizontalSpan<X> =
                    HorizontalSpan::new(format, font_width, padding_width, avail_width.get());
                self.generator.generate(first, last, Box::new(span))
            })
        })
        .into()
    }
}

impl<Y: PartialEq> Ticks<Y> {
    pub fn generate_y<X>(
        self,
        data: Signal<Data<X, Y>>,
        avail_height: Signal<f64>,
    ) -> Signal<GeneratedTicks<Y>> {
        let (font, padding) = (self.font, self.padding);
        create_memo(move |_| {
            data.with(|data| {
                let (first, last) = data.y_range();
                let line_height = font.get().height() + padding.get().height();
                let span = VerticalSpan::new(line_height, avail_height.get());
                self.generator.generate(first, last, Box::new(span))
            })
        })
        .into()
    }
}

pub fn short_format_fn<Tick>() -> TickFormatFn<Tick> {
    Rc::new(|state, tick| state.short_format(tick))
}

pub fn long_format_fn<Tick>() -> TickFormatFn<Tick> {
    Rc::new(|state, tick| state.long_format(tick))
}

pub struct VerticalSpan {
    avail_height: f64,
    line_height: f64,
}

impl VerticalSpan {
    pub fn new(line_height: f64, avail_height: f64) -> Self {
        Self {
            avail_height,
            line_height,
        }
    }
}

impl<Tick> Span<Tick> for VerticalSpan {
    fn length(&self) -> f64 {
        self.avail_height
    }

    fn consumed(&self, _: &dyn TickState<Tick = Tick>, ticks: &[Tick]) -> f64 {
        self.line_height * ticks.len() as f64
    }
}

pub struct HorizontalSpan<Tick> {
    format: TickFormatFn<Tick>,
    avail_width: f64,
    font_width: f64,
    padding_width: f64,
}

impl<Tick> HorizontalSpan<Tick> {
    pub fn new(
        format: TickFormatFn<Tick>,
        font_width: f64,
        padding_width: f64,
        avail_width: f64,
    ) -> Self {
        Self {
            format,
            avail_width,
            font_width,
            padding_width,
        }
    }
}

impl<Tick> Span<Tick> for HorizontalSpan<Tick> {
    fn length(&self) -> f64 {
        self.avail_width
    }

    fn consumed(&self, state: &dyn TickState<Tick = Tick>, ticks: &[Tick]) -> f64 {
        let max_chars = (ticks.into_iter())
            .map(|tick| (self.format)(state, tick).len())
            .max()
            .unwrap_or_default();
        let max_label_width = max_chars as f64 * self.font_width + self.padding_width * 2.0;
        max_label_width * ticks.len() as f64
    }
}
