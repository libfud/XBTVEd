#![crate_name = "XBTVEd"]
#![feature(collections, unboxed_closures)]

extern crate conrod;
extern crate piston;
extern crate opengl_graphics;
extern crate glutin_window;

use conrod::{Background, Button, Callable, Color, Colorable, Drawable, 
             Frameable, Label, Labelable, Positionable, Shapeable, Theme, Ui};
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
    schedules: Vec<Schedule>,
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

    pub fn add_schedule(&mut self, sched: &Schedule) {
        self.schedules.push(sched.clone());
    }

    pub fn get_schedule<'a>(&'a self, idx: usize) -> Option<&'a Schedule> {
        self.schedules.get(idx)
    }        
}

pub fn add_schedule() -> Schedule {
    let opengl = OpenGL::_3_2;
    let window = GlutinWindow::new(
        opengl,
        WindowSettings::new(
            "Add Program".to_string(), 
            Size { width: 600, height: 200 }
        ).exit_on_esc(true).samples(4));
    
    let window_ref = Rc::new(RefCell::new(window));
    let mut gl = GlGraphics::new(opengl);

    let font_path = Path::new("./assets/NotoSans-Regular.ttf");
    let theme = Theme::default();
    let glyph_cache = GlyphCache::new(&font_path).unwrap();
    let mut ui = Ui::<GlyphCache>::new(glyph_cache, theme);

    for event in Events::new(window_ref).ups(180).max_fps(60) {
        ui.handle_event(&event);
        if let Event::Render(args) = event {
            gl.draw([0, 0, args.width as i32, args.height as i32], |_, gl| {
                Background::new().color(Color::new(0.8, 0.8, 0.8, 1.0)).draw(&mut ui, gl);
            });
        }
    }

    Schedule::new("New Schedule", 
                  vec!(Program::new(Source::Pathname("foo".to_string()),
                                    Tags::new(),
                                    vec!(Instruction::Play(0, 0)))))
}

pub fn draw_ui(gl: &mut GlGraphics, ui: &mut Ui<GlyphCache>, xbtved: &mut XBTVEd) {
    Background::new().color(xbtved.bg_color).draw(ui, gl);
    
    Button::new(0).dimensions(200.0, 60.0).position(50.0, 50.0).rgba(0.25, 0.25, 0.25, 1.0)
        .frame(xbtved.frame_width).label("Add Schedule")
        .callback(|| {
            let (tx, rx) = channel();
            thread::spawn(move || {
                          tx.send(add_schedule()).unwrap();
                          });
            xbtved.add_schedule(&rx.recv().unwrap());
            println!("{}", xbtved.get_schedule(1).unwrap());
        }).draw(ui, gl);
}

fn main () {
    schedule::test();

    let opengl = OpenGL::_3_2;
    let window = GlutinWindow::new(
        opengl,
        WindowSettings::new(
            "XBTVEd".to_string(), 
            Size { width: 800, height: 600 }
        ).exit_on_esc(true).samples(4));
    
    let window_ref = Rc::new(RefCell::new(window));
    let mut gl = GlGraphics::new(opengl);

    let font_path = Path::new("./assets/NotoSans-Regular.ttf");
    let theme = Theme::default();
    let glyph_cache = GlyphCache::new(&font_path).unwrap();
    let mut ui = Ui::<GlyphCache>::new(glyph_cache, theme);
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
