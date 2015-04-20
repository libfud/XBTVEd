extern crate regex;

use self::regex::Regex;

pub use self::tokenize::{TokenStream, MaybeToken};
pub use super::schedule::Schedule;
pub use super::program::Source;
pub use super::tags::{TagType, Tags};
use self::translate::translate;
use std::fmt;

mod tokenize;
mod translate;

#[derive(Clone, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Data(String),
    Time(usize),
    TagKind(TagType),
    List,
    Instr,
    Play,
    Local,
    Network,
    Tag,
    Prog,
    Sched
}

impl fmt::Display for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "{}", match *self {
            Token::LParen => "(".to_string(),
            Token::RParen => ")".to_string(),
            Token::Data(ref x) => format!("\"{}\"", x),
            Token::Time(x) => x.to_string(),
            Token::TagKind(ref x) => x.to_string(),
            Token::List => "list".to_string(),
            Token::Instr => "instr".to_string(),
            Token::Play => "play".to_string(),
            Token::Local => "local".to_string(),
            Token::Network => "network".to_string(),
            Token::Tag => "tag".to_string(), 
            Token::Prog => "program".to_string(),
            Token::Sched => "schedule".to_string()
        }));
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum ParseError {
    BadToken(String),
    UnbalancedParens,
    BadAction,
    BadTime,
}

impl fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "{}", match *self {
            ParseError::BadToken(ref x) => x.clone(),
            ParseError::UnbalancedParens => "Unbalanced parens!".to_string(),
            ParseError::BadAction => "That action is inappropriate in tthis context.".to_string(),
            ParseError::BadTime => "Improper or impossible time.".to_string(),
        }));
        Ok(())
    }
}

pub fn is_paren(expr: &str) -> MaybeToken<Token, ParseError> {
    match expr.chars().next().unwrap() {
        '(' => (Some(Ok(Token::LParen)), 1),
        ')' => (Some(Ok(Token::RParen)), 1),
        _   => (None, 0)
    }
}

pub fn is_paren_rev(expr: &str) -> MaybeToken<Token, ParseError> {
    if let Some(c) = expr.chars().rev().next() {
        match c {
            '(' => (Some(Ok(Token::LParen)), 1),
            ')' => (Some(Ok(Token::RParen)), 1),
            _   => (None, 0)
        }
    } else {
        (None, 0)
    }
}

pub fn is_keyword(expr: &str) -> MaybeToken<Token, ParseError> {
    if expr.starts_with("list ") {
        (Some(Ok(Token::List)), 4)
    } else if expr.starts_with("tags ") {
        (Some(Ok(Token::Tag)), 4)
    } else if expr.starts_with("play") { 
        (Some(Ok(Token::Play)), 4)
    } else if expr.starts_with("local ") {
        (Some(Ok(Token::Local)), 5)
    } else if expr.starts_with("network ") {
        (Some(Ok(Token::Network)), 7)
    } else if expr.starts_with("program ") {
        (Some(Ok(Token::Prog)), 7)
    } else if expr.starts_with("schedule ") {
        (Some(Ok(Token::Sched)), 8)
    } else if expr.starts_with("instr ") {
        (Some(Ok(Token::Instr)), 5)
    } else {
        (None, 0)
    }
}

pub fn is_keyword_rev(expr: &str) -> MaybeToken<Token, ParseError> {
    if expr.ends_with("list") {
        (Some(Ok(Token::List)), 4)
    } else if expr.ends_with("tags") {
        (Some(Ok(Token::Tag)), 4)
    } else if expr.ends_with("play") { 
        (Some(Ok(Token::Play)), 4)
    } else if expr.ends_with("local") {
        (Some(Ok(Token::Local)), 5)
    } else if expr.ends_with("network") {
        (Some(Ok(Token::Network)), 7)
    } else if expr.ends_with("program") {
        (Some(Ok(Token::Prog)), 7)
    } else if expr.ends_with("schedule") {
        (Some(Ok(Token::Sched)), 8)
    } else if expr.ends_with("instr") {
        (Some(Ok(Token::Instr)), 5)
    } else {
        (None, 0)
    }
}

