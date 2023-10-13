use super::{gen::Span, GeneratedTicks, TickGen};
use crate::{series::Data, Font, Padding};
use leptos::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Ticks<Tick> {
    pub(crate) font: MaybeSignal<Font>,
    pub(crate) padding: MaybeSignal<Padding>,
    pub(crate) debug: MaybeSignal<bool>,
    pub(crate) generator: Arc<dyn TickGen<Tick = Tick>>,
}

#[derive(Clone, Debug)]
pub struct UseTicks<Tick: 'static> {
    pub(crate) font: MaybeSignal<Font>,
    pub(crate) padding: MaybeSignal<Padding>,
    pub(crate) debug: MaybeSignal<bool>,
    pub(crate) ticks: Signal<GeneratedTicks<Tick>>,
}

impl<X> Ticks<X> {
    pub fn generate_x<Y>(self, data: Signal<Data<X, Y>>, avail_width: Signal<f64>) -> UseTicks<X> {
        let (font, padding) = (self.font, self.padding);
        UseTicks {
            font,
            padding,
            debug: self.debug,
            ticks: Signal::derive(move || {
                data.with(|data| {
                    let (first, last) = data.x_range();
                    let font_width = font.get().width();
                    let padding_width = padding.get().width();
                    let span = HorizontalSpan::new(font_width, padding_width, avail_width.get());
                    self.generator.generate(first, last, Box::new(span))
                })
            }),
        }
    }
}

impl<Y> Ticks<Y> {
    pub fn generate_y<X>(self, data: Signal<Data<X, Y>>, avail_height: Signal<f64>) -> UseTicks<Y> {
        let (font, padding) = (self.font, self.padding);
        UseTicks {
            font,
            padding,
            debug: self.debug,
            ticks: Signal::derive(move || {
                data.with(|data| {
                    let (first, last) = data.y_range();
                    let line_height = font.get().height() + padding.get().height();
                    let span = VerticalSpan::new(line_height, avail_height.get());
                    self.generator.generate(first, last, Box::new(span))
                })
            }),
        }
    }
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

impl Span for VerticalSpan {
    fn length(&self) -> f64 {
        self.avail_height
    }

    fn consumed(&self, ticks: &[&str]) -> f64 {
        self.line_height * ticks.len() as f64
    }
}

pub struct HorizontalSpan {
    avail_width: f64,
    font_width: f64,
    padding_width: f64,
}

impl HorizontalSpan {
    pub fn new(font_width: f64, padding_width: f64, avail_width: f64) -> Self {
        Self {
            avail_width,
            font_width,
            padding_width,
        }
    }
}

impl Span for HorizontalSpan {
    fn length(&self) -> f64 {
        self.avail_width
    }

    fn consumed(&self, ticks: &[&str]) -> f64 {
        let max_chars = (ticks.into_iter())
            .map(|tick| tick.len())
            .max()
            .unwrap_or_default();
        let max_label_width = max_chars as f64 * self.font_width + self.padding_width * 2.0;
        max_label_width * ticks.len() as f64
    }
}
