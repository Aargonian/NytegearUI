use crate::widget::color::Color;

pub trait Renderer
{
    fn draw_pixel(&mut self, x: u32, y: u32, value: u32);
    fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, value: u32);
    fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: Color);
    fn clear_background(&mut self, background_color: Color);
    fn begin(&mut self);
    fn end(&mut self);
}
