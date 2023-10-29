use gl::types::GLfloat;
use raw_gl_context::{GlConfig, GlContext};
use winit::window::Window as WinitWindow;

use crate::backends::opengl::shader::Shader;
use crate::Renderer;
use crate::widget::Color;

pub struct OpenGLRenderer {
    context: GlContext,
    shader: Shader,
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

        let vertex_src = include_str!("shaders/ui_basic.vert");
        let fragment_src = include_str!("shaders/ui_basic.frag");

        // Create shader program
        let shader = match Shader::new(vertex_src, fragment_src) {
            Ok(shader) => shader,
            Err(e) => {
                eprintln!("Shader creation failed: {}", e);
                return None;
            }
        };

        let renderer = OpenGLRenderer {
            context: gl_context,
            shader,  // Include shader in your struct
        };

        Some(renderer)
    }

    fn begin(&mut self) {
        unsafe {
            self.context.make_current();
            gl::UseProgram(self.shader.program());
        }
    }

    fn end(&mut self) {
        // Do Nothing for now
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

    fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: Color) {}

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
