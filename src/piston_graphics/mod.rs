use piston_window::{Window, PistonWindow};

use piston::input::*;
use piston::window::WindowSettings;
use opengl_graphics::OpenGL;

pub mod app;
pub mod data;
// pub mod color;

use self::app::{App, DrawType};
use self::data::Data;

impl Data {
    fn new(window: PistonWindow) -> Self {
        let size = window.size();
        let (screen_width, screen_height) = (size.width, size.height);

        Self { 
            is_cursor_on: false,
            is_window_focus: false,
            screen_width, screen_height,
            mouse_x: 0.0,
            mouse_y: 0.0,
            button_held: Vec::new(),
            window
        }
    }
}

pub fn create_window(name: &str, width: u32, height: u32) -> WindowSettings {
    WindowSettings::new(
            name,
            [width, height]
        )
        .opengl(OpenGL::V4_5)
        .exit_on_esc(true)
}

pub fn start<'a, T>(mut window: PistonWindow, mut app: T, draw_type: DrawType)
where T: App {
    // let mut events = Events::new(EventSettings::new());
    let mut _found = false;

    app.init(&mut window);
    let mut data = Data::new(window);
    
    loop {
        let e = data.window.next();

        if let None = e {
            return;
        }

        let e = e.unwrap();

        match e {
            Event::Custom(_a, _b) => {

            },
            Event::Loop(l) => {
                match l {
                    Loop::Render(r) => {
                        app.pre_render(&r, &mut data);
                        match draw_type {
                            DrawType::Dim2 => data.window.draw_2d(&e, |c, g| app.render_2d(&r, c, g)),
                            DrawType::Dim3 => data.window.draw_3d(&e, |w| app.render_3d(&r, w)),
                        };
                    },
                    Loop::Update(u) => {
                        let mut button_held = data.button_held.clone();
                        for button in button_held.drain(..) {
                            match button.button {
                                Button::Keyboard(key) => 
                                    app.handle_key_held(&button, key, &mut data),
                                Button::Mouse(mouse) => 
                                    app.handle_mouse_held(&button, mouse, &mut data),
                                Button::Controller(controller) => 
                                    app.handle_controller_held(&button, controller, &mut data),
                                Button::Hat(hat) => 
                                    app.handle_controller_hat_held(&button, hat, &mut data),
                            }
                        }

                        app.update(&u, &mut data);
                    },
                    Loop::AfterRender(ar) => {
                        app.post_render(&ar, &mut data);
                    },
                    Loop::Idle(i) => {
                        app.idle(&i, &mut data);
                    }
                }
            }
            Event::Input(i) => {
                match i {
                    Input::Button(button) => {
                        let index = data.button_held.iter().position(|x| x.button == button.button);
                        
                        if let None = index {
                            match button.button {
                                Button::Keyboard(key) => 
                                    app.handle_key(&button, key, &mut data),
                                Button::Mouse(mouse) => 
                                    app.handle_mouse(&button, mouse, &mut data),
                                Button::Controller(controller) => 
                                    app.handle_controller(&button, controller, &mut data),
                                Button::Hat(hat) => 
                                    app.handle_controller_hat(&button, hat, &mut data),
                            }
                        }

                        match button.state {
                            ButtonState::Press => {
                                if let None = index {
                                    data.button_held.push(button);
                                    println!("{:?}", data.button_held);
                                } else {
                                    panic!("button is already registered")
                                }
                            },
                            ButtonState::Release => {
                                if let Some(index) = index {
                                    data.button_held.remove(index);
                                } else {
                                    panic!("button is not already registered")
                                }
                            }
                        }
                    },
                    Input::Move(m) => {
                        if let Motion::MouseCursor(x, y) = m {
                            data.mouse_x = x;
                            data.mouse_y = y;
                        }
                        app.handle_mouse_moved(&m, &mut data);
                    },
                    Input::Resize(w, h) => {
                        data.screen_width = w;
                        data.screen_height = h;
                        app.handle_resize(w, h, &mut data);
                    },
                    Input::Text(_t) => { },
                    Input::Cursor(c) => {
                        data.is_cursor_on = c;
                        app.handle_cursor(c, &mut data);
                    },
                    Input::Focus(f) => {
                        data.is_window_focus = f;
                        app.handle_focus(f, &mut data);
                    },
                    Input::Close(c) => {
                        app.on_close(&c, &mut data);
                    }
                }
            }
        }
    }
}
