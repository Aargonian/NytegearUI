use crate::layout::Dimension;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Spacing {
    pub top: Dimension,
    pub right: Dimension,
    pub bottom: Dimension,
    pub left: Dimension,
}

impl Spacing {
    #[inline]
    const fn equal_spacing(value: Dimension) -> Spacing {
        Spacing {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }
}

impl From<(u32, u32, u32, u32)> for Spacing {
    fn from(spacing: (u32, u32, u32, u32)) -> Self {
        Self {
            top: spacing.0.into(),
            right: spacing.1.into(),
            bottom: spacing.2.into(),
            left: spacing.3.into(),
        }
    }
}

impl From<u32> for Spacing {
    fn from(spacing: u32) -> Self {
        Self::equal_spacing(spacing.into())
    }
}

impl From<(f64, f64, f64, f64)> for Spacing {
    fn from(spacing: (f64, f64, f64, f64)) -> Self {
        Self {
            top: spacing.0.into(),
            right: spacing.1.into(),
            bottom: spacing.2.into(),
            left: spacing.3.into(),
        }
    }
}

impl From<f64> for Spacing {
    fn from(spacing: f64) -> Self {
        Self::equal_spacing(spacing.into())
    }
}
