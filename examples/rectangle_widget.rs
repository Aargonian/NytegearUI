//TODO: Remove this
#![allow(dead_code)]
#![allow(unused_variables)]

use nytegearui::widget::{
    Position, Sizing,
    style::Style,
};
use nytegearui::window::{Renderer, Widget};
use nytegearui::window::Window;

pub struct RectangleWidget {
    position: Position,
    sizing: Sizing,
    style: Style,
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
        }
    }
}

impl Widget for RectangleWidget {
    fn draw(&self, renderer: &mut dyn Renderer) {
        //renderer.set_style();
        /*
        renderer.fill_rect(self.position.x,
                           self.position.y,
                           self.sizing.size.width,
                           self.sizing.size.height,
                           self.style.foreground_color);

         */
    }
}

fn main() {
    let window = Window::new();

    let rect_widget = RectangleWidget::new(50, 50);
    /*
    event_loop.run(move |event, window_target| {

        match event {
            Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                window_target.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                unsafe {
                    context.make_current();
                }

                unsafe {
                    gl::ClearColor(0.2, 0.1, 0.5, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }

                context.swap_buffers();

                unsafe {
                    context.make_not_current();
                }
            }
            _ => {}
        }
    }).expect("TODO: panic message");
    */
}