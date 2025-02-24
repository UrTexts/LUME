// src/engine/math.rs

use nalgebra::{Vector3, Matrix4};

pub fn create_translation_matrix(x: f32, y: f32, z: f32) -> Matrix4<f32> {
    Matrix4::new_translation(&Vector3::new(x, y, z))
}

pub fn create_rotation_matrix(yaw: f32, pitch: f32, roll: f32) -> Matrix4<f32> {
    Matrix4::new_rotation(Vector3::new(yaw, pitch, roll))
}

pub fn create_scale_matrix(scale: f32) -> Matrix4<f32> {
    Matrix4::new_scaling(scale)
}

