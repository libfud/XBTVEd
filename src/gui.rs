/*
use std::sync::mpsc::channel;
use std::thread;
*/

use std::io;
use std::io::{Error, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::collections::VecDeque;
use super::schedule::{Schedule, Source, Program, Instruction};
use super::tags::Tags;
use super::action::*;

pub struct EdBuffer {
    schedule: Schedule,
    filepath: Option<PathBuf>,
    undo_buffer: VecDeque<Box<Action>>,
    redo_buffer: VecDeque<Box<Action>>
}

impl<'a> EdBuffer {
    pub fn new() -> EdBuffer {
        let loc = Source::Pathname("foo".to_string());
        let tags = Tags::new();
        let instrs = Instruction::Play(0, 0);
        let progs = vec!(Program::new(loc, tags, vec!(instrs)));
        EdBuffer {
            schedule: Schedule::new("Example", progs),
            filepath: None,
            undo_buffer: VecDeque::new(),
            redo_buffer: VecDeque::new()
        }
    }

    pub fn from_schedule(sched: Schedule) -> EdBuffer {
        EdBuffer {
            schedule: sched,
            filepath: None,
            undo_buffer: VecDeque::new(),
            redo_buffer: VecDeque::new()
        }
    }

    pub fn apply(&mut self, action: Box<Action>) {
        action.apply(self);
        self.undo_buffer.push_back(action);
        self.redo_buffer.clear();
    }

    pub fn undo(&mut self) {
        if let Some(action) = self.undo_buffer.pop_back() {
            action.reverse(self);
            self.redo_buffer.push_back(action);
        }
    }

    pub fn redo(&mut self) {
        if let Some(action) = self.redo_buffer.pop_back() {
            action.apply(self);
            self.undo_buffer.push_back(action);
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

    pub fn get_path(&self) -> Option<PathBuf> {
        self.filepath.clone()
    }

    pub fn get_schedule(&'a self) -> &'a Schedule {
        &self.schedule
    }

    pub fn set_name(&mut self, name: &str) {
        self.schedule.set_name(name)
    }

    pub fn save(&self) -> Result<(), Error> {
        if self.filepath.is_none() {
            Err(Error::new(ErrorKind::Other, "There is no file for this buffer yet. Please use save as"))
        } else { 
            let path = self.get_path().unwrap();
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
        let mut file = try!(File::open(path));
        let mut s = String::new();
        try!(file.read_to_string(&mut s));
        let sched = match super::parse::parse(&s) {
            Ok(x) => x,
            Err(f) => return Err(Error::new(ErrorKind::Other, f.to_string().as_str()))
        };

        let mut buffer = EdBuffer::from_schedule(sched);
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
}

pub fn get_input<'a>(msg: Option<&'a str>) -> Option<String> {
    if let Some(words) = msg {
        println!("{}", words);
    }

    let mut input = String::new();
    let mut reader = io::stdin();
    if let Err(f) = reader.read_line(&mut input) {
        println!("{}", f);
        return None
    }

    Some(input.trim().to_string())
}

pub fn draw_ui(xbtved: &mut XBTVEd) {
    let ui = 
"Valid commands are the following:
File:     Schedule:        Program:     Actions:        Selection:  
New       ChangeName       ChangeLoc    AddPlayAction   SelectSched      
Open      AddProgram       EditTags     AddSubProgram   SelectNextProg 
Save      InsertProgram                 DeleteAction    SelectPrevProg
SaveAs    DeleteProgram                 CutAction       SelectProg
Quit      CutProgram                    CopyAction  
          CopyProgram                   PasteAction
          PasteProgram  

Display:            History:
DisplaySchedule     Undo
DisplaySelected     Redo

type `ui' to see this message again.
";
    println!("{}", ui);
    loop {

        let input = match get_input(None) {
            Some(x) => x,
            None => continue
        };

        match input.as_str() {
            "quit" | "exit" | "Quit" => break,
            "ui" => println!("{}", ui),
            "new" | "New" => xbtved.add_buffer(),
            "Open" | "open" => {
                let path = match get_input(Some("Type the name of the file to open.")) {
                    Some(x) => {
                        if x.len() == 0 {
                            println!("No filename, no opening.");
                            continue
                        }
                        x
                    },
                    None => {
                        println!("No filename, no opening.");
                        continue
                    }
                };
                if let Err(f) = xbtved.open_file(Path::new(&path)) {
                    println!("{}", f)
                }
            },
            "save" | "Save" => {
                if let Err(uh_oh) = xbtved.save() {
                    println!("{}", uh_oh)
                }
            },
            "save as" | "SaveAs" => {
                let path = match get_input(Some("Type the name of the file to open.")) {
                    Some(x) => {
                        if x.len() == 0 {
                            println!("No filename, no opening.");
                            continue
                        }
                        x
                    },
                    None => {
                        println!("No filename, no opening.");
                        continue
                    }
                };

                if let Err(f) = xbtved.save_as(&path) {
                    println!("{}", f)
                }
            },

            "DisplaySchedule" | 
            "displayschedule" | 
            "display schedule" => println!("{}", xbtved.current_buffer().get_schedule()),

            "ChangeName"  |
            "Change Name" |
            "change name" => {
                let new = match get_input(Some("Type the new name.")) {
                    Some(x) => {
                        if x.len() == 0 {
                            println!("No zero length names, please.");
                            continue
                        }
                        x
                    },
                    None => {
                        println!("No zero length names.");
                        continue
                    }
                };
                let old = xbtved.current_buffer().get_schedule().get_name();
                xbtved.current_buffer_mut().apply(ChangeName::new(&old, &new));
            },

            "Undo" | "undo" => xbtved.current_buffer_mut().undo(),
            "Redo" | "redo"  => xbtved.current_buffer_mut().redo(),

            _ => println!("{} is not a valid command. Type ui to see all valid commands", input)
        }
    }
}
