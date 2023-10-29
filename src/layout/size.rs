use crate::layout::dimension::Dimension;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Size {
    pub width: Dimension,
    pub height: Dimension,
}

impl From<(u32, u32)> for Size {
    fn from(value: (u32, u32)) -> Size {
        Size {
            width: Dimension::Pixel(value.0),
            height: Dimension::Pixel(value.1),
        }
    }
}

impl From<(f64, f64)> for Size {
    fn from(value: (f64, f64)) -> Size {
        Size {
            width: Dimension::Percent(value.0),
            height: Dimension::Percent(value.1),
        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AbsoluteSize {
    pub width: u32,
    pub height: u32,
}

impl From<AbsoluteSize> for Size {
    fn from(value: AbsoluteSize) -> Size {
        Size {
            width: Dimension::Pixel(value.width),
            height: Dimension::Pixel(value.height),
        }
    }
}

impl From<(u32, u32)> for AbsoluteSize {
    fn from(value: (u32, u32)) -> AbsoluteSize {
        AbsoluteSize {
            width: value.0,
            height: value.1,
        }
    }
}
