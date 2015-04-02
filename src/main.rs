#![crate_name = "XBTVEd"]
#![feature(core, collections, unboxed_closures)]

extern crate rgtk;

use rgtk::*;
use rgtk::gtk::signals::{Clicked, DeleteEvent};
use std::num::FromPrimitive;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub mod parse;
pub mod schedule;
pub mod tags;
pub mod blocks;

fn main () {
    schedule::test();

    let mut scheds: Vec<schedule::Schedule> = Vec::new();

    gtk::init();

    let mut window = gtk::Window::new(gtk::WindowType::TopLevel).unwrap();
    window.set_title("XBTVEd");
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(800, 600);

    let mut toolbar = gtk::Toolbar::new().unwrap();

    let open_icon = gtk::Image::new_from_icon_name("document-open", gtk::IconSize::SmallToolbar).unwrap();
    let save_icon = gtk::Image::new_from_icon_name("document-save", gtk::IconSize::SmallToolbar).unwrap();
    let text_view = gtk::TextView::new().unwrap();

    let mut open_button = gtk::ToolButton::new::<gtk::Image>(Some(&open_icon), Some("Open")).unwrap();
    open_button.set_is_important(true);
    Connect::connect(&open_button, Clicked::new(&mut || {
        let file_chooser = gtk::FileChooserDialog::new(
            "Open File", None, gtk::FileChooserAction::Open,
            [("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]);
        let response: Option<gtk::ResponseType> = FromPrimitive::from_i32(file_chooser.run());

        match response {
            Some(gtk::ResponseType::Ok) => {
                let filename = file_chooser.get_filename().unwrap();
                let file = File::open(&filename).unwrap();

                let mut reader = BufReader::new(file);
                let mut contents = String::new();
                let _ = reader.read_to_string(&mut contents);

//                text_view.get_buffer().unwrap().set_text(&contents);

                match parse::parse(&contents) {
                    Ok(x) => {
                        text_view.get_buffer().unwrap().set_text(&(x.to_string()));
                        scheds.push(x);
                    },
                    Err(f) => {
                        text_view.get_buffer().unwrap().set_text(&(f.to_string()));
                    }
                }
            },
            _ => {}
        }
        
        file_chooser.destroy();
    }));
                     
    let mut save_button = gtk::ToolButton::new::<gtk::Image>(Some(&save_icon), Some("Save")).unwrap();
    save_button.set_is_important(true);
    Connect::connect(&save_button, Clicked::new(&mut || {
        if scheds.len() > 0 {
            let file_chooser = gtk::FileChooserDialog::new(
                "Save File", None, gtk::FileChooserAction::Save,
                [("Save", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]);
            let response: Option<gtk::ResponseType> = FromPrimitive::from_i32(file_chooser.run());

            match response {
                Some(gtk::ResponseType::Ok) => {
                    let filename = file_chooser.get_filename().unwrap();
                    let mut file = File::open(&filename).unwrap();

                    match file.write(scheds[0].to_string().as_bytes()) {
                        Ok(_) => { },
                        Err(f) => println!("{}", f)
                    }
                },                
                _ => println!("Oh no!")
            }

            file_chooser.destroy();
        } else { }
    }));

    toolbar.add(&open_button);
    toolbar.add(&save_button);

    let mut scroll = gtk::ScrolledWindow::new(None, None).unwrap();
    scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scroll.add(&text_view);

    let mut vbox = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    vbox.pack_start(&toolbar, false, true, 0);
    vbox.pack_start(&scroll, true, true, 0);

    window.add(&vbox);

    Connect::connect(&window, DeleteEvent::new(&mut |_| {
        gtk::main_quit();
        true
    }));

    window.show_all();
    gtk::main();

}
