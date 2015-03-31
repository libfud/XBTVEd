use std::fmt;
use super::tags::Tags;

pub type Schedule = Vec<Program>;

impl fmt::Display for Schedule {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "(schedule "));
        for program in self.iter() {
            try!(write!(fmt, "{}", program));
        }
        try!(write!(fmt, ")"));
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Source {
    Pathname(String),
    URL(String)
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

impl fmt::Display for Vec<Instruction> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "(instr "));
        for instr in self.iter() {
            try!(write!(fmt, "{}", format!("{} ",instr)));
        }
        try!(write!(fmt, " )"));
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Program {
    location: Source,
    tags: Tags,
    instructions: Vec<Instruction>
}

impl fmt::Display for Program {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(fmt, "(program ({}", match (*self).location {
            Source::Pathname(ref x) => format!("local \"{}\")", x),
            Source::URL(ref x) => format!("network \"{}\")", x)
        }));
        try!(write!(fmt, "{}", self.tags));
        try!(write!(fmt, "{})", self.instructions));
        Ok(())
    }
}

impl Program {
    pub fn new(source: Source, tags: Tags, instrs: Vec<Instruction>) -> Program {
        Program { 
            location: source,
            tags: tags,
            instructions: instrs
        }
    }
}

pub fn test() {
    let test1 = 
        "(schedule 
             (program (local \"~/htpc/Videos/fsn.webm\") 
                 (tags media_type=\" anime\" studio=\"Studio Deen\")
                 (instr (play 00:00:01 00:10:00) 
                     (program 
                        ( network \"https://www.youtube.com/watch?v=EiTInQ6R8eM\" 
                        )
                        (tags ) (instr (play 00:01:00))
                     )
                     (play 00:10:00)
                 )
             )
             (program (local \"~/htpc/Music/Gorillaz/Gorillaz/Punk.ogg\" ) 
                      (tags artist=\"Gorillaz\") (instr (play )))
         )";
    println!("{}", test1);
    let test2 = match super::parse::parse(test1) {
        Ok(res) => { 
            println!("{}", res);
            res
        },
        Err(f) => { 
            println!("Error: {}", f);
            return
        }
    };
    

    let test3 = test2.to_string();
    println!("{}", test3);
    let mut test4 = match super::parse::parse(&test3) {
        Ok(res) => res,
        Err(f) => {
            println!("{}", f);
            panic!("Crap!")
        }
    };

    assert_eq!(test2,test4);

    test4.get_mut(0).unwrap().tags.director=Some("John Wayne".to_string());

    println!("{}", test4);

    println!("Success!");
}
