pub extern crate piston;
pub extern crate piston_window;
pub extern crate graphics;
pub extern crate image;
pub extern crate imageproc;
pub extern crate num;
pub extern crate glutin_window;
pub extern crate opengl_graphics;
pub extern crate gfx_graphics;
pub extern crate gfx_device_gl;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod piston_graphics;
