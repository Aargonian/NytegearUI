extern crate gl;

use std::ffi::CString;
use std::ptr;

pub struct Shader {
    program: gl::types::GLuint,
}

impl Shader {
    pub fn new(vertex_src: &str, fragment_src: &str) -> Result<Self, String> {
        let vertex_shader = Self::compile_shader(vertex_src, gl::VERTEX_SHADER)?;
        let fragment_shader = Self::compile_shader(fragment_src, gl::FRAGMENT_SHADER)?;
        let program = Self::link_program(vertex_shader, fragment_shader)?;

        Ok(Shader { program })
    }

    fn compile_shader(src: &str, ty: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
        let shader;
        unsafe {
            shader = gl::CreateShader(ty);
            let c_str = CString::new(src.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            let mut status = gl::FALSE as gl::types::GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            if status != (gl::TRUE as gl::types::GLint) {
                return Err(Self::get_shader_error(shader));
            }
        }
        Ok(shader)
    }

    fn link_program(vs: gl::types::GLuint, fs: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
        let program;
        unsafe {
            program = gl::CreateProgram();
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);

            let mut status = gl::FALSE as gl::types::GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

            if status != (gl::TRUE as gl::types::GLint) {
                return Err(Self::get_program_error(program));
            }

            gl::DeleteShader(vs);
            gl::DeleteShader(fs);
        }
        Ok(program)
    }

    fn get_shader_error(shader: gl::types::GLuint) -> String {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        }
        let error = Self::create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
        }

        error.to_string_lossy().into_owned()
    }

    fn get_program_error(program: gl::types::GLuint) -> String {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
        }
        let error = Self::create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
        }

        error.to_string_lossy().into_owned()
    }

    fn create_whitespace_cstring_with_len(len: usize) -> CString {
        let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
        buffer.extend([b' '].iter().cycle().take(len));
        unsafe { CString::from_vec_unchecked(buffer) }
    }

    pub fn program(&self) -> gl::types::GLuint {
        self.program
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
}