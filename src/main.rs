#![crate_name = "XBTVEd"]
#![feature(collections, unboxed_closures)]

extern crate piston;

use piston::event::{Event, Events};
use std::cell::RefCell;
use gui::{make_window, draw_ui, XBTVEd};
use std::rc::Rc;

pub mod parse;
pub mod schedule;
pub mod tags;
pub mod blocks;
pub mod gui;

fn main () {
    schedule::test();

    let (window, mut gl, mut ui) = make_window("XBTVEd", 800, 600);
    let window_ref = Rc::new(RefCell::new(window));
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
