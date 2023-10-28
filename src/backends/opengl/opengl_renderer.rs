use gl::types::GLfloat;
use raw_gl_context::{GlConfig, GlContext};
use winit::window::Window as WinitWindow;

use crate::widget::color::Color;
use crate::window::Renderer;

pub struct OpenGLRenderer {
    context: GlContext,
}

impl OpenGLRenderer
{
    pub fn try_create(window: &WinitWindow) -> Option<OpenGLRenderer>
    {
        let gl_context = unsafe {
            let potential_context = GlContext::create(window, GlConfig::default());
            if let Ok(context) = potential_context {
                context.make_current();

                gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

                context
            } else {
                return None;
            }
        };

        Some(OpenGLRenderer {
            context: gl_context,
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

    fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: Color) {
        todo!()
    }

    fn clear_background(&mut self, background_color: Color) {
        let red: GLfloat = ((background_color.r as f64) / 255.0) as GLfloat;
        let green: GLfloat = ((background_color.g as f64) / 255.0) as GLfloat;
        let blue: GLfloat = ((background_color.b as f64) / 255.0) as GLfloat;
        let alpha: GLfloat = ((background_color.a as f64) / 255.0) as GLfloat;
        unsafe {
            gl::ClearColor(red, green, blue, alpha);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn begin(&mut self) {
        unsafe {
            self.context.make_current();
        }
    }

    fn end(&mut self) {
        self.context.swap_buffers();
        unsafe {
            self.context.make_not_current();
        }
    }
}
