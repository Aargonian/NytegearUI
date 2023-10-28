use crate::widget::color::Color;

#[derive(Default)]
pub struct Border {
    pub thickness: u32,
    pub color: Color,
}

#[derive(Default)]
pub struct Style {
    pub background_color: Color,
    pub foreground_color: Color,
    pub border: Border,
    pub corner_radius: (u32, u32, u32, u32),
}

