//TODO: Remove this
#![allow(dead_code)]
#![allow(unused_variables)]

use winit::dpi::Position;
use nytegearui::widget::color::Color;
use nytegearui::widget::style::Style;
use nytegearui::widget::{Renderer, Widget};

pub struct RectangleWidget {
    position: Position,
    sizing: String,
    style: Style,
    color: Color,
}

impl RectangleWidget {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            position: Default::default(),
            sizing: Sizing {
                size: (width, height).into(),
                ..Default::default()
            },
            style: Default::default(),
            color: Default::default(),
        }
    }
}

impl Widget for RectangleWidget {
    fn draw(&self, renderer: &mut Box<dyn Renderer>) {
        renderer.fill_rect(self.layout.position.x,
                           self.layout.position.y,
                           250,
                           250,
                           self.style.foreground_color);
    }
}

fn main() {
    let mut window = Window::new("Test Window", (500, 500).into());

    let mut rect_widget = RectangleWidget::new(50, 50);
    rect_widget.position = (250, 250).into();
    rect_widget.sizing = Sizing {
        size: (50, 50).into(),
        ..Default::default()
    };
    rect_widget.color = Color::new(255, 255, 0, 0);

    window.add_widget(Box::new(rect_widget));
    window.set_visible(true);
    window.run();
}