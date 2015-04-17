#![crate_name = "XBTVEd"]
#![crate_type = "dylib"]
#![feature(core, convert, collections, unboxed_closures)]
#[deny(warnings)]

pub mod action;
pub mod parse;
pub mod schedule;
pub mod program;
pub mod tags;
pub mod blocks;
pub mod gui;

pub use gui::EdBuffer;

fn main() {
    gui::draw_gui();
}
