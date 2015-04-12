#![crate_name = "XBTVEd"]
#![crate_type = "dylib"]
#![feature(libc, convert, collections, unboxed_closures)]

extern crate libc;

use std::ffi::{CStr, CString};
use std::mem;

pub mod action;
pub mod parse;
pub mod schedule;
pub mod program;
pub mod tags;
pub mod blocks;
pub mod gui;

pub use gui::XBTVEd;
use action::*;

#[no_mangle]
pub extern fn create_app() -> *const XBTVEd {
    let app = Box::new(XBTVEd::new());
    unsafe {
        mem::transmute(app)
    }
}

#[no_mangle]
pub extern fn undo(xbtved: *mut XBTVEd) {
    unsafe {
        (*xbtved).current_buffer_mut().undo() 
    }
}

#[no_mangle]
pub extern fn redo(xbtved: *mut XBTVEd) {
    unsafe {
        (*xbtved).current_buffer_mut().redo()
    }
}

#[no_mangle]
pub extern fn buffers_len(xbtved: *const XBTVEd) -> u32 {
    unsafe {
        let res = (*xbtved).buffers_len();
        res as u32
    }
}

#[no_mangle]
pub extern fn new_buffer(xbtved: *mut XBTVEd) {
    unsafe {
        (*xbtved).add_buffer();
    }
}

#[no_mangle]
pub extern fn prev_buffer(xbtved: *mut XBTVEd) {
    unsafe {
        (*xbtved).prev_buffer();
    }
}

#[no_mangle]
pub extern fn next_buffer(xbtved: *mut XBTVEd) {
    unsafe {
        (*xbtved).next_buffer();
    }
}

#[no_mangle]
pub extern fn get_buffer_name(xbtved: *const XBTVEd) -> *const i8 {
    unsafe {
        match CString::new((*xbtved).current_buffer().get_name()) {
            Ok(x) => x.as_ptr(),
            Err(_) => CString::new("SCREWED UP!").unwrap().as_ptr()
        }
    }
}

fn ptr_to_string(bullshit: *const libc::c_char) -> String {
    unsafe {
        match String::from_utf8(CStr::from_ptr(bullshit).to_bytes().to_vec()) {
            Ok(x) => x,
            Err(f) => panic!(f)
        }
    }
}

#[no_mangle]
pub extern fn set_buffer_name(xbtved: *mut XBTVEd, name: *const libc::c_char) {
    unsafe {
        let new_name = ptr_to_string(name);
        let name_change = SetName::new((*xbtved).current_buffer(), &new_name);
        if let Err(f) = (*xbtved).current_buffer_mut().apply(name_change) {
            panic!(f)
        }
    }
}

#[no_mangle]
pub extern fn add_program(xbtved: *mut XBTVEd,
                          source: *const libc::c_char, 
                          loc: *const libc::c_char) -> *const i8 {
    unsafe {
        let location = ptr_to_string(loc);
        let source = match ptr_to_string(source).as_str() {
            "local" => program::Source::Pathname(location),
            "network" => program::Source::URL(location),
            x => return CString::new(format!("{} is invalid source", x)).unwrap().as_ptr()
        };

        let prog = program::Program::new(source, tags::Tags::new(), 
                                         vec!(program::Instruction::Play(0, 0)));
        let add_prog = AddProgram::new(&prog);
        
        match (*xbtved).current_buffer_mut().apply(add_prog) {
            Err(f) => CString::new(f).unwrap().as_ptr(),
            Ok(_) => CString::new("Ok").unwrap().as_ptr()
        }
    }
}
