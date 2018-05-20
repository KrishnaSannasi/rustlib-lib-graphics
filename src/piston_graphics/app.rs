use piston::input::*;
use piston_window::{Context};
use gfx_graphics::GfxGraphics;
use gfx_device_gl::{Resources, CommandBuffer};

use super::data::{Ptr, WindowData};

pub type AppGraphics<'a> = GfxGraphics<'a, Resources, CommandBuffer>;

pub trait App {
    fn render(&self, _args: Context, _gl: &mut AppGraphics);
    fn update(&mut self, _args: &UpdateArgs);
    fn set_window_data(&mut self, _data: Ptr<WindowData>);

    // called after rendering
    fn post_render(&self, _args: &AfterRenderArgs) {}

    fn idle(&self, _args: &IdleArgs) {}

    fn handle_key(&mut self, _key: Key) {}
    
    fn handle_mouse(&mut self, _mouse_button: MouseButton, _mouse_x: f64, _mouse_y: f64) {}

    fn handle_controller(&mut self, _controller_button: ControllerButton) {}
    
    fn handle_key_held(&mut self, _key: Key) {}
    
    fn handle_mouse_held(&mut self, _mouse_button: MouseButton) {}

    fn handle_controller_held(&mut self, _controller_button: ControllerButton) {}

    // handle mouse movement
    fn mouse_moved(&mut self, _args: &Motion) {}
    
    // handle cursor going on and off screen
    fn handle_cursor(&mut self, _cursor: bool) {}

    // handle window focus going on and off
    fn handle_focus(&mut self, _focus: bool) {}

    // handle window resizing
    fn handle_resize(&mut self, _width: u32, _height: u32) {}

    fn on_close(&mut self, _args: &CloseArgs) {}
}