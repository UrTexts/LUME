use std::collections::HashSet;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, MouseButton, MouseScrollDelta, WindowEvent};

pub struct Input {
    keys_pressed: HashSet<VirtualKeyCode>,
    mouse_delta: (f32, f32),
    scroll_delta: f32,
    left_mouse_pressed: bool,
}

impl Input {
    pub fn new() -> Self {
        Input {
            keys_pressed: HashSet::new(),
            mouse_delta: (0.0, 0.0),
            scroll_delta: 0.0,
            left_mouse_pressed: false,
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(keycode) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => {
                            self.keys_pressed.insert(keycode);
                        }
                        ElementState::Released => {
                            self.keys_pressed.remove(&keycode);
                        }
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if *button == MouseButton::Left {
                    self.left_mouse_pressed = *state == ElementState::Pressed;
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_delta = (position.x as f32, position.y as f32);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                if let MouseScrollDelta::LineDelta(_, y) = delta {
                    self.scroll_delta = *y;
                }
            }
            _ => {}
        }
    }

    pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn reset_frame(&mut self) {
        self.mouse_delta = (0.0, 0.0);
        self.scroll_delta = 0.0;
    }
}

