#![crate_name = "XBTVEd"]
#![feature(collections, unboxed_closures)]

extern crate conrod;
extern crate piston;
extern crate opengl_graphics;
extern crate glutin_window;

use conrod::{Background, 
             Button, 
             Callable, 
             Color, 
             Colorable, 
             Drawable, 
             Frameable, 
             Label, 
             Labelable, 
             Point, 
             Positionable, 
             Shapeable, 
             TextBox, 
             Theme, 
             Ui,
             WidgetMatrix
};
use opengl_graphics::{GlGraphics, OpenGL};
use opengl_graphics::glyph_cache::GlyphCache;
use glutin_window::GlutinWindow;
use piston::window::{WindowSettings, Size};
use piston::event::{Event, Events};
use std::sync::mpsc::channel;
use std::thread;
use std::rc::Rc;
use std::cell::RefCell;
use std::convert::AsRef;
use std::path::Path;
use schedule::{Schedule, Source, Program, Instruction};
use tags::Tags;

pub mod parse;
pub mod schedule;
pub mod tags;
pub mod blocks;

pub struct XBTVEd {
    pub schedules: Vec<Schedule>,
    bg_color: Color,
    frame_width: f64,
    current_schedule: usize
}

impl XBTVEd {
    pub fn new() -> XBTVEd {
        XBTVEd {
            schedules: vec!(Schedule::new("Example", 
                                          vec!(Program::new(Source::Pathname("example.schedule".to_string()),
                                                            Tags::new(),
                                                            vec!(Instruction::Play(0, 0)))))),
            bg_color: Color::new(0.2, 0.2, 0.2, 1.0),
            frame_width: 1.0,
            current_schedule: 0
        }
    }

    pub fn get_schedule<'a>(&'a self, idx: usize) -> Option<&'a Schedule> {
        self.schedules.get(idx)
    }

    pub fn prev_schedule(&mut self) {
        if self.current_schedule == 0 {
            ()
        } else {
            self.current_schedule -= 1
        }
    }

    pub fn next_schedule(&mut self) {
        if self.current_schedule + 1 == self.schedules.len() {
            ()
        } else {
            self.current_schedule += 1
        }
    }

    pub fn change_current_schedule(&mut self, idx: usize) -> Result<(), String> {
        if idx >= self.schedules.len() {
            Err("Out of bounds".to_string())
        } else {
            self.current_schedule = idx;
            Ok(())
        }
    }
}

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
    

pub fn add_schedule() -> Option<Schedule> {
    let (window, mut gl, mut ui) = make_window("Add Schedule", 600, 200);
    let window_ref = Rc::new(RefCell::new(window));

    let light_bg = Color::new(0.8, 0.8, 0.8, 1.0);

    let ref mut sched_name = "name".to_string();

    for event in Events::new(window_ref).ups(180).max_fps(60) {
        let (mut clicked, mut val) = (false, None);

        ui.handle_event(&event);
        if let Event::Render(args) = event {
            gl.draw([0, 0, args.width as i32, args.height as i32], |_, gl| {
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
                    .dimensions(50.0, 60.0)
                    .position(5.0, 100.0)
                    .rgba(0.25, 0.25, 0.25, 1.0)
                    .frame(1.0)
                    .label("Ok")
                    .callback(|| {
                        let tags = Tags::new();
                        let instrs = vec!(Instruction::Play(0, 0));
                        let program = Program::new(Source::Pathname("foo".to_string()), tags, instrs);

                        clicked = true;
                        val = Some(Schedule::new(&sched_name, vec!(program)));
                    }).draw(&mut ui, gl);

                Button::new(2)
                    .dimensions(50.0, 60.0)
                    .position(60.0, 100.0)
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

pub fn draw_ui(gl: &mut GlGraphics, ui: &mut Ui<GlyphCache>, xbtved: &mut XBTVEd) {
    let add_sched_uiid = 10;
    let buff_display_uiid = 144;
    
    let ref mut buffer = xbtved.get_schedule(xbtved.current_schedule).unwrap().to_string();

    Background::new().color(xbtved.bg_color).draw(ui, gl);
    
    Button::new(add_sched_uiid)
        .dimensions(200.0, 60.0)
        .position(50.0, 50.0)
        .rgba(0.25, 0.25, 0.25, 1.0)
        .frame(xbtved.frame_width)
        .label("Add Schedule")
        .callback(|| {
            let (tx, rx) = channel();
            thread::spawn(move || {
                          tx.send(add_schedule()).unwrap();
                          });
            if let Some(sched) = rx.recv().unwrap() {
                xbtved.schedules.push(sched)
            };
            println!("{}", xbtved.get_schedule(1).unwrap());
        }).draw(ui, gl);

    TextBox::new(buff_display_uiid, buffer)
        .font_size(16)
        .dimensions(500.0, 400.0)
        .position(50.0, 120.0)
        .frame(1.0)
        .frame_color(xbtved.bg_color.invert().plain_contrast())
        .color(xbtved.bg_color.invert())
        .callback(|_string: &mut String| {})
        .draw(ui, gl);
        
}

fn main () {
    schedule::test();

    let (window, mut gl, mut ui) = make_window("XBTVEd", 800, 600);

//    let opengl = OpenGL::_3_2;
//    let window = GlutinWindow::new(
//        opengl,
//        WindowSettings::new(
//            "XBTVEd".to_string(), 
//            Size { width: 800, height: 600 }
//        ).exit_on_esc(true).samples(4));
    
    let window_ref = Rc::new(RefCell::new(window));
//    let mut gl = GlGraphics::new(opengl);

//    let font_path = Path::new("./assets/NotoSans-Regular.ttf");
//    let theme = Theme::default();
//    let glyph_cache = GlyphCache::new(&font_path).unwrap();
//    let mut ui = Ui::<GlyphCache>::new(glyph_cache, theme);
    let mut xbtved = XBTVEd::new();
    
    for event in Events::new(window_ref).ups(180).max_fps(60) {
        ui.handle_event(&event);
        if let Event::Render(args) = event {
            gl.draw([0, 0, args.width as i32, args.height as i32], |_, gl| {
                draw_ui(gl, &mut ui, &mut xbtved);
            });
        }
    }
}
