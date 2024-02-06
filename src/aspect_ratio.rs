use leptos::*;

/// Calculates width and height for a chart.
///
/// TODO greatly expand the documentation.
#[derive(Clone, Debug, PartialEq)]
pub struct AspectRatio(CalcUsing);

#[derive(Clone, Debug, PartialEq)]
enum CalcUsing {
    Env(EnvCalc),
    Known(KnownAspectRatio),
}

#[derive(Clone, Debug, PartialEq)]
enum EnvCalc {
    WidthAndRatio(f64),
    HeightAndRatio(f64),
    WidthAndHeight,
}

#[derive(Clone, Debug, PartialEq)]
pub enum KnownAspectRatio {
    Outer(AspectRatioVars),
    Inner(AspectRatioVars),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AspectRatioVars {
    WidthAndRatio(f64, f64),
    HeightAndRatio(f64, f64),
    WidthAndHeight(f64, f64),
}

impl AspectRatio {
    const fn new_outer(vars: AspectRatioVars) -> Self {
        Self(CalcUsing::Known(KnownAspectRatio::Outer(vars)))
    }

    const fn new_inner(vars: AspectRatioVars) -> Self {
        Self(CalcUsing::Known(KnownAspectRatio::Inner(vars)))
    }

    /// The height is set by width / ratio.
    pub const fn outer_height(width: f64, ratio: f64) -> Self {
        Self::new_outer(AspectRatioVars::WidthAndRatio(width, ratio))
    }

    /// The width is set by height * ratio.
    pub const fn outer_width(height: f64, ratio: f64) -> Self {
        Self::new_outer(AspectRatioVars::HeightAndRatio(height, ratio))
    }

    /// Sets the width and height of the chart. Ratio is implied width / height.
    pub const fn outer_ratio(width: f64, height: f64) -> Self {
        Self::new_outer(AspectRatioVars::WidthAndHeight(width, height))
    }

    /// The height is set by width / ratio.
    pub const fn inner_height(width: f64, ratio: f64) -> Self {
        Self::new_inner(AspectRatioVars::WidthAndRatio(width, ratio))
    }

    /// The width is set by height * ratio.
    pub const fn inner_width(height: f64, ratio: f64) -> Self {
        Self::new_inner(AspectRatioVars::HeightAndRatio(height, ratio))
    }

    /// Sets the width and height of the chart. Ratio is implied width / height.
    pub const fn inner_ratio(width: f64, height: f64) -> Self {
        Self::new_inner(AspectRatioVars::WidthAndHeight(width, height))
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

    pub(crate) fn known_signal(
        aspect_ratio: MaybeSignal<Self>,
        env_width: Memo<f64>,
        env_height: Memo<f64>,
    ) -> Memo<KnownAspectRatio> {
        create_memo(move |_| {
            let env_width = env_width.get();
            let env_height = env_height.get();
            match aspect_ratio.get().0 {
                CalcUsing::Env(calc) => calc.into_known(env_width, env_height),
                CalcUsing::Known(calc) => calc,
            }
        })
    }
}

impl EnvCalc {
    fn into_known(self, width: f64, height: f64) -> KnownAspectRatio {
        use AspectRatioVars as C;
        use KnownAspectRatio as K;
        match self {
            Self::WidthAndRatio(ratio) => K::Outer(C::WidthAndRatio(width, ratio)),
            Self::HeightAndRatio(ratio) => K::Outer(C::HeightAndRatio(height, ratio)),
            Self::WidthAndHeight => K::Outer(C::WidthAndHeight(width, height)),
        }
    }
}

impl KnownAspectRatio {
    pub fn inner_width_signal(known: Memo<Self>, left: Memo<f64>, right: Memo<f64>) -> Memo<f64> {
        create_memo(move |_| match known.get() {
            Self::Inner(vars) => vars.width(),
            Self::Outer(vars) => vars.width() - left.get() - right.get(),
        })
    }

    pub fn inner_height_signal(known: Memo<Self>, top: Memo<f64>, bottom: Memo<f64>) -> Memo<f64> {
        create_memo(move |_| match known.get() {
            Self::Inner(vars) => vars.height(),
            Self::Outer(vars) => vars.height() - top.get() - bottom.get(),
        })
    }
}

impl AspectRatioVars {
    pub fn width(self) -> f64 {
        match self {
            Self::WidthAndRatio(width, _) => width,
            Self::HeightAndRatio(height, ratio) => height * ratio,
            Self::WidthAndHeight(width, _) => width,
        }
    }

    pub fn height(self) -> f64 {
        match self {
            Self::WidthAndRatio(width, ratio) => width / ratio,
            Self::HeightAndRatio(height, _) => height,
            Self::WidthAndHeight(_, height) => height,
        }
    }
}
