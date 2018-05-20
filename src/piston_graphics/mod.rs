use super::{unwrap, unwrap_mut};

use std::sync::atomic::AtomicPtr;
use piston_window::PistonWindow;

use piston::input::*;

pub mod app;
pub mod data;

use self::app::App;
use self::data::Data;

pub fn start<T>(mut window: PistonWindow, mut app: T)
    where T: App {
    // let mut events = Events::new(EventSettings::new());
    let mut _found = false;

    let mut data = Data::new(&window);

    app.set_data(AtomicPtr::new(&mut data));
    app.set_window(AtomicPtr::new(&mut window));

    let mut data = AtomicPtr::new(&mut data);
    let mut window = AtomicPtr::new(&mut window);

    loop {
        let e = unwrap_mut(&mut window).next();

        if let None = e {
            break;
        }

        let e = e.unwrap();

        match e {
            Event::Custom(_a, _b) => {

            },
            Event::Loop(l) => {
                match l {
                    Loop::Render(_r) => {
                        unwrap_mut(&mut window).draw_2d(&e, |c, g| app.render(c, g));
                    },
                    Loop::Update(u) => {
                        let d = unwrap(&data);

                        for button in &d.button_held {
                            match button {
                                &Button::Keyboard(key) => 
                                    app.handle_key_held(key),
                                &Button::Mouse(mouse_button) => 
                                    app.handle_mouse_held(mouse_button),
                                &Button::Controller(controller_button) => 
                                    app.handle_controller_held(controller_button)
                            }
                        }

                        app.update(&u);
                    },
                    Loop::AfterRender(ar) => {
                        app.post_render(&ar);
                    },
                    Loop::Idle(i) => {
                        // println!("idle time {:?}ms", _a.dt * 1000.0);
                        app.idle(&i);
                    }
                }
            }
            Event::Input(i) => {
                match i {
                    Input::Button(b) => {
                        let d = unwrap_mut(&mut data);
                        let contains = d.button_held.contains(&b.button);
                        
                        if !contains {
                            match b.button {
                                Button::Keyboard(key) => 
                                    app.handle_key(key),
                                Button::Mouse(mouse_button) => 
                                    app.handle_mouse(mouse_button, d.mouse_x, d.mouse_y),
                                Button::Controller(controller_button) => 
                                    app.handle_controller(controller_button)
                            }
                        }

                        match b.state {
                            ButtonState::Press => {
                                if !contains {
                                    d.button_held.push(b.button);
                                }
                            },
                            ButtonState::Release => {
                                if contains {
                                    let index = d.button_held.iter().position(|x| *x == b.button).unwrap();
                                    d.button_held.remove(index);
                                }
                            }
                        }
                    },
                    Input::Move(m) => {
                        let d = unwrap_mut(&mut data);
                        if let Motion::MouseCursor(x, y) = m {
                            d.mouse_x = x;
                            d.mouse_y = y;
                        }
                    },
                    Input::Resize(w, h) => {
                        let d = unwrap_mut(&mut data);

                        d.screen_width = w;
                        d.screen_height = h;
                    },
                    Input::Text(_t) => {

                    },
                    Input::Cursor(c) => {
                        let d = unwrap_mut(&mut data);

                        d.is_cursor_on = c;
                        app.handle_cursor(c);
                    },
                    Input::Focus(f) => {
                        let d = unwrap_mut(&mut data);

                        d.is_window_focus = f;
                        app.handle_focus(f);
                    },
                    Input::Close(c) => {
                        app.on_close(&c);
                    }
                }
            }
        }
    }
}
