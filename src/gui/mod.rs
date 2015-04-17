extern crate piston;
extern crate conrod;
extern crate graphics;
extern crate opengl_graphics;
extern crate glutin_window;

pub mod editor;

use std::path::Path;
use std::thread;
use std::sync::mpsc::channel;

pub use self::editor::{EdBuffer, XBTVEd};
use super::schedule::Schedule;
use super::tags::Tags;
use super::program::{Source, Program, Instruction};
use super::action;

use self::conrod::{
    Background,
    Button,
    Callable,
    Color,
    Colorable,
    Drawable,
    DropDownList,
    Frameable,
    Label,
    Labelable,
    Point,
    Positionable,
    Shapeable,
    TextBox,
    Theme,
    Ui
};
use self::conrod::color::{self, rgb, white, black};
use self::opengl_graphics::{GlGraphics, OpenGL};
use self::opengl_graphics::glyph_cache::GlyphCache;
use self::piston::event::*;
use self::piston::window::{WindowSettings, Size};
use self::glutin_window::GlutinWindow;

pub fn make_window(title: &str, width: u32, height: u32) -> (GlutinWindow, GlGraphics, Ui<GlyphCache>) {
    let opengl = OpenGL::_3_2;
    let window = GlutinWindow::new(
        opengl,
        WindowSettings::new(
            title.to_string(),
            Size { width: width, height: height }
            ).exit_on_esc(true).samples(4));
    let gl = GlGraphics::new(opengl);
    let font_path = Path::new("./assets/NotoSans-Regular.ttf");
    let theme = Theme::default();
    let glyph_cache = GlyphCache::new(&font_path).unwrap();
    let ui = Ui::<GlyphCache>::new(glyph_cache, theme);
    (window, gl, ui)
}

pub fn pop_up_msg(msg: &str) {
    let (tx, rx) = channel();
    let mesg = msg.to_string();
    tx.send(mesg).unwrap();;
    thread::spawn(move || {
        let mesg = rx.recv().unwrap();
        let (window, mut gl, mut ui) = make_window("Error", 300, 200);

        let light_bg = rgb(0.8, 0.8, 0.8);
        let mut clicked = false;

        for event in window.events().ups(180).max_fps(60) {
            ui.handle_event(&event);
            if let Some(args) = event.render_args() {
                gl.draw(args.viewport(), |_, gl| {
                    Background::new().color(light_bg.clone()).draw(&mut ui, gl);

                    Label::new(&mesg)
                        .position(5.0, 10.0)
                        .size(16)
                        .color(light_bg.plain_contrast())
                        .draw(&mut ui, gl);
                
                    Button::new(0)
                        .dimensions(100.0, 60.0)
                        .position(5.0, 100.0)
                        .rgba(0.25, 0.25, 0.25, 1.0)
                        .frame(1.0)
                        .label("Ok")
                        .callback(|| {
                            clicked = true;
                        }).draw(&mut ui, gl);

                });
            }

            if clicked {
                break
            }
        }
    });
}

pub fn confirm(msg: &str) -> bool {
    let (msg_tx, msg_rx) = channel();
    let (res_tx, res_rx) = channel();

    let mesg = msg.to_string();
    msg_tx.send(mesg).unwrap();;

    thread::spawn(move || {
        let mesg = msg_rx.recv().unwrap();
        let (window, mut gl, mut ui) = make_window("Confirm", 500, 200);

        let light_bg = rgb(0.8, 0.8, 0.8);
        let mut clicked_cancel = false;
        let mut clicked_ok = false;

        for event in window.events().ups(180).max_fps(60) {
            ui.handle_event(&event);
            if let Some(args) = event.render_args() {
                gl.draw(args.viewport(), |_, gl| {
                    Background::new().color(light_bg.clone()).draw(&mut ui, gl);

                    Label::new(&mesg)
                        .position(5.0, 10.0)
                        .size(16)
                        .color(light_bg.plain_contrast())
                        .draw(&mut ui, gl);
                
                    Button::new(0)
                        .dimensions(100.0, 60.0)
                        .position(5.0, 100.0)
                        .rgba(0.25, 0.25, 0.25, 1.0)
                        .frame(1.0)
                        .label("Confirm")
                        .callback(|| {
                            clicked_ok = true;
                        }).draw(&mut ui, gl);

                    Button::new(1)
                        .dimensions(100.0, 60.0)
                        .position(110.0, 100.0)
                        .rgba(0.25, 0.25, 0.25, 1.0)
                        .frame(1.0)
                        .label("Cancel")
                        .callback(|| {
                            clicked_cancel = true;
                        }).draw(&mut ui, gl);

                });
            }

            if clicked_cancel || clicked_ok {
                break
            }
        }
        if clicked_ok {
            res_tx.send(true).unwrap();
        } else {
            res_tx.send(false).unwrap();
        }
    });
    if let Ok(true) = res_rx.recv() {
        true
    } else {
        false
    }
}    

