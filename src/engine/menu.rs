// src/engine/menu.rs

pub struct Menu {
    options: Vec<String>,
    selected_index: usize,
}

impl Menu {
    pub fn new() -> Self {
        Menu {
            options: vec!["Start Game".to_string(), "Options".to_string(), "Exit".to_string()],
            selected_index: 0,
        }
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_index < self.options.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn select(&self) -> &str {
        &self.options[self.selected_index]
    }

    pub fn render(&self) {
        println!("=== PlayStation-Style Menu ===");
        for (i, option) in self.options.iter().enumerate() {
            if i == self.selected_index {
                println!("> {}", option);
            } else {
                println!("  {}", option);
            }
        }
    }
}

