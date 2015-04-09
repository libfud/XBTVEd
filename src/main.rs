#![crate_name = "XBTVEd"]
#![feature(convert, collections, unboxed_closures)]

pub mod action;
pub mod parse;
pub mod schedule;
pub mod tags;
pub mod blocks;
pub mod gui;

pub use gui::{XBTVEd, EdBuffer};

#[cfg(test)]
mod test;

fn main () {
    let mut xbtved = XBTVEd::new();

    gui::draw_ui(&mut xbtved);
}
