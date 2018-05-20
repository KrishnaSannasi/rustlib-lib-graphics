use piston::input::Button;
use piston_window::{Window, PistonWindow};

pub struct Data {
    pub is_cursor_on: bool,
    pub is_window_focus: bool,
    pub screen_width: u32,
    pub screen_height: u32,
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub button_held: Vec<Button>
}

impl Data {
    pub fn new(window: &PistonWindow) -> Data {
        let size = window.size();
        let (screen_width, screen_height) = (size.width, size.height);

        Data {
            is_cursor_on: false,
            is_window_focus: false,
            screen_width, screen_height,
            mouse_x: 0.0,
            mouse_y: 0.0,
            button_held: Vec::new()
        }
    }
}
