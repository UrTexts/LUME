use std::io::{self, Write};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use glutin::{ContextBuilder, GlRequest};
use crate::engine::{game::Game, renderer::Renderer};

mod engine;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("PS1 Rust Engine")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let gl_window = ContextBuilder::new()
        .with_gl(GlRequest::Latest)
        .build_windowed(window, &event_loop)
        .unwrap();

    let gl_window = unsafe { gl_window.make_current().unwrap() };
    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    let mut game = Game::new();
    let renderer = Renderer::new();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => game.handle_input(&event), // Handle input
            },
            Event::MainEventsCleared => {
                game.update(1.0 / 60.0); // Update with a delta time (60 FPS)
                renderer.render(); // Render the scene
                gl_window.swap_buffers().unwrap(); // Swap the buffers
            }
            _ => {}
        }
    });
}

