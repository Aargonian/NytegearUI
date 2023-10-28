extern crate nytegearui;

use nytegearui::widget::{AbsoluteSize, Dimension, Margin, Padding, Size, Sizing};

#[test]
fn test_desired_size_only() {
    let width = 500;
    let height = 300;

    let child_size = Sizing {
        padding: Default::default(),
        margin: Default::default(),
        min_size: None,
        max_size: None,
        size: Size::from((width, height)),
    };

    // Preferred Size with Equal Space
    let available_space = AbsoluteSize::from((width, height));
    let preferred_sizing = child_size.calculate_sizing(available_space);
    assert_eq!(preferred_sizing.size.width, Dimension::Pixel(width));
    assert_eq!(preferred_sizing.size.height, Dimension::Pixel(height));

    // Preferred Size with Greater Space
    let available_space = AbsoluteSize::from((width + 100, height + 100));
    let preferred_sizing = child_size.calculate_sizing(available_space);
    assert_eq!(preferred_sizing.size.width, Dimension::Pixel(width));
    assert_eq!(preferred_sizing.size.height, Dimension::Pixel(height));

    // Preferred Size with Less Space
    let available_space = AbsoluteSize::from((width - 100, height - 100));
    let preferred_sizing = child_size.calculate_sizing(available_space);
    assert_eq!(preferred_sizing.size.width, Dimension::Pixel(width));
    assert_eq!(preferred_sizing.size.height, Dimension::Pixel(height));
}

#[test]
fn test_fixed_desired_size_and_no_margin() {
    let padding_top = 20;
    let padding_bottom = 30;
    let padding_left = 40;
    let padding_right = 60;
    let width = 500;
    let height = 300;

    let child_size = Sizing {
        padding: Padding {
            top: padding_top.into(),
            right: padding_right.into(),
            left: padding_left.into(),
            bottom: padding_bottom.into(),
        },
        margin: Default::default(),
        min_size: None,
        max_size: None,
        size: Size::from((width, height)),
    };

    // The Sizing should not change for any test
    let expected_sizing = Sizing {
        padding: Padding {
            top: padding_top.into(),
            right: padding_right.into(),
            left: padding_left.into(),
            bottom: padding_bottom.into(),
        },
        margin: Default::default(),
        min_size: None,
        max_size: None,
        size: Size::from((width, height)),
    };

    let width_with_padding = width + padding_left + padding_right;
    let height_with_padding = height + padding_top + padding_bottom;

    // Preferred Size with Equal Space (including padding)
    let available_space = AbsoluteSize::from((width_with_padding, height_with_padding));
    let preferred_sizing = child_size.calculate_sizing(available_space);
    assert_eq!(preferred_sizing, expected_sizing);

    // Preferred Size with Equal Space (excluding padding)
    let available_space = AbsoluteSize::from((width, height));
    let preferred_sizing = child_size.calculate_sizing(available_space);
    assert_eq!(preferred_sizing, expected_sizing);

    // Preferred Size with Greater Space
    let available_space = AbsoluteSize::from((width_with_padding + 100, height_with_padding + 100));
    let preferred_sizing = child_size.calculate_sizing(available_space);
    assert_eq!(preferred_sizing, expected_sizing);

    // Preferred Size with Less Space
    let available_space = AbsoluteSize::from((width - 100, height - 100));
    let preferred_sizing = child_size.calculate_sizing(available_space);
    assert_eq!(preferred_sizing, expected_sizing);
}

