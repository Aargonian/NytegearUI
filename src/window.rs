use std::collections::VecDeque;

use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowBuilder},
};
use winit::event::{Event, WindowEvent};
use winit::platform::pump_events::EventLoopExtPumpEvents;

use crate::backends::OpenGLRenderer;
use crate::widget::color::Color;

pub trait Renderer
{
    fn draw_pixel(&mut self, x: u32, y: u32, value: u32);
    fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, value: u32);
    fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: Color);
    fn clear_background(&mut self, background_color: Color);
    fn begin(&mut self);
    fn end(&mut self);
}

pub trait Widget
{
    fn draw(&self, renderer: &mut dyn Renderer);
}

pub struct Window
{
    window: WinitWindow,
    event_loop: EventLoop<()>,
    renderer: Box<dyn Renderer>,
    children: Vec<Box<dyn Widget>>,
}

impl Window
{
    pub fn new() -> Self {
        // Construct the EventLoop for Winit
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);

        let window = WindowBuilder::new().build(&event_loop).unwrap();


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
        self.renderer.end();
    }

    fn resolve_rendering_backend(window: &WinitWindow) -> Option<Box<dyn Renderer>>
    {
        //TODO: Expand this to allow for Metal and DirectX Renderers

        // First check if OpenGL is available
        OpenGLRenderer::try_create(window).map(|renderer| Box::new(renderer) as Box<dyn Renderer>)
    }
}