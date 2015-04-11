use std::io::{Error, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::fs::File;
use super::schedule::Schedule;
use super::program::Program;
use super::action::{
    Action,
};

#[repr(C)]
pub struct EdBuffer {
    schedule: Schedule,
    filepath: Option<PathBuf>,
    undo_buffer: Vec<Box<Action>>,
    redo_buffer: Vec<Box<Action>>
}

impl<'a> EdBuffer {
    pub fn new() -> EdBuffer {
        EdBuffer {
            schedule: Schedule::example(),
            filepath: None,
            undo_buffer: Vec::new(),
            redo_buffer: Vec::new()
        }
    }

    pub fn from_schedule(sched: &Schedule) -> EdBuffer {
        EdBuffer {
            schedule: sched.clone(),
            filepath: None,
            undo_buffer: Vec::new(),
            redo_buffer: Vec::new()
        }
    }

    pub fn apply(&mut self, action: Box<Action>) -> Result<(), String> {
        if let Err(f) = action.apply(self) {
            self.redo_buffer.clear();
            return Err(f)
        }
        self.undo_buffer.push(action);
        self.redo_buffer.clear();
        Ok(())
    }

    pub fn undo(&mut self) {
        if let Some(action) = self.undo_buffer.pop() {
            if let Err(f) = action.reverse(self) {
                panic!(format!("Unexpected error occurred: {}", f))
            }
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
            Some(pathbuf) => Some(pathbuf.as_path()),
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

    pub fn get_program(&'a self, idx: usize) -> Option<&'a Program> {
        self.schedule.get_program(idx)
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

    pub fn save(&self) -> Result<(), Error> {
        if self.filepath.is_none() {
            Err(Error::new(ErrorKind::Other, "There is no file for this buffer yet. Please use save as"))
        } else { 
            let path = self.get_path().unwrap().to_path_buf();
            let mut file = try!(File::create(path.as_path()));
            try!(file.write_all(self.get_schedule().to_string().as_bytes()));
            Ok(())
        }
    }

    pub fn save_as(&mut self, path: &str) -> Result<(), Error> {
        self.filepath = Some(Path::new(path).to_path_buf());
        self.save()
    }
}

#[repr(C)]
pub struct XBTVEd {
    buffers: Vec<EdBuffer>,
    current_buffer: usize,
}

impl<'a> XBTVEd {
    pub fn new() -> XBTVEd {
        XBTVEd {
            buffers: vec!(EdBuffer::new()),
            current_buffer: 0,
        }
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

    pub fn open_file(&mut self, path: &Path) -> Result<(), Error> {
        if let Some(idx) = self.buffers.iter().position(|&edbuf| edbuf.get_path() == Some(path)) {
            self.current_buffer = idx;
            return Ok(())
        }
/*
        match self.buffers.iter().position(|x| x == pathbuf) {
            Some(idx) => {
                self.current_buffer = idx;
                return
            },
            None => { }
        }
*/
        let mut file = try!(File::open(path));
        let mut s = String::new();
        try!(file.read_to_string(&mut s));
        let sched = match super::parse::parse(&s) {
            Ok(x) => x,
            Err(f) => return Err(Error::new(ErrorKind::Other, f.to_string().as_str()))
        };

        let mut buffer = EdBuffer::from_schedule(&sched);
        buffer.set_path(path);
        self.buffers.push(buffer);
        self.current_buffer += 1;

        Ok(())
    }

    pub fn save(&self) -> Result<(), Error> {
        self.current_buffer().save()
    }

    pub fn save_as(&mut self, path: &str) -> Result<(), Error> {
        self.current_buffer_mut().save_as(path)
    }

    pub fn buffers_len(&self) -> usize {
        self.buffers.len()
    }

    pub fn add_example(&mut self) {
        self.buffers.push(EdBuffer::new());
    }
}
