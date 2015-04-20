extern crate conrod;
extern crate opengl_graphics;

use self::conrod::{
    Drawable,
    Positionable,
    Ui
};
use self::opengl_graphics::GlGraphics;
use self::opengl_graphics::glyph_cache::GlyphCache;

use std::io::{Error, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::fs::File;
use super::menu::MenuBar;
use super::super::schedule::Schedule;
use super::super::program::Program;
use super::super::action::Action;

pub struct EdBuffer {
    schedule: Schedule,
    filepath: Option<PathBuf>,
    undo_buffer: Vec<Box<Action>>,
    redo_buffer: Vec<Box<Action>>,
    modified: bool
}

impl<'a> EdBuffer {
    pub fn new() -> EdBuffer {
        EdBuffer {
            schedule: Schedule::example(),
            filepath: None,
            undo_buffer: Vec::new(),
            redo_buffer: Vec::new(),
            modified: true
        }
    }

    pub fn from_schedule(sched: &Schedule) -> EdBuffer {
        EdBuffer {
            schedule: sched.clone(),
            filepath: None,
            undo_buffer: Vec::new(),
            redo_buffer: Vec::new(),
            modified: false
        }
    }

    pub fn apply(&mut self, action: Box<Action>) -> Result<(), String> {
        if let Err(f) = action.apply(self) {
            self.modified = true;
            self.redo_buffer.clear();
            return Err(f)
        }
        self.modified = true;
        self.undo_buffer.push(action);
        self.redo_buffer.clear();
        Ok(())
    }

    pub fn undo(&mut self) {
        if let Some(action) = self.undo_buffer.pop() {
            if let Err(f) = action.reverse(self) {
                panic!(format!("Unexpected error occurred: {}", f))
            }
            self.modified = true;
            self.redo_buffer.push(action);
        }
    }

    pub fn redo(&mut self) {
        if let Some(action) = self.redo_buffer.pop() {
            if let Err(f) = action.apply(self) {
                panic!(format!("Unexpected error occurred: {}", f))
            }
            self.undo_buffer.push(action);
        }
    }

    pub fn is_undo_buffer_empty(&self) -> bool {
        self.undo_buffer.len() == 0
    }

    pub fn is_redo_buffer_empty(&self) -> bool {
        self.redo_buffer.len() == 0
    }

    pub fn set_path(&mut self, path: &Path) {
        self.filepath = Some(path.to_path_buf())
    }

    pub fn get_path(&'a self) -> Option<&'a Path> {
        match self.filepath {
            Some(ref pathbuf) => Some(pathbuf.as_path()),
            None => None
        }
    }

    pub fn get_schedule(&'a self) -> &'a Schedule {
        &self.schedule
    }

    pub fn set_name(&mut self, name: &str) {
        self.schedule.set_name(name)
    }

    pub fn get_name(&'a self) -> &'a str {
        self.schedule.name_ref()
    }

    pub fn get_program(&'a self) -> Option<&'a Program> {
        self.schedule.get_program()
    }

    pub fn get_program_at(&'a self, idx: usize) -> Option<&'a Program> {
        self.schedule.get_program_at(idx)
    }

    pub fn get_program_mut(&'a mut self) -> Option<&'a mut Program> {
        self.schedule.get_program_mut()
    }

    pub fn get_program_at_mut(&'a mut self, idx: usize) -> Option<&'a mut Program> {
        self.schedule.get_program_mut_at(idx)
    }

    pub fn last_program(&'a self) -> Option<&'a Program> {
        self.schedule.last_program()
    }

    pub fn add_program(&mut self, prog: &Program) {
        self.schedule.add_program(prog);
    }

    pub fn pop_program(&mut self) -> Option<Program> {
        self.schedule.pop_program()
    }

    pub fn insert_program(&mut self, idx: usize, prog: &Program) -> Result<(), String> {
        self.schedule.insert_program(idx, prog)
    }

    pub fn delete_program(&mut self, idx: usize) -> Result<(), String> {
        self.schedule.delete_program(idx)
    }

    pub fn modified(&self) -> bool {
        self.modified
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if self.filepath.is_none() {
            Err(Error::new(ErrorKind::Other, "There is no file for this buffer yet. Please use save as"))
        } else { 
            let path = self.get_path().unwrap().to_path_buf();
            let mut file = try!(File::create(path.as_path()));
            try!(file.write_all(self.get_schedule().to_string().as_bytes()));
            self.modified = false;
            Ok(())
        }
    }

    pub fn save_as(&mut self, path: &str) -> Result<(), Error> {
        self.filepath = Some(Path::new(path).to_path_buf());
        self.save()
    }
}

pub struct XBTVEd {
    buffers: Vec<EdBuffer>,
    current_buffer: usize,
    menu_bar: MenuBar,
    _width: f64,
    exit_signal: bool
}

impl<'a> XBTVEd {
    pub fn new(width: f64) -> XBTVEd {
        let file_entries = vec!("File".to_string(),
                                "New".to_string(), 
                                "Open".to_string(),
                                "Save".to_string(),
                                "Save as".to_string(),
                                "Exit".to_string());

        let edit_entries = vec!("Edit".to_string(),
                                "Undo".to_string(),
                                "Redo".to_string());

        let entries = vec!(file_entries, edit_entries);
//        let methods = vec!(file_methods, edit_methods);

        XBTVEd {
            buffers: vec!(EdBuffer::new()),
            current_buffer: 0,
            menu_bar: MenuBar::new(11.0, entries, width),
            _width: width,
            exit_signal: false,
        }
    }

    pub fn exit(&self) -> bool {
        self.exit_signal
    }

    pub fn current_buffer(&'a self) -> &'a EdBuffer {
        &self.buffers.get(self.current_buffer).unwrap()
    }
 
    pub fn current_buffer_mut(&'a mut self) -> &'a mut EdBuffer {
        self.buffers.get_mut(self.current_buffer).unwrap()
    }

    pub fn add_buffer(&mut self) {
        self.buffers.push(EdBuffer::new());
        self.current_buffer += 1;
    }

    pub fn prev_buffer(&mut self) {
        if self.current_buffer > 0 {
            self.current_buffer -= 1
        }
    }

    pub fn next_buffer(&mut self) {
        if self.current_buffer + 1 < self.buffers.len() {
            self.current_buffer += 1;
        }
    }

    pub fn set_current_schedule(&mut self, idx: usize) -> Result<(), String> {
        if idx >= self.buffers.len() {
            Err("Out of bounds".to_string())
        } else {
            self.current_buffer = idx;
            Ok(())
        }
    }

    pub fn remove_buffer(&mut self, idx: usize) -> Result<(), String> {
        if idx >= self.buffers.len() {
            Err("Out of bounds".to_string())
        } else if self.buffers.len() == 1 {
            self.buffers.remove(idx);

            self.buffers.push(EdBuffer::new());
            Ok(())
        } else {
            if idx + 1 == self.buffers.len() {
                self.current_buffer -= 1;
            }
            self.buffers.remove(idx);
            Ok(())
        }
    }

    pub fn open_file(&mut self, path: &str) -> Result<(), Error> {
        let pathname = Path::new(path);
        if let Some(idx) = self.buffers.iter().position(|ref edbuf| edbuf.get_path() == Some(&pathname)) {
            self.current_buffer = idx;
            return Ok(())
        }

        let mut file = try!(File::open(path));
        let mut s = String::new();
        try!(file.read_to_string(&mut s));
        let sched = match super::super::parse::parse(&s) {
            Ok(x) => x,
            Err(f) => return Err(Error::new(ErrorKind::Other, f.to_string().as_str()))
        };

        let mut buffer = EdBuffer::from_schedule(&sched);
        buffer.set_path(pathname);
        self.buffers.push(buffer);
        self.current_buffer += 1;

        Ok(())
    }

    pub fn save(&mut self) -> Result<(), Error> {
        self.current_buffer_mut().save()
    }

    pub fn save_as(&mut self, path: &str) -> Result<(), Error> {
        self.current_buffer_mut().save_as(path)
    }

    pub fn save_all(&mut self) -> Result<(), Error> {
        for buf in self.buffers.iter_mut() {
            if buf.modified() {
                try!(buf.save());
            }
        }
        Ok(())
    }

    pub fn buffers_len(&self) -> usize {
        self.buffers.len()
    }

    pub fn buffer_modified(&self) -> bool {
        self.current_buffer().modified()
    }

    pub fn any_buffer_modified(&self) -> bool {
        self.buffers.iter().any(|buf| buf.modified())
    }

    pub fn add_example(&mut self) {
        self.buffers.push(EdBuffer::new());
    }

    pub fn menu_bar_mut(&'a mut self) -> &'a mut MenuBar {
        &mut self.menu_bar
    }

    pub fn menu_bar(&'a self) -> &'a MenuBar {
        &self.menu_bar
    }

    pub fn draw_menus(&mut self, gl: &mut GlGraphics, ui: &mut Ui<GlyphCache<'a>>) {
        self.menu_bar.menu_mut(0).unwrap().draw(ui, gl);
        if let Some(idx) = self.menu_bar.menu(0).unwrap().idx() {
            match idx {
                0 => { }, //File
                1 => self.add_buffer(),
                5 => self.exit_signal = true,
                x => println!("{}", x)
            }
            self.menu_bar.menu_mut(0).unwrap().set_idx(None);
        }

        self.menu_bar.menu_mut(1).unwrap().draw(ui, gl);
        if let Some(idx) = self.menu_bar.menu(1).unwrap().idx() {
            match idx {
                1 => self.current_buffer_mut().undo(),
                2 => self.current_buffer_mut().redo(),
                _ => { }
            }
        }
    }
}
