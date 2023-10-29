use std::collections::VecDeque;

use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowBuilder},
};
use winit::event::{Event, WindowEvent};
use winit::platform::pump_events::EventLoopExtPumpEvents;

use crate::backends::OpenGLRenderer;
use crate::layout::Size;
use crate::Renderer;
use crate::widget::color::Color;
use crate::widget::Widget;

pub struct Window
{
    window: WinitWindow,
    event_loop: EventLoop<()>,
    renderer: Box<dyn Renderer>,
    children: Vec<Box<dyn Widget>>,
}

impl Window {
    pub fn set_visible(&self, p0: bool) {
        // No-op
    }
}

impl Window
{
    pub fn new(title: &str, _size: Size) -> Self {
        // Construct the EventLoop for Winit
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);

        let builder = WindowBuilder::new().with_title(title);
        let window = builder.build(&event_loop).unwrap();


        // Determine the available rendering backend
        if let Some(renderer) = Self::resolve_rendering_backend(&window) {
            Window {
                window,
                event_loop,
                renderer,
                children: vec![],
            }
        } else {
            panic!("FUCK")
        }
    }

    pub fn run(&mut self) {
        let timeout = None;
        let mut exit_loop = false;
        let mut event_queue = VecDeque::new();

        while !exit_loop {
            self.event_loop.pump_events(timeout, |event, elwt| {
                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        window_id,
                    } if window_id == self.window.id() => {
                        elwt.exit();
                        exit_loop = true;
                    }

                    _ => event_queue.push_back(event)
                }
            });

            while let Some(event) = event_queue.pop_front() {
                match event {
                    Event::AboutToWait => {
                        self.window.request_redraw();
                    }

                    Event::WindowEvent {
                        event: WindowEvent::RedrawRequested,
                        ..
                    } => {
                        self.redraw();
                    }

                    _ => (),
                }
            }
        }
    }

    fn redraw(&mut self) {
        self.renderer.begin();
        self.renderer.clear_background(Color::new(255, 51, 25, 128));

        for widget in self.children.iter() {
            widget.draw(&mut *self.renderer);
        }
        self.renderer.end();
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.children.push(widget);
    }

    fn resolve_rendering_backend(window: &WinitWindow) -> Option<Box<dyn Renderer>>
    {
        //TODO: Expand this to allow for Metal and DirectX Renderers

        // First check if OpenGL is available
        OpenGLRenderer::try_create(window).map(|renderer| Box::new(renderer) as Box<dyn Renderer>)
    }
}