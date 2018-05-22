use piston::input::*;
use piston_window::{PistonWindow, Context};
use gfx_graphics::GfxGraphics;
use gfx_device_gl::{Resources, CommandBuffer};

use super::data::WindowData;

pub type AppGraphics<'a> = GfxGraphics<'a, Resources, CommandBuffer>;

/// What dimensions are you drawing to
pub enum DrawType {
    /// two dimensions
    Dim2, 
    /// three dimensions
    Dim3
}

#[allow(unused_variables)]
pub trait App {
    /// renders things to the screen for 2d graphics
    fn render_2d(&self, args: &RenderArgs, c: Context, gl: &mut AppGraphics) {}

    /// renders things to the screen for 3d graphics
    fn render_3d(&self, args: &RenderArgs, window: &mut PistonWindow) {}

    //. should be used to do calculations and more
    fn update(&mut self, args: &UpdateArgs, data: &WindowData) {}
    
    /// called right before render should be used to do setup before rendering
    /// this is to be used to do renderargs specific adjustments
    fn pre_render(&mut self, args: &RenderArgs, data: &WindowData) {}

    /// called after rendering
    fn post_render(&mut self, args: &AfterRenderArgs, data: &WindowData) {}

    /// handles idle time
    fn idle(&mut self, args: &IdleArgs, data: &WindowData) {}

    /// handles a keyboard key being presesd down (only for the first tick, not while it is held down)
    fn handle_key(&mut self, key: Key, data: &WindowData) {}
    
    /// handles a mouse button being presesd down (only for the first tick, not while it is held down)
    fn handle_mouse(&mut self, mouse_button: MouseButton, data: &WindowData) {}

    /// handles a controller button being presesd down (only for the first tick, not while it is held down)
    fn handle_controller(&mut self, controller_button: ControllerButton, data: &WindowData) {}
    
    /// handles a keyboard keys being held down
    fn handle_key_held(&mut self, key: Key, data: &WindowData) {}
    
    /// handles a mouse button being held down
    fn handle_mouse_held(&mut self, mouse_button: MouseButton, data: &WindowData) {}

    /// handle controller button being held down
    fn handle_controller_held(&mut self, controller_button: ControllerButton, data: &WindowData) {}

    /// handle mouse movement
    fn mouse_moved(&mut self, args: &Motion, data: &WindowData) {}
    
    /// handle cursor going on and off screen
    fn handle_cursor(&mut self, cursor: bool, data: &WindowData) {}

    /// handle window focus going on and off
    fn handle_focus(&mut self, focus: bool, data: &WindowData) {}

    /// handle window resizing
    fn handle_resize(&mut self, width: u32, height: u32, data: &WindowData) {}

    /// handles closing the window
    fn on_close(&mut self, args: &CloseArgs, data: &WindowData) {}
}