/// Represents the position of an element within its parent container. Positions can be absolute or
/// relative. Absolute and Relative are always relative to the parent widget. The library currently
/// does not support children widgets being absolute irrespective of parent position.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Position {
    Absolute(u32, u32),
    Relative(f64, f64),
}

impl Default for Position {
    fn default() -> Self {
        Self::Absolute(0, 0)
    }
}

impl From<(u32, u32)> for Position {
    fn from(value: (u32, u32)) -> Position {
        Self::Absolute(value.0, value.1)
    }
}

impl From<(f64, f64)> for Position {
    fn from(value: (f64, f64)) -> Self {
        Self::Relative(value.0, value.1)
    }
}