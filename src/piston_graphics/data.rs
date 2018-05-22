use piston::input::Button;
use piston_window::PistonWindow;

pub struct WindowData {
    pub app_data: AppData,
    pub window: PistonWindow
}

pub struct AppData {
    pub is_cursor_on: bool,
    pub is_window_focus: bool,
    pub screen_width: u32,
    pub screen_height: u32,
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub button_held: Vec<Button>
}
