use gl::types::*;
use std::ffi::CString;
use std::ptr;
use obj::Obj;
use std::fs::File;
use std::io::BufReader;

pub struct Model {
    pub vertices: Vec<f32>,
}

impl Model {
    pub fn load(path: &str) -> Self {
        let input = BufReader::new(File::open(path).unwrap_or_else(|_| panic!("Failed to open file: {}", path)));
        let obj: Obj = obj::load_obj(input).unwrap_or_else(|_| panic!("Failed to parse OBJ file: {}", path));

        let mut vertices = Vec::new();
        for vertex in obj.vertices {
            vertices.push(vertex.position[0]);
            vertices.push(vertex.position[1]);
            vertices.push(vertex.position[2]);
        }

        Model { vertices }
    }
}

pub struct Renderer {
    shader_program: GLuint,
}

impl Renderer {
    pub fn new() -> Self {
        let vertex_shader = Renderer::compile_shader(include_str!("shaders/vertex.glsl"), gl::VERTEX_SHADER);
        let fragment_shader = Renderer::compile_shader(include_str!("shaders/fragment.glsl"), gl::FRAGMENT_SHADER);

        let shader_program = unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            let mut success = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                panic!("Shader linking failed");
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            program
        };

        Renderer { shader_program }
    }

    fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
        let shader = unsafe { gl::CreateShader(shader_type) };
        let c_str = CString::new(source).unwrap();
        unsafe {
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            let mut success = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                panic!("Shader compilation failed");
            }
        }
        shader
    }

    pub fn render(&self) {
        unsafe {
            gl::UseProgram(self.shader_program);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}

