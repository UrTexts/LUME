// src/engine/physics.rs

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn add(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug)]
pub struct RigidBody {
    pub position: Vec3,
    pub velocity: Vec3,
    pub mass: f32,
}

impl RigidBody {
    pub fn new(x: f32, y: f32, z: f32, mass: f32) -> Self {
        RigidBody {
            position: Vec3::new(x, y, z),
            velocity: Vec3::new(0.0, 0.0, 0.0),
            mass,
        }
    }

    pub fn apply_gravity(&mut self, dt: f32) {
        let gravity = Vec3::new(0.0, -9.8, 0.0);
        self.velocity = self.velocity.add(Vec3::new(
            gravity.x * dt,
            gravity.y * dt,
            gravity.z * dt,
        ));
    }

    pub fn update(&mut self, dt: f32) {
        self.apply_gravity(dt);
        self.position = self.position.add(Vec3::new(
            self.velocity.x * dt,
            self.velocity.y * dt,
            self.velocity.z * dt,
        ));

        // Collision with ground
        if self.position.y < 0.0 {
            self.position.y = 0.0;
            self.velocity.y = 0.0;
        }
    }
}

