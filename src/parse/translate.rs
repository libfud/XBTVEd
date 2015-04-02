//! Translate tokens into expressions and atoms.

use super::ParseError::*;
use super::{Token, Schedule, ParseError};
use super::Token::*;
use super::tokenize::TokenStream;
use super::super::tags::Tags;
use super::super::schedule::{Program, Source, Instruction};
use super::super::schedule::Source::*;

pub type SchedResult = Result<Schedule, ParseError>;
pub type ParseResult = Result<(), ParseError>;

pub fn begin_expr(tokens: &mut TokenStream<Token, ParseError>) -> Result<(),ParseError> {
    match tokens.next() {
        Some(Ok(Token::LParen)) => Ok(()),
        Some(Ok(x)) => Err(BadToken(format!("{}{}", "Expected LParen but found ",x))),
        Some(Err(msg)) => Err(msg),
        None => panic!("Unexpected end of tokenstream")
    }
}

pub fn strip<T>(t: Option<Result<T, ParseError>>) -> Result<T, ParseError> {
    match t {
        Some(x) => x,
        None => Err(BadToken("Expected a token but found nothing!".to_string()))
    }
}

pub fn get_location(tokens: &mut TokenStream<Token, ParseError>) -> Result<Source, ParseError> {
    try!(begin_expr(tokens));
    let res = match try!(strip(tokens.next())) {
        Local => match try!(strip(tokens.next())) {
            Location(x) => Ok(Pathname(x)),
            x => Err(BadToken(format!("{}{}", "Expected source for file, found ".to_string(), x))),
        },
        Network => match try!(strip(tokens.next())) {
            Location(x) => Ok(Pathname(x)),
            x => Err(BadToken(format!("{}{}", "Expected source for file, found ".to_string(), x))),
        },
        x => Err(BadToken(format!("{}{}", "Expected source for file, found ".to_string(), x)))
    };
    match try!(strip(tokens.next())) {
        RParen => res,
        x => Err(BadToken(format!("{}{}", "In get_location, expected rparen, found ".to_string(), x)))
    }
}

pub fn get_tags(tokens: &mut TokenStream<Token, ParseError>) -> Result<Tags, ParseError> {
    try!(begin_expr(tokens));
    match try!(strip(tokens.next())) {
        Tag => {
            let mut tags = Tags::new();
            loop {
                match try!(strip(tokens.next())) {
                    TagData(ref x, ref y) => {
                        try!(tags.modify_tag(x, y));
                    },
                    RParen => break,
                    x => return Err(BadToken(format!("{}{}", "Expected rparen, found ".to_string(), x)))
                }
            }
            Ok(tags)
        },
        _ => {
            match tokens.rev(2) {
                Ok(()) => Ok(Tags::new()),
                Err(()) => panic!("Unexpected truncation of string!")
            }
        }
    }
}

pub fn play_handler(tokens: &mut TokenStream<Token, ParseError>) -> Result<Instruction, ParseError> {
    match try!(strip(tokens.next())) {
        RParen => Ok(Instruction::Play(0, 0)),

        Time(start) => match try!(strip(tokens.next())) {
            RParen => Ok(Instruction::Play(start, 0)),

            Time(duration) => match try!(strip(tokens.next())) {
                RParen => Ok(Instruction::Play(start, duration)),

                x => Err(BadToken(format!("{}{}{}", "Expected rparen but found ".to_string(),
                                          x.to_string(),
                                          ". Only start time and duration permitted.".to_string())))
            },

            x => Err(BadToken(format!("{}{} ", "Expected time or rparen but found ".to_string(),
                                         x.to_string())))
        },

        x => Err(BadToken(format!("{}{} ", "Expected time or rparen but found ".to_string(), x.to_string())))
    }
}

pub fn add_instrs(tokens: &mut TokenStream<Token, ParseError>) -> Result<Vec<Instruction>, ParseError> {
    try!(begin_expr(tokens));
    match try!(strip(tokens.next())) {
        Instr => {
            let mut instructions = Vec::new();
            try!(begin_expr(tokens));
            loop { 
                match try!(strip(tokens.next())) {
                    LParen => continue,
                    Play => { 
                        instructions.push(try!(play_handler(tokens)));
                    },
                    Prog => {
                        instructions.push(Instruction::SubProgram(try!(add_program(tokens))));
                    },
                    RParen => break,
                    x => return Err(BadToken(format!("{}{} ", 
                                         "Expected beginnning of instructions but found ".to_string(),
                                         x.to_string())))
                }
            }
            Ok(instructions)
        },
        x => Err(BadToken(format!("{}{} ", "Expected beginnning of instructions but found ".to_string(),
                                  x.to_string())))
    }


}

pub fn add_program(tokens: &mut TokenStream<Token, ParseError>) -> Result<Program, ParseError> {

    let source = try!(get_location(tokens));
    let tags = try!(get_tags(tokens));
    let instructions = try!(add_instrs(tokens));

    match try!(strip(tokens.next())) {
        RParen => { },
        x => return Err(BadToken(format!("{}{}", "Expected closing paren, but found ".to_string(), x)))
    }

    let prog = Program::new(source, tags, instructions);
    Ok(prog)
}

pub fn translate(tokens: &mut TokenStream<Token, ParseError>) -> SchedResult {
    try!(begin_expr(tokens));
    let mut progs: Vec<Program> = match try!(strip(tokens.next())) {
        Sched => Vec::new(),
        _ => return Err(BadAction)
    };

    let name = match try!(strip(tokens.next())) {
        Location(x) => x,
        x => return Err(BadToken(format!("{}{}", "Expected name but found ", x)))
    };

    loop {
        match tokens.next() {
            Some(Ok(LParen)) => continue,
            Some(Ok(Prog)) => {
                progs.push(try!(add_program(tokens)));
            },
            Some(Ok(RParen)) => match tokens.next() {
                None => break,
                Some(Ok(x)) => return Err(BadToken(format!("{}{}", 
                                                       "Expected end of tokenstream but found ".to_string(),
                                                       x.to_string()))),
                Some(Err(f)) => return Err(f)
            },
            Some(Ok(x)) => return Err(BadToken(format!("{}{}", 
                                                       "Expected (, ), or Program but found ".to_string(),
                                                       x.to_string()))),
            Some(Err(f)) => return Err(f),
            None => break
        }
    }
    Ok(Schedule::new(name, progs))
}
