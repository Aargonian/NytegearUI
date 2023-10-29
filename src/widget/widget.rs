use crate::layout::{Layout, Position};
use crate::Renderer;
use crate::widget::style::Style;

pub trait Widget {
    fn style(&self) -> Option<Style>;

    fn layout(&self) -> Layout;

    fn position(&self) -> Position;

    fn draw(&self, renderer: &mut dyn Renderer);
}


