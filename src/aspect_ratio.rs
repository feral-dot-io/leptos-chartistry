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
    /// The outer height is set by width / ratio.
    pub const fn outer_height(width: f64, ratio: f64) -> Self {
        Self(CalcUsing::Known(AspectRatioCalc::WidthAndRatio(
            Dimension::Outer(width),
            ratio,
        )))
    }

    /// The outer width is set by height * ratio.
    pub const fn outer_width(height: f64, ratio: f64) -> Self {
        Self(CalcUsing::Known(AspectRatioCalc::HeightAndRatio(
            Dimension::Outer(height),
            ratio,
        )))
    }

    /// Sets the outer width and height of the chart. Ratio is implied width / height.
    pub const fn outer_ratio(width: f64, height: f64) -> Self {
        Self(CalcUsing::Known(AspectRatioCalc::WidthAndHeight(
            Dimension::Outer(width),
            Dimension::Outer(height),
        )))
    }

    /// The inner height is set by width / ratio.
    pub const fn inner_height(width: f64, ratio: f64) -> Self {
        Self(CalcUsing::Known(AspectRatioCalc::WidthAndRatio(
            Dimension::Inner(width),
            ratio,
        )))
    }

    /// The inner width is set by height * ratio.
    pub const fn inner_width(height: f64, ratio: f64) -> Self {
        Self(CalcUsing::Known(AspectRatioCalc::HeightAndRatio(
            Dimension::Inner(height),
            ratio,
        )))
    }

    /// Sets the inner width and height of the chart. Ratio is implied width / height.
    pub const fn inner_ratio(width: f64, height: f64) -> Self {
        Self(CalcUsing::Known(AspectRatioCalc::WidthAndHeight(
            Dimension::Inner(width),
            Dimension::Inner(height),
        )))
    }

    /// The outer height is set by the width of the parent container and a given ratio (width / ratio).
    pub const fn environment_height(ratio: f64) -> Self {
        Self(CalcUsing::Env(EnvCalc::WidthAndRatio(ratio)))
    }

    /// The outer width is set by the height of the parent container and a given ratio (height * ratio).
    pub const fn environment_width(ratio: f64) -> Self {
        Self(CalcUsing::Env(EnvCalc::HeightAndRatio(ratio)))
    }

    /// Uses both the width and height of the parent container.
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
    pub fn inner_width_signal(
        calc: Memo<AspectRatioCalc>,
        left: Memo<f64>,
        right: Memo<f64>,
    ) -> Memo<f64> {
        create_memo(move |_| {
            let options = left.get() + right.get();
            match calc.get() {
                AspectRatioCalc::WidthAndRatio(width, _) => width.size(options),
                AspectRatioCalc::HeightAndRatio(height, ratio) => height.size(options) * ratio,
                AspectRatioCalc::WidthAndHeight(width, _) => width.size(options),
            }
        })
    }

    pub fn inner_height_signal(
        calc: Memo<AspectRatioCalc>,
        top: Memo<f64>,
        bottom: Memo<f64>,
    ) -> Memo<f64> {
        create_memo(move |_| {
            let options = top.get() + bottom.get();
            match calc.get() {
                AspectRatioCalc::WidthAndRatio(width, ratio) => width.size(options) / ratio,
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
