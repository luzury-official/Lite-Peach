use macroquad::prelude::*;

pub struct Engine {
    pub is_running: bool,
}

impl Engine {
    pub fn new() -> Self {
        Self { is_running: true }
    }

    pub async fn update(&mut self) {
        if is_key_down(KeyCode::Escape) {
            self.is_running = false;
        }
    }

    pub fn draw(&self) {
        clear_background(Color::new(1.0, 0.75, 0.6, 1.0));
        draw_text("ENGINE ACTIVE", 20.0, 40.0, 30.0, DARKGRAY);
    }
}