use std::fmt;
use super::program::Program;

#[derive(Clone, PartialEq, Debug)]
pub struct Schedule {
    programs: Vec<Program>,
    name: String,
    current_program: Option<usize>
}

impl fmt::Display for Schedule {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "(schedule \"{}\" ", self.name));
        for program in self.programs.iter() {
            try!(write!(fmt, "{}", program));
        }
        try!(write!(fmt, ")"));
        Ok(())
    }
}

impl<'a> Schedule {
    pub fn new(nom: &str, progs: Vec<Program>) -> Schedule {
        let current = match progs.len() {
            0 => None,
            x => Some(x - 1)
        };
        Schedule { 
            name: nom.to_string(), 
            programs: progs,
            current_program: current
        }
    }

    pub fn example() -> Schedule {
        Schedule {
            name: "Example".to_string(),
            programs: vec!(Program::example()),
            current_program: Some(0)
        }
    }

    pub fn set_name(&mut self, nom: &str) {
        self.name = nom.to_string();
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn name_ref(&'a self) -> &'a str {
        &self.name
    }
    
    pub fn programs(&self) -> Vec<Program> {
        self.programs.clone()
    }

    pub fn last_program(&'a self) -> Option<&'a Program> {
        self.programs.get(self.programs.len() - 1)
    }

    pub fn add_program(&mut self, prog: &Program) {
        self.programs.push(prog.clone());
    }

    pub fn pop_program(&mut self) -> Option<Program> {
        self.programs.pop()
    }

    pub fn delete_program(&mut self, idx: usize) -> Result<(), String> {
        if idx > self.programs.len() - 1 {
            Err("Index is out of bounds.".to_string())
        } else {
            self.programs.remove(idx);
            Ok(())
        }
    }

    pub fn insert_program(&mut self, idx: usize, prog: &Program) -> Result<(), String> {
        if idx > self.programs.len() - 1 {
            Err("Index is out of bounds.".to_string())
        } else {
            self.programs.insert(idx, prog.clone());
            Ok(())
        }
    }

    pub fn get_program_mut(&'a mut self, idx: usize) -> Option<&'a mut Program> {
        self.programs.get_mut(idx)
    }

    pub fn get_program(&'a self, idx: usize) -> Option<&'a Program> {
        self.programs.get(idx)
    }

    pub fn programs_len(&self) -> usize {
        self.programs.len()
    }
}   
