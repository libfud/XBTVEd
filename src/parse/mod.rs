pub use self::tokenize::{TokenStream, MaybeToken};
pub use super::schedule::{Schedule, Source};
pub use super::tags::{TagType, Tags};
use self::translate::translate;
use std::fmt;

mod tokenize;
mod translate;

#[derive(Clone, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Instr,
    Play,
    Local,
    Network,
    Location(String),
    Time(usize),
    Tag,
    TagData(TagType, String),
    Prog,
    Sched
}

impl fmt::Display for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "{}", match *self {
            Token::LParen => "(".to_string(),
            Token::RParen => ")".to_string(),
            Token::Instr => "instr".to_string(),
            Token::Play => "play".to_string(),
            Token::Local => "local".to_string(),
            Token::Network => "network".to_string(),
            Token::Location(ref x) => x.clone(),
            Token::Time(x) => x.to_string(),
            Token::Tag => "tag".to_string(),
            Token::TagData(ref x, ref y) => format!("{}=\"{}\"", x, y),
            Token::Prog => "program".to_string(),
            Token::Sched => "schedule".to_string()
        }));
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
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

pub fn is_keyword(expr: &str) -> MaybeToken<Token, ParseError> {
    if expr.starts_with("tags ") {
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

pub fn is_location(expr: &str) -> MaybeToken<Token, ParseError> {
    if expr.starts_with("\"") {
        /* c == '"' is kind of a bad assumption, but I haven't really encountered *
         * many quotes in filenames. I should probably come back and try to find  *
         * a better solution later */

        let close = match expr.chars().skip(1).position(|c: char| c == '"') {
            Some(x) => x,
            None => return (Some(Err(ParseError::BadToken("Cannot find closing quote!".to_string()))), 0)
        };
        /* Close is an indexed position, but it's also offset by one since we skip the first *
         * character in the string. */
        let location = expr.slice_chars(1, close + 1).to_string();
        let advance = location.len() + 2; //quotation marks
        (Some(Ok(Token::Location(location))), advance)
    } else {
        (None, 0)
    }
}

pub fn is_time(expr: &str) -> MaybeToken<Token, ParseError> {
    if expr.len() >= 8 && expr.chars().skip(2).next().unwrap() == ':' 
        && expr.chars().skip(5).next().unwrap() == ':'
    {
        let hours: usize = match expr.chars().take(2).collect::<String>().parse() {
            Ok(x) => x,
            Err(_) => return (Some(Err(
                ParseError::BadToken("Unrecognized token. Time format is xx:yy::zz".to_string()))), 0)
        };
        let minutes: usize = match expr.chars().skip(3).take(2).collect::<String>().parse() {
            Ok(x) => x,
            Err(_) => return (Some(Err(
                ParseError::BadToken("Unrecognized token. Time format is xx:yy::zz".to_string()))), 0)
        };
        let seconds: usize = match expr.chars().skip(6).take(2).collect::<String>().parse() {
            Ok(x) => x,
            Err(_) => return (Some(Err(
                ParseError::BadToken("Unrecognized token. Time format is xx:yy::zz".to_string()))), 0)
        };
        (Some(Ok(Token::Time(hours * 3600 + minutes * 60 + seconds))), 8)
    } else {
        (None, 0)
    }
}

pub fn is_tagdata(expr: &str) -> MaybeToken<Token, ParseError> {
    let eq_pos = match expr.chars().position(|c: char| c == '=') {
        Some(x) => x,
        None => return (None, 0)
    };

    let tagname = match expr.chars().take(eq_pos).collect::<String>().parse::<TagType>() {
        Ok(x) => x,
        Err(_) => return (Some(Err(ParseError::BadToken("Badly formatted tag.".to_string()))), 0)
    };

    let rest = expr.chars().skip(eq_pos + 1).collect::<String>();

    let (tagdata, advance) = if rest.starts_with("\"") {
        let close = match rest.chars().skip(1).position(|c: char| c == '"') {
            Some(x) => x,
            None => return (Some(Err(ParseError::BadToken("Cannot find closing quote!".to_string()))), 0)
        };
        
        let tagdata = rest.slice_chars(1, close + 1).to_string();
        let advance = tagdata.len() + 2 + eq_pos + 1; //two double quotes, and add one for index of =
        (tagdata, advance)
    } else {
        return (Some(Err(ParseError::BadToken("Empty tags are not permitted".to_string()))), 0)
    };

    (Some(Ok(Token::TagData(tagname, tagdata))), advance)
}

pub fn parse(s: &str) -> Result<Schedule, ParseError> {
    let rules: Vec<fn(&str) -> MaybeToken<Token, ParseError>> = 
        vec!(is_paren, is_keyword, is_location, is_time, is_tagdata);

    let mut tokens = TokenStream::new(s.to_string(), rules, 
                                      ParseError::BadToken("Unrecognized token".to_string()));

    translate(&mut tokens)
}