pub fn is_data(expr: &str) -> MaybeToken<Token, ParseError> {
    if expr.starts_with("\"") {
        /* c == '"' is kind of a bad assumption, but I haven't really encountered *
         * many quotes in filenames. I should probably come back and try to find  *
         * a better solution later.
         * Another failure case, although unlikely, is double quotes in URLs.
         * However, those are illegal characters and _should_ cause an error anyway.*/

        let close = match expr[1..].find('"') {
            Some(x) => x + 1,
            None => return (Some(Err(ParseError::BadToken("Cannot find closing quote!".to_string()))), 0)
        };
        let location = expr[1 .. close].to_string();
        let advance = close + 1;
        (Some(Ok(Token::Data(location))), advance)
    } else {
        (None, 0)
    }
}

pub fn is_data_rev(expr: &str) -> MaybeToken<Token, ParseError> {
    if expr.ends_with("\"") {
        let close = match expr[..expr.len() - 1].rfind('"') {
            Some(x) => x + 1,
            None => return (Some(Err(ParseError::BadToken("Cannot find closing quote!".to_string()))), 0)
        };

        let location = expr[close .. expr.len() - 1].to_string();
        let advance = location.len() + 2;
        (Some(Ok(Token::Data(location))), advance)
    } else {
        (None, 0)
    }
}

#[test]
fn loc_rev_test() {
    assert!(is_data_rev("(foo \"bar\"") == (Some(Ok(Token::Data("bar".to_string()))), 5));
    assert!(is_data_rev("(foo bar\"") == (Some(Err(ParseError::BadToken(
        "Cannot find closing quote!".to_string()))), 0));
    assert!(is_data_rev("foo bar") == (None, 0));
}

pub fn is_time(expr: &str) -> MaybeToken<Token, ParseError> {
    let re = match Regex::new(r"^\d{2}:\d{2}:\d{2}") {
        Ok(re) => re,
        Err(f) => panic!(f)
    };
    if re.is_match(expr) {
        let hours: usize = match expr[..2].parse() {
            Ok(x) => x,
 //           Err(_) => return (Some(Err(ParseError::BadToken(bad_time))), 0)
            Err(f) => panic!(f)
        };
        let minutes: usize = match expr[3..5].parse() {
            Ok(x) => x,
            Err(f) => panic!(f)
        };
        let seconds: usize = match expr[6..8].parse() {
            Ok(x) => x,
            Err(f) => panic!(f)
        };
        (Some(Ok(Token::Time(hours * 3600 + minutes * 60 + seconds))), 8)
    } else {
        (None, 0)
    }
}

#[test]
fn time_test() {
    assert!(is_time("00:00:00") == (Some(Ok(Token::Time(0))), 8));
    assert!(is_time("01:30:05") == (Some(Ok(Token::Time(90 * 60 + 5))), 8));
    assert!(is_time("ab:cd:ef") == (None, 0));
}

pub fn is_time_rev(expr: &str) -> MaybeToken<Token, ParseError> {
    let re = match Regex::new(r"^\d{2}:\d{2}:\d{2}") {
        Ok(re) => re,
        Err(f) => panic!(f)
    };

    if expr.len() > 8 && re.is_match(&expr[expr.len() - 8 ..]) {
        let hours: usize = match expr[expr.len() - 8 .. expr.len() - 6].parse() {
            Ok(x) => x,
            Err(f) => panic!(f)
        };
        let minutes: usize = match expr[expr.len() - 5 .. expr.len() - 3].parse() {
            Ok(x) => x,
            Err(f) => panic!(f)
        };
        let seconds: usize = match expr[expr.len() - 2 ..].parse() {
            Ok(x) => x,
            Err(f) => panic!(f)
        };
        (Some(Ok(Token::Time(hours * 3600 + minutes * 60 + seconds))), 8)
    } else {
        (None, 0)
    }
}

