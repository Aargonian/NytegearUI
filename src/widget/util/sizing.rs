use crate::widget::Dimension;

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

#[derive(Default, Clone, Copy, Debug, PartialEq)]
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

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Margin {
    pub top: Dimension,
    pub right: Dimension,
    pub bottom: Dimension,
    pub left: Dimension,
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Padding {
    pub top: Dimension,
    pub right: Dimension,
    pub bottom: Dimension,
    pub left: Dimension,
}

/// The Sizing and Positional information for a Widget
///
/// Use this struct to position your widget within the parent element and provide sizing hints for
/// the layout engine. Positional information may be ignored by the layout engine depending on the
/// layout used by the parent widget, and sizing information will be used for a hint-style system.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Sizing {
    pub margin: Margin,
    pub padding: Padding,
    pub min_size: Option<Size>,
    pub max_size: Option<Size>,
    pub size: Size,
}

impl Sizing {
    /// Calculates the size this widget will take up given an available amount of space.
    ///
    /// This calculation will use a combination of the margin, padding, and sizing values to
    /// determine the final size of the widget. If the available space is less than the combination
    /// of margin, padding, and the minimum size of this widget, the return value will be greater
    /// than the available space. If the available space is larger than the combination of padding,
    /// margin, and the max size of this Sizing, it will be less than the available space. For all
    /// other situations, the returned Spacing value will be the calculated margin, padding, and
    /// size of this widget sizing within the available space.
    ///
    /// In cases where the size has (likely incorrectly) been configured to be larger than the max
    /// size, or smaller than the min size, the larger or smaller of the two will be used,
    /// respectively.
    ///
    /// The returned size from this function will always have a `None` value for the `min_size` and
    /// `max_size` fields. The returned `Sizing` object will have the margin, padding, and inner
    /// size value to reserve for this widget.
    pub fn calculate_sizing(&self, available_space: AbsoluteSize) -> Sizing {
        // Helper function for converting dimensions to u32
        fn calculate_dimension_size(dimension: Dimension, available_space: u32) -> u32 {
            match dimension {
                Dimension::Pixel(value) => value,
                Dimension::Percent(percent) => (available_space as f64 * percent) as u32
            }
        }

        let top_pad = calculate_dimension_size(self.padding.top, available_space.height);
        let right_pad = calculate_dimension_size(self.padding.right, available_space.width);
        let bottom_pad = calculate_dimension_size(self.padding.bottom, available_space.height);
        let left_pad = calculate_dimension_size(self.padding.left, available_space.width);

        let top_margin = calculate_dimension_size(self.margin.top, available_space.height);
        let right_margin = calculate_dimension_size(self.margin.right, available_space.width);
        let bottom_margin = calculate_dimension_size(self.margin.bottom, available_space.height);
        let left_margin = calculate_dimension_size(self.margin.left, available_space.width);

        // We use box-model sizing, so min/max are calculated to the border not including padding
        let remaining_height = available_space.height - (top_margin + bottom_margin);
        let remaining_width = available_space.width - (left_margin + right_margin);

        // Calculate our minimum sizes in case min size is greater than declared size
        let possible_min_width = calculate_dimension_size(self.size.width, remaining_width);
        let possible_min_height = calculate_dimension_size(self.size.height, remaining_height);
        let min_width = if let Some(min_size) = self.min_size {
            let declared_min_width = calculate_dimension_size(min_size.width, remaining_width);
            declared_min_width.min(possible_min_width)
        } else {
            possible_min_width
        };
        let min_height = if let Some(min_size) = self.min_size {
            let declared_min_height = calculate_dimension_size(min_size.height, remaining_height);
            declared_min_height.min(possible_min_height)
        } else {
            possible_min_height
        };


        // Calculate our maximum sizes in case max size is greater than declared size
        let possible_max_width = calculate_dimension_size(self.size.width, remaining_width);
        let possible_max_height = calculate_dimension_size(self.size.height, remaining_height);
        let max_width = if let Some(max_size) = self.max_size {
            let declared_max_width = calculate_dimension_size(max_size.width, remaining_width);
            declared_max_width.max(possible_max_width)
        } else {
            possible_max_width
        };
        let max_height = if let Some(max_size) = self.max_size {
            let declared_max_height = calculate_dimension_size(max_size.height, remaining_height);
            declared_max_height.max(possible_max_height)
        } else {
            possible_max_height
        };

        // Determine final sizes.
        let final_width = if remaining_width < min_width {
            min_width
        } else if remaining_width > max_width {
            max_width
        } else {
            remaining_width
        };

        let final_height = if remaining_height < min_height {
            min_height
        } else if remaining_height > max_height {
            max_height
        } else {
            remaining_height
        };

        let size = Size {
            width: final_width.into(),
            height: final_height.into(),
        };
        let padding = Padding {
            top: top_pad.into(),
            right: right_pad.into(),
            bottom: bottom_pad.into(),
            left: left_pad.into(),
        };
        let margin = Margin {
            top: top_margin.into(),
            right: right_margin.into(),
            bottom: bottom_margin.into(),
            left: left_margin.into(),
        };

        Sizing {
            min_size: None,
            max_size: None,
            padding,
            margin,
            size,
        }
    }
}
