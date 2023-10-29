use crate::layout::Dimension;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Spacing {
    pub top: Dimension,
    pub right: Dimension,
    pub bottom: Dimension,
    pub left: Dimension,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AbsoluteSpacing {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

impl Spacing {
    #[inline]
    const fn equal_spacing(value: Dimension) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }
}

impl AbsoluteSpacing {
    #[inline]
    const fn equal_spacing(value: u32) -> Self {
        Self {
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

impl From<(u32, u32, u32, u32)> for AbsoluteSpacing {
    fn from(value: (u32, u32, u32, u32)) -> Self {
        Self {
            top: value.0,
            right: value.1,
            bottom: value.2,
            left: value.3,
        }
    }
}

impl From<u32> for AbsoluteSpacing {
    fn from(value: u32) -> Self {
        Self::equal_spacing(value)
    }
}

impl From<AbsoluteSpacing> for Spacing {
    fn from(value: AbsoluteSpacing) -> Self {
        Self {
            top: value.top.into(),
            right: value.right.into(),
            bottom: value.bottom.into(),
            left: value.left.into(),
        }
    }
}