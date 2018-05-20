pub extern crate piston;
pub extern crate piston_window;
pub extern crate graphics;
pub extern crate image;
pub extern crate imageproc;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate gfx_graphics;
extern crate gfx_device_gl;

use std::sync::atomic::{AtomicPtr, Ordering};

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
