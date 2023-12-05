use super::use_series::{IntoUseLine, NextSeries, PrepareSeries};
use super::GetY;
use super::{line::UseLine, use_series::RenderSeries};
use crate::colours::Colour;
use crate::{series::line::RenderLine, state::State, Line};
use leptos::*;
use std::ops::Add;
use std::rc::Rc;

#[derive(Clone)]
pub struct Stack<T, Y> {
    lines: Vec<Line<T, Y>>,
}

#[derive(Clone)]
struct StackedLine<'a, T, Y> {
    line: &'a Line<T, Y>,
    previous: Option<GetY<T, Y>>,
}

#[derive(Clone)]
pub struct UseStack {
    lines: Vec<UseLine>,
}

impl<T, Y> Stack<T, Y> {
    pub fn new(lines: Vec<Line<T, Y>>) -> Self {
        Self { lines }
    }
}

impl<T: 'static, X, Y: Add<Output = Y> + 'static> PrepareSeries<T, X, Y> for Stack<T, Y> {
    fn prepare(self: Rc<Self>, acc: &mut NextSeries<T, Y>) -> Rc<dyn RenderSeries<X, Y>> {
        let mut lines = Vec::new();
        let mut previous: Option<GetY<T, Y>> = None;
        for line in &self.lines {
            // Add stacked line to acc
            let line = StackedLine {
                line,
                previous: previous.clone(),
            };
            let (get_y, line) = acc.add_line(&line);
            // Next line will be summed with this one
            previous = Some(get_y.clone());
            lines.push(line);
        }
        Rc::new(UseStack { lines })
    }
}

impl<'a, T: 'static, Y: Add<Output = Y> + 'static> IntoUseLine<T, Y> for StackedLine<'a, T, Y> {
    fn into_use_line(&self, id: usize, colour: Colour) -> (GetY<T, Y>, UseLine) {
        let (get_y, line) = self.line.into_use_line(id, colour);
        let previous = self.previous.clone();
        let get_y = move |t: &T| {
            previous
                .as_ref()
                .map_or_else(|| get_y(t), |prev| get_y(t) + prev(t))
        };
        (Rc::new(get_y), line)
    }
}

impl<X, Y> RenderSeries<X, Y> for UseStack {
    fn render(self: Rc<Self>, positions: Vec<Signal<Vec<(f64, f64)>>>, _: &State<X, Y>) -> View {
        view!( <RenderStack stack=&self positions=positions  /> )
    }
}

#[component]
fn RenderStack<'a>(stack: &'a UseStack, positions: Vec<Signal<Vec<(f64, f64)>>>) -> impl IntoView {
    let lines = stack.lines.clone();
    view! {
        <g class="_chartistry_stack">
            <For
                each=move || lines.clone().into_iter()
                key=|line| line.id
                let:line>
                <RenderLine line=&line positions=positions[line.id] />
            </For>
        </g>
    }
}
