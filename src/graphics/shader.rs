// src/graphics/shader.rs

use std::fs;
use std::ptr;
use gl::types::*;

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        let vertex_code = fs::read_to_string(vertex_path).expect("Unable to read vertex shader");
        let fragment_code = fs::read_to_string(fragment_path).expect("Unable to read fragment shader");

        let vertex_shader = Shader::compile_shader(&vertex_code, gl::VERTEX_SHADER);
        let fragment_shader = Shader::compile_shader(&fragment_code, gl::FRAGMENT_SHADER);

        let program = Shader::link_program(vertex_shader, fragment_shader);

        Shader { id: program }
    }

    fn compile_shader(source: &str, shader_type: GLuint) -> GLuint {
        let shader = unsafe { gl::CreateShader(shader_type) };
        let c_str = std::ffi::CString::new(source.as_bytes()).unwrap();
        unsafe {
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);
        }
        shader
    }

    fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
        let program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
        }
        program
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

