#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self { a, r, g, b }
    }
}

impl From<u32> for Color {
    fn from(argb: u32) -> Self {
        Self {
            a: ((argb >> 24) & 0xff) as u8,
            r: ((argb >> 16) & 0xff) as u8,
            g: ((argb >> 8) & 0xff) as u8,
            b: (argb & 0xff) as u8,
        }
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        ((self.a as u32) << 24)
            | ((self.r as u32) << 16)
            | ((self.g as u32) << 8)
            | (self.b as u32)
    }
}

// Constants for common colors
pub const BLACK: Color = Color::new(255, 0, 0, 0);
pub const WHITE: Color = Color::new(255, 255, 255, 255);
pub const RED: Color = Color::new(255, 255, 0, 0);
pub const GREEN: Color = Color::new(255, 0, 255, 0);
pub const BLUE: Color = Color::new(255, 0, 0, 255);
pub const YELLOW: Color = Color::new(255, 255, 255, 0);
pub const ORANGE: Color = Color::new(255, 255, 165, 0);
pub const PURPLE: Color = Color::new(255, 128, 0, 128);
pub const GRAY: Color = Color::new(255, 128, 128, 128);
pub const LIGHT_GRAY: Color = Color::new(255, 192, 192, 192);
pub const DARK_GRAY: Color = Color::new(255, 64, 64, 64);
