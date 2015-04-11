use std::fmt;
use super::tags::Tags;

#[derive(Clone, PartialEq, Debug)]
pub enum Source {
    Pathname(String),
    URL(String)
}

impl<'a> Source {
    pub fn is_path(&self) -> bool {
        match self {
            &Source::Pathname(_) => true,
            _ => false
        }
    }

    pub fn is_url(&self) -> bool {
        match self {
            &Source::URL(_) => true,
            _ => false
        }
    }

    pub fn path(&'a self) -> Option<&'a str> {
        match self {
            &Source::Pathname(ref s) => Some(s),
            _ => None
        }
    }

    pub fn url(&'a self) -> Option<&'a str> {
        match self {
            &Source::URL(ref s) => Some(s),
            _ => None
        }
    }
}

impl fmt::Display for Source {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Source::Pathname(ref x) => try!(write!(fmt, "local {}", x.clone())),
            Source::URL(ref x) => try!(write!(fmt, "network {}", x.clone()))
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Instruction {
    Play(usize, usize), //Start Time and End Time.
    SubProgram(Program)
}

impl<'a> Instruction {
    pub fn is_play(&self) -> bool {
        match self {
            &Instruction::Play(_, _) => true,
            _ => false
        }
    }

    pub fn is_subprogram(&self) -> bool {
        match self {
            &Instruction::SubProgram(_) => true,
            _ => false
        }
    }

    pub fn start_time(&self) -> Option<usize> {
        match self {
            &Instruction::Play(x, _) => Some(x),
            _ => None
        }
    }

    pub fn duration(&self) -> Option<usize> {
        match self {
            &Instruction::Play(_, x) => Some(x),
            _ => None
        }
    }

    pub fn start_and_duration(&self) -> Option<(usize, usize)> {
        match self {
            &Instruction::Play(x, y) => Some((x, y)),
            _ => None
        }
    }

    pub fn subprogram(&'a self) -> Option<&'a Program> {
        match self {
            &Instruction::SubProgram(ref p) => Some(p),
            _ => None
        }
    }

    pub fn subprogram_mut(&'a mut self) -> Option<&'a mut Program> {
        match self {
            &mut Instruction::SubProgram(ref mut p) => Some(p),
            _ => None
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::Play(x, y) => {
                try!(write!(fmt, "(play "));
                
                let (mut total_x, mut total_y) = (x, y);
                
                let (start_h, duration_h) = (total_x / 3600, total_y / 3600);
                
                total_x -= start_h * 3600;
                total_y -= duration_h * 3600;
                
                let (start_m, duration_m) = (total_x / 60, total_y / 60);
                
                total_x -= start_m * 60;
                total_y -= duration_m * 60;
                
                try!(write!(fmt, " {}:{}:{} ",
                            if start_h < 10 { 
                                format!("{}{}",0,start_h) 
                            } else { 
                                start_h.to_string() 
                            },
                            if start_m < 10 { 
                                format!("{}{}",0,start_m)
                            } else { 
                                start_m.to_string()
                            },
                            if total_x < 10 { 
                                format!("{}{}",0,total_x)
                            } else { 
                                total_x.to_string()
                            }));

                try!(write!(fmt, " {}:{}:{} ",
                            if duration_h < 10 { 
                                format!("{}{}",0,duration_h) 
                            } else { 
                                duration_h.to_string()
                            },
                            if duration_m < 10 { 
                                format!("{}{}",0,duration_m) 
                            } else { 
                                duration_m.to_string()
                            },
                            if total_y < 10 { 
                                format!("{}{}",0,total_y)
                            } else { 
                                total_y.to_string()
                            }));

                try!(write!(fmt, ")"));
            },
            Instruction::SubProgram(ref x) => {
                try!(write!(fmt, "{}", x.to_string()));
            }
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Program {
    location: Source,
    tags: Tags,
    instructions: Vec<Instruction>,
    current_instr: Option<usize>
}

impl fmt::Display for Program {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "(program ({}", match (*self).location {
            Source::Pathname(ref x) => format!("local \"{}\")", x),
            Source::URL(ref x) => format!("network \"{}\")", x)
        }));
        try!(write!(fmt, "(tags "));
        try!(write!(fmt, "{})", self.tags));
        try!(write!(fmt, "(instr "));
        for instr in self.instructions.iter() {
            try!(write!(fmt, "{}", format!("{}",instr)));
        }
        try!(write!(fmt, "))"));
        Ok(())
    }
}

impl<'a> Program {
    pub fn new(source: Source, tags: Tags, instrs: Vec<Instruction>) -> Program {
        let current = match instrs.len() {
            0 => None,
            x => Some(x - 1)
        };
        Program { 
            location: source,
            tags: tags,
            instructions: instrs,
            current_instr: current
        }
    }

    pub fn example() -> Program {
        Program {
            location: Source::Pathname("example".to_string()),
            tags: Tags::new(),
            instructions: vec!(Instruction::Play(0, 0)),
            current_instr: Some(0)
        }
    }

    pub fn get_location(&'a self) -> &'a Source {
        &self.location
    }

    pub fn set_location(&mut self, location: &Source) {
        self.location = location.clone();
    }

    pub fn get_path(&'a self) -> Option<&'a str> {
        self.location.path()
    }

    pub fn set_path(&mut self, path: &str) {
        self.location = Source::Pathname(path.to_string());
    }

    pub fn get_url(&'a self) -> Option<&'a str> {
        self.location.url()
    }

    pub fn set_url(&mut self, url: &str) {
        self.location = Source::URL(url.to_string())
    }

    pub fn is_location_path(&self) -> bool {
        self.location.is_path()
    }

    pub fn is_location_url(&self) -> bool {
        self.location.is_url()
    }

    pub fn get_tags(&'a self) -> &'a Tags {
        &self.tags
    }

    pub fn set_tags(&mut self, tags: &Tags) {
        self.tags = tags.clone();
    }

    pub fn delete_all_tags(&mut self) {
        self.tags = Tags::new();
    }

    pub fn get_instrs(&'a self) -> &'a Vec<Instruction> {
        &self.instructions
    }

    pub fn get_current_instr(&'a self) -> Option<&'a Instruction> {
        if self.current_instr.is_none() {
            None
        } else {
            self.instructions.get(self.current_instr.unwrap())
        }
    }

    pub fn get_current_instr_mut(&'a mut self) -> Option<&'a mut Instruction> {
        if self.current_instr.is_none() {
            None
        } else {
            self.instructions.get_mut(self.current_instr.unwrap())
        }
    }

    pub fn get_instr(&'a self, idx: usize) -> Option<&'a Instruction> {
        self.instructions.get(idx)
    }

    pub fn get_instr_mut(&'a mut self, idx: usize) -> Option<&'a mut Instruction> {
        self.instructions.get_mut(idx)
    }

    pub fn push_instruction(&mut self, instr: &Instruction) {
        self.instructions.push(instr.clone());
        self.next_instr();
    }

    pub fn insert_instruction(&mut self, idx: usize, elt: &Instruction) -> Result<(), String> {
        if idx > self.instructions.len() {
            Err("Out of bounds".to_string())
        } else {
            self.instructions.insert(idx, elt.clone());
            Ok(())
        }
    }

    pub fn pop_instruction(&mut self) -> Option<Instruction> {
        let prog = self.instructions.pop();
        if self.instructions.len() == 0 {
            self.current_instr = None
        } else {
            self.prev_instr();
        }

        prog
    }

    pub fn remove_instruction(&mut self, idx: usize) -> Result<Instruction, String> {
        if idx > self.instructions.len() {
            Err("Out of bounds".to_string())
        } else {
            if idx == self.instructions.len() {
                self.prev_instr();
            } else if idx > self.current_instr.unwrap() {
                self.current_instr = Some(idx - 1);
            }
            Ok(self.instructions.remove(idx))
        }
    }

    pub fn set_current_instr_idx(&mut self, idx: usize) -> Result<(), String> {
        if idx > self.instructions.len() {
            Err("Out of bounds".to_string())
        } else {
            self.current_instr = Some(idx);
            Ok(())
        }
    }

    pub fn next_instr(&mut self) {
        let current_maybe = self.current_instr;
        match (self.instructions.len() > 0, current_maybe) {
            (true, Some(current)) => {
                let max = self.instructions.len() - 1;
                if current < max {
                    self.current_instr = Some(current + 1);
                }
            },
            (true, None) => {
                self.current_instr = Some(0);
            },
            (false, None) => { },
            (false, Some(_)) => panic!("Bad condition of instructions for program.")
        }
    }

    pub fn prev_instr(&mut self) {
        let current_maybe = self.current_instr;
        match (self.instructions.len() > 0, current_maybe) {
            (true, Some(current)) => {
                if current > 0 {
                    self.current_instr = Some(current - 1);
                }
            },
            (true, None) => {
                self.current_instr = Some(0);
            },
            (false, None) => { },
            (false, Some(_)) => panic!("Bad condition of instructions for program.")
        }
    }

}
