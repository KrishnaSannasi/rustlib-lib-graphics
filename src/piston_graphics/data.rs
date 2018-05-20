use std::sync::atomic::{AtomicPtr, Ordering};
use std::ops::{Deref, DerefMut};

use piston::input::Button;
use piston_window::PistonWindow;

enum PtrVal<T> {
    Null,
    Ptr(AtomicPtr<T>)
}

pub struct Ptr<T> {
    value: PtrVal<T>
}

impl<T> Ptr<T> {
    pub fn null() -> Self {
        Self { value: PtrVal::Null }
    }

    pub fn from(t: &mut T) -> Self {
        Self { value: PtrVal::Ptr(AtomicPtr::new(t)) }
    }

    pub fn is_null(&self) -> bool {
        match &self.value {
            &PtrVal::Null => true,
            _ => false
        }
    }
}

impl<T> Deref for Ptr<T> {
    type Target = T;

    fn deref(&self) -> &T {
        match &self.value {
            &PtrVal::Ptr(ref ptr) =>
                unsafe {
                    match ptr.load(Ordering::Relaxed).as_ref() {
                        Some(val) => val,
                        None => panic!("tried to deref a null ptr (unsafe)")
                    }
                },
            &PtrVal::Null => panic!("tried to deref a null ptr (safe)")
        }
    }
}

impl<T> DerefMut for Ptr<T> {
    fn deref_mut(&mut self) -> &mut T {
        match &self.value {
            &PtrVal::Ptr(ref ptr) =>
                unsafe {
                    match ptr.load(Ordering::Relaxed).as_mut() {
                        Some(val) => val,
                        None => panic!("tried to deref a null ptr (unsafe)")
                    }
                },
            &PtrVal::Null => panic!("tried to deref a null ptr (safe)")
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