#[test]
fn time_rev_test() {
    assert!(is_time_rev("(play 00:00:00") == (Some(Ok(Token::Time(0))), 8));
    assert!(is_time_rev("(play 00:00:0a") == (None, 0));
    assert!(is_time_rev("(play 01:30:05") == (Some(Ok(Token::Time(90 * 60 + 5))), 8));
}

pub fn is_tag(expr: &str) -> MaybeToken<Token, ParseError> {
    if expr.starts_with(":") {
        match expr.find(|c: char| c.is_whitespace()) {
            Some(x) => match expr[1..x].parse::<TagType>() {
                Ok(tagtype) => (Some(Ok(Token::TagKind(tagtype))), x),
                Err(_) => (Some(Err(ParseError::BadToken(format!("Badly formatted tag. {}", expr)))), 0)
            },
            None => (Some(Err(ParseError::BadToken(format!("Badly formatted tag. {}", expr)))), 0)
        }
    } else {
        (None, 0)
    }
}

pub fn is_tag_rev(expr: &str) -> MaybeToken<Token, ParseError> {
    match expr.rfind(|c: char| c == ':') {
        Some(x) => match expr[x + 1..].parse::<TagType>() {
            Ok(tagtype) => (Some(Ok(Token::TagKind(tagtype))), x),
            Err(_) => (Some(Err(ParseError::BadToken("Badly formatted tag.".to_string()))), 0)
        },
        None => (None, 0)
    }
}

pub fn parse(s: &str) -> Result<Schedule, ParseError> {
    let next_rules: Vec<fn(&str) -> MaybeToken<Token, ParseError>> = 
        vec!(is_paren, is_keyword, is_data, is_time, is_tag);
    let back_rules: Vec<fn(&str) -> MaybeToken<Token, ParseError>> =
        vec!(is_paren_rev, is_keyword_rev, is_data_rev, is_time_rev, is_tag);

    let mut tokens = TokenStream::new(s, next_rules, back_rules,
                                      ParseError::BadToken("Unrecognized token".to_string()));

    translate(&mut tokens)
}


#[test]
fn toy_example() {
    let should_work = 
"(schedule \"foo\" (program (local \"foo\") (tags :director \"Bar Baz\")
 (instr (play 00:00:00 00:00:00))))";

    let schedule_failure1 = 
"(sked \"foo\" (program (local \"foo\") (tags :director \"Bar Baz\")
 (instr (play 00:00:00 00:00:00))))";

    let schedule_failure2 = 
"(program (tags director=\"Bar Baz\")
 (instr (play 00:00:00 00:00:00))))";

    let missing_paren_failure1 =
"(schedule \"foo\" (program (local \"foo\") (tags :director \"Bar Baz\")
 (instr (play 00:00:00 00:00:00)))";

    let missing_paren_failure2 =
"(schedule \"foo\" (program (local \"foo\") (tags :director \"Bar Baz\")
 (instr play 00:00:00 00:00:00))))"; 
        
    let bad_tag_failure = 
"(schedule \"foo\" (program (local \"foo\") (tags :dierekteur \"Bar Baz\")
 (instr (play 00:00:00 00:00:00))))";

    let no_tags = 
"(schedule \"foo\" (program (local \"foo\") (instr (play 00:00:00 00:00:00))))";

    let should_work2 = 
"(schedule \"foo\" (program (local \"foo\")
 (tags :director \"Bar Baz\" :cast (list \"Foo\" \"Bar\" \"Baz\"))
 (instr (play 00:00:00 00:00:00))))";


    if let Err(f) = parse(should_work) {
        println!("{}", f);
        panic!(f)
    }
    assert!(parse(should_work).is_ok() );

    assert!(parse(schedule_failure1).is_err() );

    assert!(parse(schedule_failure2).is_err() );

    assert!(parse(missing_paren_failure1).is_err() );

    assert!(parse(missing_paren_failure2).is_err() );

    assert!(parse(bad_tag_failure).is_err() );

    assert!(parse(should_work2).is_ok());

    if let Err(f) =  parse(no_tags) {
        panic!(f)
    }
}
