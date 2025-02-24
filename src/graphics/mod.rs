// src/graphics/renderer.rs

use glutin::{ContextBuilder, event::{Event, WindowEvent}, window::WindowBuilder, platform::windows::WindowBuilderExt};
use gl::{self, types::*};
use crate::graphics::shader::Shader;

pub struct Renderer {
    shader: Shader,
}

impl Renderer {
    pub fn new() -> Self {
        // Initialize the OpenGL context using Glutin
        let event_loop = glutin::event_loop::EventLoop::new();
        let window = WindowBuilder::new().with_title("PS1 Style Engine").build(&event_loop).unwrap();
        let context = ContextBuilder::new().build_windowed(window, &event_loop).unwrap();
        unsafe {
            context.make_current().unwrap();
            gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);
        }

        // Create the shader and load it
        let shader = Shader::new("assets/shaders/vertex.glsl", "assets/shaders/fragment.glsl");

        // Return the Renderer
        Renderer { shader }
    }

    pub fn render(&self) {
        // Rendering logic (clear screen, draw objects, etc.)
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.shader.use_program();  // Use shader for rendering
            // Further OpenGL rendering would go here
        }
    }
}

