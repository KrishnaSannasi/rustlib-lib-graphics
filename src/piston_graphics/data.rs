use std::sync::{Arc, Mutex};
use std::ops::Deref;

use piston::input::Button;
use piston_window::PistonWindow;

pub struct Ptr<T> {
    value: Arc<Mutex<T>>
}

impl<'a, T: 'a> Ptr<T> {
    pub fn defualt() -> Self
    where T: Default {
        Self::create(Arc::new(Mutex::default()))
    }

    pub fn from(t: T) -> Self {
        Self::create(Arc::new(Mutex::from(t)))
    }

    fn create(value: Arc<Mutex<T>>) -> Self {
        Self { value }
    }
}

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        Self::create(self.value.clone())
    }
}

impl<'a, T> Deref for Ptr<T> {
    type Target = Mutex<T>;

    fn deref(&self) -> &Self::Target {
        &*self.value
    }
}

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