pub fn change_schedule_name() -> Option<String> {
    let (window, mut gl, mut ui) = make_window("Add Schedule", 600, 200);

    let light_bg = rgb(0.8, 0.8, 0.8);
    let (mut clicked, mut val) = (false, None);
    let ref mut sched_name = "".to_string();

    for event in window.events().ups(180).max_fps(60) {
        ui.handle_event(&event);
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |_, gl| {
                Background::new().color(light_bg.clone()).draw(&mut ui, gl);

                Label::new("Schedule Name").position(5.0, 10.0)
                    .size(18).color(light_bg.plain_contrast()).draw(&mut ui, gl);

                TextBox::new(0, sched_name)
                    .font_size(14)
                    .dimensions(100.0, 20.0)
                    .position(5.0, 35.0)
                    .frame(1.0)
                    .frame_color(light_bg.invert().plain_contrast())
                    .color(light_bg.clone())
                    .callback(|_string: &mut String| {})
                    .draw(&mut ui, gl);

                Button::new(1)
                    .dimensions(100.0, 60.0)
                    .position(5.0, 100.0)
                    .rgba(0.25, 0.25, 0.25, 1.0)
                    .frame(1.0)
                    .label("Ok")
                    .callback(|| {
                        if sched_name.len() == 0 {
                            pop_up_msg("Please give a name for the schedule.")
                        } else { 
                            clicked = true;
                            val = Some(sched_name.clone());
                        }
                    }).draw(&mut ui, gl);

                Button::new(2)
                    .dimensions(100.0, 60.0)
                    .position(110.0, 100.0)
                    .rgba(0.25, 0.25, 0.25, 1.0)
                    .frame(1.0)
                    .label("Cancel")
                    .callback(|| { 
                        clicked = true;
                        val = None;
                    }).draw(&mut ui, gl);
            });
        }
        
        if clicked {
            return val
        }
    }

    None
}

pub fn draw_gui() {
    let (window, mut gl, mut ui) = make_window("XBTVEd", 1000, 800);
    let mut xbtved = XBTVEd::new();

    for event in window.events().ups(180).max_fps(60) {
        ui.handle_event(&event);
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |_, gl| {
                draw_ui(gl, &mut ui, &mut xbtved);
            });
        }
    }
}

pub fn draw_ui<'a>(gl: &mut GlGraphics, ui: &mut Ui<GlyphCache<'a>>, xbtved: &mut XBTVEd) {
    let bg_color = rgb(0.2, 0.21, 0.25);
    let button_color = rgb(0.23, 0.25, 0.29);
    let menu_color = rgb(0.40, 0.44, 0.48);
    let label_color = rgb(0.85, 0.89, 0.95);
    
    Background::new().color(bg_color.clone()).draw(ui, gl);

    //The menubar
    //Starting with File
//    let ref mut idx = None;
    let mut file_entries = xbtved.file_entries_mut().clone();
    let width = xbtved.file_entries_width();
    DropDownList::new(0, &mut file_entries, &mut xbtved.file_selected_idx)
        .dimensions(width, 20.0)
        .position(5.0, 0.0)
        .color(menu_color.clone())
        .frame(1.0)
        .label("File")
        .label_color(label_color.clone())
        .callback(|selected_idx: &mut Option<usize>, new_idx, _string| {
            *selected_idx = Some(new_idx);
        }).draw(ui, gl);

    if let Some(idx) = xbtved.file_selected_idx {
        match idx {
            0 => { }, //File
            1 => xbtved.add_buffer(), //New
            5 => return,
            x => println!("{}", x)
        }
        xbtved.file_selected_idx = None;
    }

    /*
    //Edit
    let ref mut edit_entries = vec!("Undo".to_string(), "Redo".to_string());
    DropDownList::new(0, edit_entries, &mut None)
        .dimensions(45.0, 20.0)
        .position(50.0, 0.0)
        .color(menu_color.clone())
        .frame(1.0)
        .label("Edit")
        .label_color(label_color.clone())
        .callback(|_idx: &mut Option<usize>, new_idx, _string| {
            match new_idx {
                0 => xbtved.current_buffer_mut().undo(),
                _ => { }
            }
        }).draw(ui, gl);
*/

    let buf_len = xbtved.buffers_len().to_string();
    Label::new(&buf_len)
        .position(100.0, 100.0)
        .size(16)
        .color(label_color.clone())
        .draw(ui, gl);

    Label::new(&xbtved.current_buffer().get_schedule().to_string())
        .position(10.0, 150.0)
        .size(16)
        .color(label_color.clone())
        .draw(ui, gl);

    Button::new(100)
        .dimensions(200.0, 40.0)
        .position(50.0, 50.0)
        .color(button_color.clone())
        .frame(1.0)
        .label("Change Schedule name")
        .callback(|| {
            let (tx, rx) = channel();
            thread::spawn(move || {
                          tx.send(change_schedule_name()).unwrap();
                          });
            if let Ok(Some(name)) = rx.recv() {
                let namechange = action::SetName::new(xbtved.current_buffer_mut(), &name);
                match xbtved.current_buffer_mut().apply(namechange) {
                    Ok(_) => {},
                    Err(ref f) => pop_up_msg(&f)
                }
            };
        }).draw(ui, gl);

    Button::new(101)
        .dimensions(200.0, 40.0)
        .position(260.0, 50.0)
        .color(button_color.clone())
        .frame(1.0)
        .label("Previous Schedule")
        .callback(|| {
            xbtved.prev_buffer();
        }).draw(ui, gl);

    Button::new(102)
        .dimensions(200.0, 40.0)
        .position(480.0, 50.0)
        .color(button_color.clone())
        .frame(1.0)
        .label("Next Schedule")
        .callback(|| {
            xbtved.next_buffer();
        }).draw(ui, gl);

    Button::new(103)
        .dimensions(100.0, 40.0)
        .position(700.0, 50.0)
        .color(button_color.clone())
        .frame(1.0)
        .label("Undo")
        .callback(|| {
            xbtved.current_buffer_mut().undo();
        }).draw(ui, gl);

    Button::new(104)
        .dimensions(100.0, 40.0)
        .position(820.0, 50.0)
        .color(button_color.clone())
        .frame(1.0)
        .label("Redo")
        .callback(|| {
            xbtved.current_buffer_mut().redo();
        }).draw(ui, gl);
}
