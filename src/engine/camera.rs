use crate::engine::input::Input;
use nalgebra::{Vector3, Matrix4};

pub struct Camera {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>, // (yaw, pitch, roll)
    pub speed: f32,
    pub sensitivity: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vector3::new(0.0, 0.0, 3.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            speed: 0.1,
            sensitivity: 0.002,
        }
    }

    pub fn update(&mut self, input: &Input) {
        let forward = Vector3::new(
            self.rotation.y.sin(),
            0.0,
            self.rotation.y.cos(),
        );

        let right = Vector3::new(
            self.rotation.y.cos(),
            0.0,
            -self.rotation.y.sin(),
        );

        // Movement
        if input.is_key_pressed(winit::event::VirtualKeyCode::W) {
            self.position += forward * self.speed;
        }
        if input.is_key_pressed(winit::event::VirtualKeyCode::S) {
            self.position -= forward * self.speed;
        }
        if input.is_key_pressed(winit::event::VirtualKeyCode::A) {
            self.position -= right * self.speed;
        }
        if input.is_key_pressed(winit::event::VirtualKeyCode::D) {
            self.position += right * self.speed;
        }

        // Mouse rotation
        let (dx, dy) = input.mouse_delta;
        self.rotation.y += dx * self.sensitivity;
        self.rotation.x = (self.rotation.x - dy * self.sensitivity)
            .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::new_translation(&self.position) *
        Matrix4::from_euler_angles(self.rotation.x, self.rotation.y, self.rotation.z)
    }
}

