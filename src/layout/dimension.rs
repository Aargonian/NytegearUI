#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Dimension {
    Percent(f64),
    Pixel(u32),
}

impl Default for Dimension {
    fn default() -> Self {
        Dimension::Pixel(0)
    }
}

impl Into<Dimension> for u32 {
    fn into(self) -> Dimension {
        Dimension::Pixel(self)
    }
}

impl Into<Dimension> for f64 {
    fn into(self) -> Dimension {
        Dimension::Percent(self)
    }
}

impl Dimension {
    pub fn calculate_dimension_size(&self, available_space: u32) -> u32 {
        match self {
            Self::Pixel(value) => *value,
            Self::Percent(percent) => ((percent * available_space as f64)/100.0) as u32
        }
    }
}