#[test]
fn test_fixed_desired_size_and_no_padding() {
    let margin_top = 20;
    let margin_bottom = 30;
    let margin_left = 40;
    let margin_right = 60;
    let width = 500;
    let height = 300;

    let child_size = Sizing {
        margin: Margin {
            top: margin_top.into(),
            right: margin_right.into(),
            left: margin_left.into(),
            bottom: margin_bottom.into(),
        },
        padding: Default::default(),
        min_size: None,
        max_size: None,
        size: Size::from((width, height)),
    };

    // Sizing should not change at any point
    let expected_sizing = Sizing {
        margin: Margin {
            top: margin_top.into(),
            right: margin_right.into(),
            left: margin_left.into(),
            bottom: margin_bottom.into(),
        },
        padding: Default::default(),
        min_size: None,
        max_size: None,
        size: Size::from((width, height)),
    };
    let height_with_margin = height + margin_top + margin_bottom;
    let width_with_margin = width + margin_left + margin_right;

    // Preferred Size with Equal Space (including margin)
    let available_space = AbsoluteSize::from((width_with_margin, height_with_margin));
    let preferred_sizing = child_size.calculate_sizing(available_space);
    assert_eq!(preferred_sizing, expected_sizing);

    // Preferred Size with Equal Space (excluding margin)
    let available_space = AbsoluteSize::from((width, height));
    let preferred_sizing = child_size.calculate_sizing(available_space);
    assert_eq!(preferred_sizing, expected_sizing);

    // Preferred Size with Greater Space
    let available_space = AbsoluteSize::from((width_with_margin + 100, height_with_margin + 100));
    let preferred_sizing = child_size.calculate_sizing(available_space);
    assert_eq!(preferred_sizing, expected_sizing);

    // Preferred Size with Less Space
    let available_space = AbsoluteSize::from((width - 100, height - 100));
    let preferred_sizing = child_size.calculate_sizing(available_space);
    assert_eq!(preferred_sizing, expected_sizing);
}

#[test]
fn test_fixed_desired_size_with_padding_and_margin() {
    let margin_top = 20;
    let margin_bottom = 30;
    let margin_left = 40;
    let margin_right = 60;
    let padding_top = margin_top;
    let padding_bottom = margin_bottom;
    let padding_left = margin_left;
    let padding_right = margin_right;
    let width = 500;
    let height = 300;

    let child_size = Sizing {
        margin: Margin {
            top: margin_top.into(),
            right: margin_right.into(),
            left: margin_left.into(),
            bottom: margin_bottom.into(),
        },
        padding: Padding {
            top: padding_top.into(),
            right: padding_right.into(),
            left: padding_left.into(),
            bottom: padding_bottom.into(),
        },
        min_size: None,
        max_size: None,
        size: Size::from((width, height)),
    };

    // Sizing should not change at any point
    let expected_size = Sizing {
        margin: Margin {
            top: margin_top.into(),
            right: margin_right.into(),
            left: margin_left.into(),
            bottom: margin_bottom.into(),
        },
        padding: Padding {
            top: padding_top.into(),
            right: padding_right.into(),
            left: padding_left.into(),
            bottom: padding_bottom.into(),
        },
        min_size: None,
        max_size: None,
        size: Size::from((width, height)),
    };

    let total_height = height + margin_top + margin_bottom + padding_top + padding_bottom;
    let total_width = width + margin_left + margin_right + padding_left + padding_right;

    // Preferred Size with Equal Space (including margin)
    let available_space = AbsoluteSize::from((total_width, total_height));
    let preferred_sizing = child_size.calculate_sizing(available_space);
    assert_eq!(preferred_sizing, expected_size);

    // Preferred Size with Equal Space (excluding margin)
    let available_space = AbsoluteSize::from((width, height));
    let preferred_sizing = child_size.calculate_sizing(available_space);
    assert_eq!(preferred_sizing, expected_size);

    // Preferred Size with Greater Space
    let available_space = AbsoluteSize::from((total_width + 100, total_height + 100));
    let preferred_sizing = child_size.calculate_sizing(available_space);
    assert_eq!(preferred_sizing, expected_size);

    // Preferred Size with Less Space
    let available_space = AbsoluteSize::from((width - 100, height - 100));
    let preferred_sizing = child_size.calculate_sizing(available_space);
    assert_eq!(preferred_sizing, expected_size);
}

#[test]
fn test_percent_desired_size_only() {
    let width = 0.25;
    let height = 0.25;
    let available_width = 511;
    let available_height = 293;

    let child_size = Sizing {
        margin: Default::default(),
        padding: Default::default(),
        min_size: None,
        max_size: None,
        size: Size::from((width, height)),
    };

    // For the first test
    let expected_size = Sizing {
        margin: Default::default(),
        padding: Default::default(),
        min_size: None,
        max_size: None,
        size: Size::from((available_width / 4, available_height / 4)),
    };

    let available_space = AbsoluteSize::from((available_width, available_height));
    let calculated_size = child_size.calculate_sizing(available_space);
    assert_eq!(calculated_size, expected_size);
}