extern crate piston;
extern crate piston_window;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate gfx_graphics;
extern crate gfx_device_gl;
extern crate image;
extern crate imageproc;

use std::sync::atomic::{AtomicPtr, Ordering};

pub use piston::input::*;
pub use piston_window::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod piston_graphics;

pub fn unwrap<T>(t: &AtomicPtr<T>) -> &T {
    unsafe {
        &*t.load(Ordering::Relaxed)
    }
}

pub fn unwrap_mut<T>(t: &mut AtomicPtr<T>) -> &mut T {
    unsafe {
        &mut *t.load(Ordering::Relaxed)
    }
}
