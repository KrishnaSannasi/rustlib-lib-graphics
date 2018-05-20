use std::sync::atomic::{AtomicPtr, Ordering};
use std::ops::{Deref, DerefMut};

use piston::input::Button;
use piston_window::PistonWindow;

pub struct Ptr<T> {
    value: AtomicPtr<T>
}

impl<T> Ptr<T> {
    pub fn new(t: &mut T) -> Self {
        Self { value: AtomicPtr::new(t) }
    }
}

impl<T> Deref for Ptr<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            match self.value.load(Ordering::Relaxed).as_ref() {
                Some(val) => val,
                None => panic!("tried to deref a null ptr")
            }
        }
    }
}

impl<T> DerefMut for Ptr<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            match self.value.load(Ordering::Relaxed).as_mut() {
                Some(val) => val,
                None => panic!("tried to deref a null ptr")
            }
        }
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
