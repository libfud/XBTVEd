use super::gui::EdBuffer;
use super::program::{Program, Source};

pub trait Action {
    fn apply(&self, buffer: &mut super::gui::EdBuffer) -> Result<(), String>;
    fn reverse(&self, buffer: &mut super::gui::EdBuffer) -> Result<(), String>;
}

//Schedule methods

pub struct SetName {
    old: String,
    new: String
}

impl Action for SetName {
    fn apply(&self, buffer: &mut super::gui::EdBuffer) -> Result<(), String> {
        buffer.set_name(&self.new);
        Ok(())
    }
    fn reverse(&self, buffer: &mut super::gui::EdBuffer) -> Result<(), String> {
        buffer.set_name(&self.old);
        Ok(())
    }
}

impl SetName {
    pub fn new(buffer: &EdBuffer, novo: &str) -> Box<Action> {
        let set_name = SetName {
            old: buffer.get_name().to_string(),
            new: novo.to_string()
        };
        Box::new(set_name)
    }
}

pub struct AddProgram {
    program: Program,
}

impl AddProgram {
    pub fn new(prog: &Program) -> Box<Action> {
        Box::new(AddProgram { program: prog.clone() })
    }
}

impl Action for AddProgram {
    fn apply(&self, buffer: &mut EdBuffer) -> Result<(), String> {
        buffer.add_program(&self.program);
        Ok(())
    }

    fn reverse(&self, buffer: &mut EdBuffer) -> Result<(), String> {
        if let Some(x) =  buffer.pop_program() {
            if x != self.program {
                panic!("Modification made after undo without clearing redo buffer!")
            }
        }
        Ok(())
    }
}

pub struct PopProgram {
    program: Program
}

impl PopProgram {
    pub fn new(buffer: &EdBuffer) -> Result<Box<Action>, String> {
        match buffer.last_program() {
            Some(p) => Ok(Box::new(PopProgram { program: p.clone() })),
            None =>  Err("Can not pop an empty program".to_string())
        }
    }
}

impl Action for PopProgram {
    fn apply(&self, buffer: &mut EdBuffer) -> Result<(), String> {
        if let Some(x) = buffer.pop_program() {
            if x != self.program {
                panic!("Modification made after undo without clearing redo buffer!")
            }
        }
        Ok(())
    }

    fn reverse(&self, buffer: &mut EdBuffer) -> Result<(), String> {
        buffer.add_program(&self.program);
        Ok(())
    }
}

pub struct InsertProgram {
    program: Program,
    index: usize
}

impl InsertProgram {
    pub fn new(prog: &Program, idx: usize) -> Box<Action> {
        Box::new(InsertProgram {
            program: prog.clone(),
            index: idx
        })
    }
}

impl Action for InsertProgram {
    fn apply(&self, buffer: &mut EdBuffer) -> Result<(), String> {
        buffer.insert_program(self.index, &self.program)
    }

    fn reverse(&self, buffer: &mut EdBuffer) -> Result<(), String> {
        if let Err(f) = buffer.delete_program(self.index) {
            panic!(format!("Modifications made after undo. Error: {}", f))
        }
        Ok(())
    }
}

pub struct DeleteProgram {
    program: Program,
    index: usize
}

impl DeleteProgram {
    pub fn new(buffer: &EdBuffer, idx: usize) -> Result<Box<Action>, String> {
        match buffer.get_program_at(idx) {
            Some(p) => Ok(Box::new(DeleteProgram { program: p.clone(), index: idx })),
            None => Err(format!("Can not delete program at {} because it does not exist", idx))
        }
    }
}

impl Action for DeleteProgram {
    fn apply(&self, buffer: &mut EdBuffer) -> Result<(), String> {
        buffer.delete_program(self.index)
    }

    fn reverse(&self, buffer: &mut EdBuffer) -> Result<(), String> {
        if let Err(f) = buffer.insert_program(self.index, &self.program) {
            panic!(format!("Modifications made after undo. Error: {}", f))
        }
        Ok(())
    }
}

//Program methods
pub struct SetSource {
    old: Source,
    new: Source,
    index: usize
}

impl SetSource {
    pub fn new(buffer: &EdBuffer, novo: &Source) -> Result<Box<Action>, String> {
        if let Some(ref prog) = buffer.get_program() {
            Ok(Box::new(SetSource { 
                old: prog.get_location().clone(), 
                new: novo.clone(),
                index: buffer.get_schedule().get_current_program_idx().unwrap()
            }))
        } else {
            Err("No selected program".to_string())
        }
    }
}

impl Action for SetSource {
    fn apply(&self, buffer: &mut EdBuffer) -> Result<(), String> {
        buffer.get_program_at_mut(self.index).unwrap().set_location(&self.new);
        Ok(())
    }

    fn reverse(&self, buffer: &mut EdBuffer) -> Result<(), String> {
        buffer.get_program_at_mut(self.index).unwrap().set_location(&self.old);
        Ok(())
    }
}
