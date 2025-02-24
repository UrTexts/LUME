use crate::engine::{camera::Camera, input::Input};
use crate::engine::physics::{RigidBody, Vec3};
use crate::engine::menu::Menu;

#[derive(PartialEq)] // Derive PartialEq to allow comparisons
pub enum GameState {
    Menu,
    Playing,
}

pub struct Game {
    pub state: GameState,
    pub menu: Menu,
    pub player: RigidBody,
    pub camera: Camera,
    pub input: Input,
}

impl Game {
    pub fn new() -> Self {
        Game {
            state: GameState::Menu,
            menu: Menu::new(),
            player: RigidBody::new(0.0, 5.0, 0.0, 1.0), // Start above ground
            camera: Camera::new(),
            input: Input::new(),
        }
    }

    pub fn handle_input(&mut self, input: &str) {
        match self.state {
            GameState::Menu => match input {
                "up" => self.menu.move_up(),
                "down" => self.menu.move_down(),
                "enter" => {
                    if self.menu.select() == "Start Game" {
                        self.state = GameState::Playing;
                    } else if self.menu.select() == "Exit" {
                        std::process::exit(0);
                    }
                }
                _ => {}
            },
            GameState::Playing => {
                // Handle game input
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.state == GameState::Playing {
            self.player.update(dt);
        }
        self.camera.update(&self.input);
        self.input.reset_frame();
    }

    pub fn render(&self) {
        match self.state {
            GameState::Menu => self.menu.render(),
            GameState::Playing => println!("Game running..."),
        }
    }
}

