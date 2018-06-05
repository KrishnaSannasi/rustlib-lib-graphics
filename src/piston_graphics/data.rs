use piston::input::Button;
use piston_window::PistonWindow;

pub struct Data {
    pub window: PistonWindow,
    pub is_cursor_on: bool,
    pub is_window_focus: bool,
    pub screen_width: u32,
    pub screen_height: u32,
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub button_held: Vec<Button>
}
