use super::{
    axis_marker::AxisMarker,
    grid_line::{GridLineAttr, HorizontalGridLine, UseGridLine, VerticalGridLine},
};
use crate::{
    chart::Attr,
    inner::grid_line::{UseHorizontalGridLine, UseVerticalGridLine},
    projection::Projection,
    series::UseSeries,
};
use leptos::*;

pub enum InnerOption<X: 'static, Y: 'static> {
    AxisMarker(AxisMarker),
    HorizontalGridLine(HorizontalGridLine<X>),
    VerticalGridLine(VerticalGridLine<Y>),
}

pub enum InnerAttr<X, Y> {
    AxisMarker(AxisMarker),
    HorizontalGridLine(GridLineAttr<X>),
    VerticalGridLine(GridLineAttr<Y>),
}

pub enum InnerRender<X: 'static, Y: 'static> {
    AxisMarker(AxisMarker),
    HorizontalGridLine(UseGridLine<X>),
    VerticalGridLine(UseGridLine<Y>),
}

impl<X, Y> From<AxisMarker> for InnerOption<X, Y> {
    fn from(marker: AxisMarker) -> Self {
        Self::AxisMarker(marker)
    }
}

impl<X, Y> From<HorizontalGridLine<X>> for InnerOption<X, Y> {
    fn from(line: HorizontalGridLine<X>) -> Self {
        Self::HorizontalGridLine(line)
    }
}

impl<X, Y> From<VerticalGridLine<Y>> for InnerOption<X, Y> {
    fn from(line: VerticalGridLine<Y>) -> Self {
        Self::VerticalGridLine(line)
    }
}

impl<X, Y> InnerOption<X, Y> {
    pub fn apply_attr(self, attr: &Attr) -> InnerAttr<X, Y> {
        match self {
            Self::AxisMarker(marker) => InnerAttr::AxisMarker(marker),
            Self::HorizontalGridLine(line) => InnerAttr::HorizontalGridLine(line.apply_attr(attr)),
            Self::VerticalGridLine(line) => InnerAttr::VerticalGridLine(line.apply_attr(attr)),
        }
    }
}

impl<X, Y> InnerAttr<X, Y> {
    pub fn to_use(self, series: &UseSeries<X, Y>, proj: Signal<Projection>) -> InnerRender<X, Y> {
        match self {
            Self::AxisMarker(marker) => InnerRender::AxisMarker(marker),
            Self::HorizontalGridLine(line) => {
                InnerRender::HorizontalGridLine(line.generate_x(series, proj))
            }
            Self::VerticalGridLine(line) => {
                InnerRender::VerticalGridLine(line.generate_y(series, proj))
            }
        }
    }
}

impl<X, Y> InnerRender<X, Y> {
    pub fn render(self, proj: Signal<Projection>) -> View {
        match self {
            Self::AxisMarker(marker) => view! ( <AxisMarker marker=marker projection=proj /> ),

            Self::HorizontalGridLine(line) => {
                view! ( <UseHorizontalGridLine line=line projection=proj /> )
            }
            Self::VerticalGridLine(line) => {
                view! ( <UseVerticalGridLine line=line projection=proj /> )
            }
        }
    }
}
