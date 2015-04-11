#![crate_name = "XBTVEd"]
#![crate_type = "dylib"]
#![feature(libc, convert, collections, unboxed_closures)]

extern crate libc;

use std::ffi::CString;
use std::mem;

pub mod action;
pub mod parse;
pub mod schedule;
pub mod program;
pub mod tags;
pub mod blocks;
pub mod gui;

pub use gui::{XBTVEd, EdBuffer};

#[no_mangle]
pub extern fn create_app() -> *const XBTVEd {
    let app = Box::new(XBTVEd::new());
    unsafe {
        mem::transmute(app)
    }
}

#[no_mangle]
pub extern fn display_name(xbtved: *const XBTVEd) -> *const i8 {
    unsafe {
        match CString::new((*xbtved).current_buffer().get_name()) {
            Ok(x) => x.as_ptr(),
            Err(_) => CString::new("SCREWED UP!").unwrap().as_ptr()
        }
    }
}

#[no_mangle]
pub extern fn buffers_len(xbtved: *const XBTVEd) -> u32 {
    unsafe {
        let res = (*xbtved).buffers_len();
        println!("{}", res);
        res as u32
    }
}
