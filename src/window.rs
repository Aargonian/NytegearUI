use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowBuilder},
};

use crate::backends::OpenGLRenderer;

pub trait Renderer
{
    fn draw_pixel(&mut self, x: u32, y: u32, value: u32);
    fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, value: u32);
    fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32, value: u32);
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

    fn resolve_rendering_backend(window: &WinitWindow) -> Option<Box<dyn Renderer>>
    {
        //TODO: Expand this to allow for Metal and DirectX Renderers

        // First check if OpenGL is available
        OpenGLRenderer::try_create(window).map(|renderer| Box::new(renderer) as Box<dyn Renderer>)
    }
}