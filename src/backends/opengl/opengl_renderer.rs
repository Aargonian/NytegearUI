use crate::window::Renderer;
use raw_gl_context::{GlConfig, GlContext};
use winit::window::Window as WinitWindow;
use winit::raw_window_handle::HasRawWindowHandle;

pub struct OpenGLRenderer {
    context: GlContext,
}

impl OpenGLRenderer
{
    pub fn try_create(window: &WinitWindow) -> Option<OpenGLRenderer>
    {
        let context = unsafe { GlContext::create(window, GlConfig::default()).unwrap() };

        unsafe {
            context.make_current();
        }

        Some(OpenGLRenderer {
            context,
        })
    }
}

impl Renderer for OpenGLRenderer
{
    fn draw_pixel(&mut self, x: u32, y: u32, value: u32) {
        todo!()
    }

    fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, value: u32) {
        todo!()
    }

    fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32, value: u32) {
        todo!()
    }
}
