//! Translate tokens into expressions and atoms.

use super::ParseError::*;
use super::{Token, Schedule, ParseError};
use super::Token::*;
use super::tokenize::TokenStream;
use super::super::tags::TagType::{Cast, AudioTracks, Subtitles};
use super::super::tags::Tags;
use super::super::program::{Program, Source, Instruction};
use super::super::program::Source::*;

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

pub fn create_list(tokens: &mut TokenStream<Token, ParseError>) -> Result<Vec<String>, ParseError> {
    let mut list = Vec::new();
    loop {
        match try!(strip(tokens.next())) {
            LParen => return Err(BadToken("Nested lists are not allowed".to_string())),
            RParen => break,
            List => return Err(BadToken("Nested lists are not allowed".to_string())),
            Data(x) => list.push(x),
            x => return Err(BadToken(format!("Expected tag data, found {}", x)))
        }
    }
    Ok(list)
}    

pub fn get_location(tokens: &mut TokenStream<Token, ParseError>) -> Result<Source, ParseError> {
    try!(begin_expr(tokens));
    let loc_type = try!(strip(tokens.next()));
    let loc_place = try!(strip(tokens.next()));
    let res = match (loc_type, loc_place) {
        (Local, Data(x)) => Ok(Pathname(x)),
        (Network, Data(x)) => Ok(URL(x)),
        (x, y) => return Err(BadToken(format!("Expected source for file, found {} {}", x, y)))
    };
    match try!(strip(tokens.next())) {
        RParen => res,
        x => Err(BadToken(format!("In get_location, expected rparen, found {}", x)))
    }
}

pub fn get_tags(tokens: &mut TokenStream<Token, ParseError>) -> Result<Tags, ParseError> {
    try!(begin_expr(tokens));
    let mut tokens2 = tokens.clone();
    if let Some(Ok(Tag)) = tokens2.next() {
        try!(strip(tokens.next()));
        let mut tags = Tags::new();
        loop {
            let tag_type = match try!(strip(tokens.next())) {
                TagKind(x) => x,
                RParen => break,
                x => return Err(BadToken(format!("Expected tag type or rparen, found {}", x)))
            };

            match tag_type {
                Cast |
                AudioTracks |
                Subtitles => {
                    try!(begin_expr(tokens));
                    let list = match try!(strip(tokens.next())) {
                        List => try!(create_list(tokens)),
                        x => return Err(BadToken(format!("Expected a list for tag data, found {}", x)))
                    };
                    try!(tags.modify_multi(&list, tag_type));
                },
                single => {
                    let tagdata = match try!(strip(tokens.next())) {
                        Data(x) => x,
                        x => return Err(BadToken(format!("Expected tag data, found {}", x)))
                    };
                    try!(tags.modify_tag(&single, &tagdata));
                }
            }
        }
        Ok(tags)
    } else {
        match try!(strip(tokens.previous())) {
            LParen => { },
            x => panic!("Expected lparen when reversing tokenstream, found {}", x)
        }
            
        Ok(Tags::new())
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
                    x => return Err(BadToken(format!("Expected beginnning of instructions but found {}", x)))
                }
            }
            Ok(instructions)
        },
        x => return Err(BadToken(format!("Expected beginnning of instructions but found {}", x)))
    }


}

pub fn add_program(tokens: &mut TokenStream<Token, ParseError>) -> Result<Program, ParseError> {

    let source = try!(get_location(tokens));
    let tags = try!(get_tags(tokens));
    let instructions = try!(add_instrs(tokens));

    match try!(strip(tokens.next())) {
        RParen => { },
        x => return Err(BadToken(format!("Expected closing paren, but found {}", x)))
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
        Data(x) => x,
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
                Some(Ok(x)) => return Err(BadToken(format!("Expected end of tokenstream, but found {}", x))),
                Some(Err(f)) => return Err(f)
            },
            Some(Ok(x)) => return Err(BadToken(format!("Expected (, ), or Program but found {}", x))),
            Some(Err(f)) => return Err(f),
            None => return Err(UnbalancedParens)
        }
    }
    Ok(Schedule::new(&name, progs))
}
