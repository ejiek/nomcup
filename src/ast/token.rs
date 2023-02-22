use nom::{
    branch::alt,
    IResult,
};

use std::fmt;

use super::comment::Comment;
use super::assignment::{AssignmentValue, parse_assignment};
use super::function::parse_function;
use super::pkgbuild::Span;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Comment(Comment<'a>),
    Assignment(String, AssignmentValue),
    Function(String, String),
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Comment(c) => write!(f, "{}", c),
            Token::Assignment(k, v) => write!(f, "{}={}", k, v),
            Token::Function(k, v) => write!(f, "function {} ({})", k, v),
        }
    }
}

impl Token<'_> {
    pub fn parse(input: Span) -> IResult<Span, Token> {
        alt((Comment::parse, parse_assignment, parse_function))(input)
    }
}

