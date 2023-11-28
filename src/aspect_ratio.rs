use leptos::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AspectRatio(pub(crate) CalcUsing);

#[derive(Clone, Debug, PartialEq)]
pub enum CalcUsing {
    Env(EnvCalc),
    Known(AspectRatioCalc),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EnvCalc {
    WidthAndRatio(f64),
    HeightAndRatio(f64),
    WidthAndHeight,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AspectRatioCalc {
    WidthAndRatio(Dimension, f64),
    HeightAndRatio(Dimension, f64),
    WidthAndHeight(Dimension, Dimension),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Dimension {
    Outer(f64),
    Inner(f64),
}

impl AspectRatio {
    /// Sets the outer width of the chart and applies a ratio.
    pub const fn outer_width(width: f64, ratio: f64) -> Self {
        Self(CalcUsing::Known(AspectRatioCalc::WidthAndRatio(
            Dimension::Outer(width),
            ratio,
        )))
    }

    /// Sets the outer height of the chart and applies a ratio.
    pub const fn outer_height(height: f64, ratio: f64) -> Self {
        Self(CalcUsing::Known(AspectRatioCalc::HeightAndRatio(
            Dimension::Outer(height),
            ratio,
        )))
    }

    /// Sets the outer width and height of the chart.
    pub const fn outer(width: f64, height: f64) -> Self {
        Self(CalcUsing::Known(AspectRatioCalc::WidthAndHeight(
            Dimension::Outer(width),
            Dimension::Outer(height),
        )))
    }

    /// Sets the inner width of the chart and applies a ratio.
    pub const fn inner_width(width: f64, ratio: f64) -> Self {
        Self(CalcUsing::Known(AspectRatioCalc::WidthAndRatio(
            Dimension::Inner(width),
            ratio,
        )))
    }

    /// Sets the inner height of the chart and applies a ratio.
    pub const fn inner_height(height: f64, ratio: f64) -> Self {
        Self(CalcUsing::Known(AspectRatioCalc::HeightAndRatio(
            Dimension::Inner(height),
            ratio,
        )))
    }

    /// Sets the inner width and height of the chart.
    pub const fn inner(width: f64, height: f64) -> Self {
        Self(CalcUsing::Known(AspectRatioCalc::WidthAndHeight(
            Dimension::Inner(width),
            Dimension::Inner(height),
        )))
    }

    /// Gets the width from the parent container and applies a ratio.
    pub const fn environment_width(ratio: f64) -> Self {
        Self(CalcUsing::Env(EnvCalc::WidthAndRatio(ratio)))
    }

    /// Gets the height from the parent container and applies a ratio.
    pub const fn environment_height(ratio: f64) -> Self {
        Self(CalcUsing::Env(EnvCalc::HeightAndRatio(ratio)))
    }

    /// Gets the width and height from the parent container.
    pub const fn environment() -> Self {
        Self(CalcUsing::Env(EnvCalc::WidthAndHeight))
    }
}

impl EnvCalc {
    pub fn mk_signal(self, width: Memo<f64>, height: Memo<f64>) -> AspectRatioCalc {
        use AspectRatioCalc as C;
        use Dimension as D;
        match self {
            EnvCalc::WidthAndRatio(ratio) => C::WidthAndRatio(D::Outer(width.get()), ratio),
            EnvCalc::HeightAndRatio(ratio) => C::HeightAndRatio(D::Outer(height.get()), ratio),
            EnvCalc::WidthAndHeight => {
                C::WidthAndHeight(D::Outer(width.get()), D::Outer(height.get()))
            }
        }
    }
}

impl AspectRatioCalc {
    pub fn inner_width_signal(self, left: Memo<f64>, right: Memo<f64>) -> Memo<f64> {
        create_memo(move |_| {
            let options = left.get() + right.get();
            match self {
                AspectRatioCalc::WidthAndRatio(width, _) => width.size(options),
                AspectRatioCalc::HeightAndRatio(height, ratio) => height.size(options) / ratio,
                AspectRatioCalc::WidthAndHeight(width, _) => width.size(options),
            }
        })
    }

    pub fn inner_height_signal(self, top: Memo<f64>, bottom: Memo<f64>) -> Memo<f64> {
        create_memo(move |_| {
            let options = top.get() + bottom.get();
            match self {
                AspectRatioCalc::WidthAndRatio(width, ratio) => width.size(options) * ratio,
                AspectRatioCalc::HeightAndRatio(height, _) => height.size(options),
                AspectRatioCalc::WidthAndHeight(_, height) => height.size(options),
            }
        })
    }
}

impl Dimension {
    pub fn size(self, options: f64) -> f64 {
        match self {
            Dimension::Outer(value) => value - options,
            Dimension::Inner(value) => value,
        }
    }
}
