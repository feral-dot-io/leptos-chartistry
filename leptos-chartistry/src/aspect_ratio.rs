use leptos::prelude::*;

/// Calculates the width and height of a chart.
///
/// An AspectRatio is built from the available constructors: `[inner|outer|env]_[width|height|ratio]`.
///
/// The first part `[inner|outer]` is a choice of what kind of dimensions we are calculating: the inner chart area or the outer chart including the edge layout. Environment auto-fills the width and / or height from the outer parent container.
///
/// The second part `[width|height|ratio]` is a choice of which variable to calculate from the formula: `width / height = ratio`.
///
/// ## Why is this important?
///
/// See the [sunspot activity example](https://feral-dot-io.github.io/leptos-chartistry/examples.html#aspect-ratio) for a visual demonstration.
///
/// ## Practical advice
///
/// Bank to 45 degrees[^bank] and preference [inner constructors](Self::from_inner_ratio).
///
/// Bank the slopes of your lines to an average of 45 degrees and you'll maximise the ability to differentiate slope differences. Sunspots become readable. This is intended as a good heuristic to start with and then adjust as needed.
///
/// You should preference "inner" dimensions when you can which leads to a chart-centric approach (the chart is always matches your intention). However charts also live on a web page with constraints -- and its in this context that you'll probably prefer to work with a chart with known "outer" dimensions. When using outer dimensions try to ensure the layout is a fixed size regardless of the input data and this will be just as good as having a fixed inner chart (setting [TickLabels::min_chars](crate::TickLabels::min_chars) on the Y axis is an example of this).
///
/// Finally the "env" dimensions work great for automatically grabbing the width or height from the parent container. The same caveats as "outer" apply but you have less control. YOLO! The risk is that if you change the page and incidentally change the dimensions, you'll change your perception of the chart.
///
/// I find that there's usually an obvious choice of width or height to pick from e.g., my page is growing horizontally or I'm expecting a lot of free space to the side. In these cases, I'll pick that variable and use a ratio that fits well for the chart. You can't really go wrong here as they're all part of the same choice.
///
/// [^bank]: Cleveland, W. S., McGill, M. E., & McGill, R. (1988). The Shape Parameter of a Two-Variable Graph. Journal of the American Statistical Association, 83(402), 289–300. <https://doi.org/10.2307/2288843>
#[derive(Clone, Debug, PartialEq)]
pub struct AspectRatio(CalcUsing);

#[derive(Clone, Debug, PartialEq)]
enum CalcUsing {
    Env(EnvCalc),
    Known(KnownAspectRatio),
}

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
enum EnvCalc {
    AutoWidthAndRatio(f64),
    AutoWidthAndHeight(f64),
    AutoHeightAndRatio(f64),
    AutoHeightAndWidth(f64),
    FullyAuto,
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

    /// The outer height is set by width / ratio.
    pub const fn from_outer_height(width: f64, ratio: f64) -> Self {
        Self::new_outer(AspectRatioVars::WidthAndRatio(width, ratio))
    }

    /// The outer width is set by height * ratio.
    pub const fn from_outer_width(height: f64, ratio: f64) -> Self {
        Self::new_outer(AspectRatioVars::HeightAndRatio(height, ratio))
    }

    /// Sets the outer width and height of the chart. Ratio is implied width / height.
    pub const fn from_outer_ratio(width: f64, height: f64) -> Self {
        Self::new_outer(AspectRatioVars::WidthAndHeight(width, height))
    }

    /// The inner height is set by the given width / ratio.
    pub const fn from_inner_height(width: f64, ratio: f64) -> Self {
        Self::new_inner(AspectRatioVars::WidthAndRatio(width, ratio))
    }

    /// The inner width is set by the given height * ratio.
    pub const fn from_inner_width(height: f64, ratio: f64) -> Self {
        Self::new_inner(AspectRatioVars::HeightAndRatio(height, ratio))
    }

    /// Sets the width and height of the inner chart. Ratio is implied width / height. If you're unsure of which constructor to use, pick this one.
    pub const fn from_inner_ratio(width: f64, height: f64) -> Self {
        Self::new_inner(AspectRatioVars::WidthAndHeight(width, height))
    }

    /// Automatically gets the width from the environment. The parent container *must* have a width. Then calls [Self::from_outer_ratio] See the notes on [Self::from_env].
    pub const fn from_env_width(height: f64) -> Self {
        Self(CalcUsing::Env(EnvCalc::AutoWidthAndHeight(height)))
    }

    /// Automatically gets the width from the environment. The parent container *must* have a width. Then calls [Self::from_outer_height]. See the notes on [Self::from_env].
    pub const fn from_env_width_apply_ratio(ratio: f64) -> Self {
        Self(CalcUsing::Env(EnvCalc::AutoWidthAndRatio(ratio)))
    }

    /// Automatically gets the height from the environment. The parent container *must* have a height. Then calls [Self::from_outer_ratio]. See the notes on [Self::from_env].
    pub const fn from_env_height(width: f64) -> Self {
        Self(CalcUsing::Env(EnvCalc::AutoHeightAndWidth(width)))
    }

    /// Automatically gets the height from the environment. The parent container *must* have a height. Then calls [Self::from_outer_width]. See the notes on [Self::from_env].
    pub const fn from_env_height_apply_ratio(ratio: f64) -> Self {
        Self(CalcUsing::Env(EnvCalc::AutoHeightAndRatio(ratio)))
    }

    /// Automatically gets the width and height from the environment. Then calls [Self::from_outer_ratio].
    ///
    /// You should avoid using this where possible as you have to be certain the parent container has an associated width and height. You should consider using [Self::from_inner_ratio] or [Self::from_outer_ratio] instead. Sometimes you'll have a width set but not a height in which case [Self::from_env_width] will still work.
    pub const fn from_env() -> Self {
        Self(CalcUsing::Env(EnvCalc::FullyAuto))
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

    pub(super) fn is_env(&self) -> bool {
        matches!(self.0, CalcUsing::Env(_))
    }
}

impl EnvCalc {
    fn into_known(self, width: f64, height: f64) -> KnownAspectRatio {
        use AspectRatioVars as V;
        use KnownAspectRatio as K;
        match self {
            Self::AutoWidthAndRatio(ratio) => K::Outer(V::WidthAndRatio(width, ratio)),
            Self::AutoWidthAndHeight(height) => K::Outer(V::WidthAndHeight(width, height)),
            Self::AutoHeightAndRatio(ratio) => K::Outer(V::HeightAndRatio(height, ratio)),
            Self::AutoHeightAndWidth(width) => K::Outer(V::WidthAndHeight(width, height)),
            Self::FullyAuto => K::Outer(V::WidthAndHeight(width, height)),
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
