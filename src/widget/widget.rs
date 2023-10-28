use crate::widget::Position;
use crate::widget::style::Style;
use crate::window::Renderer;

trait Widget {
    fn style(&self) -> Option<Style>;
    fn position(&self) -> Position;
    fn draw(&self, renderer: &dyn Renderer);
}


