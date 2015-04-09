#![crate_name = "XBTVEd"]
#![crate_type = "dylib"]
#![feature(convert, collections, unboxed_closures)]

pub mod action;
pub mod parse;
pub mod schedule;
pub mod tags;
pub mod blocks;
pub mod gui;

pub use gui::{XBTVEd, EdBuffer};

#[no_mangle]
pub extern fn times2(num: isize) -> isize {
    num * 2
}
