use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::multispace0,
    multi::separated_list0,
    IResult,
};

use std::fmt;

use super::value::Value;
use super::pkgbuild::Span;
use super::token::Token;

#[derive(Debug, PartialEq)]
pub enum AssignmentValue {
    Literal(Value),
    Array(Vec<Value>),
}

impl fmt::Display for AssignmentValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssignmentValue::Literal(v) => write!(f, "{}", v),
            AssignmentValue::Array(v) => {
                write!(f, "(")?;
                for (i, val) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, ")")
            }
        }
    }
}

pub fn parse_assignment(input: Span) -> IResult<Span, Token> {
    let (input, _) = multispace0(input)?; // TODO: Throw a warning if there is whitespace before the assignment
    let (input, key) = take_until("=")(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, value) = alt((parse_array, parse_literal))(input)?;
    Ok((input, Token::Assignment(key.to_string(), value)))
}

fn parse_literal(input: Span) -> IResult<Span, AssignmentValue> {
    let (input, value) = Value::parse(input)?;
    let (input, _) = multispace0(input)?; // remove trailing whitespaces and newlines
    Ok((input, AssignmentValue::Literal(value)))
}

fn parse_array(input: Span) -> IResult<Span, AssignmentValue> {
    let (input, _) = tag("(")(input)?;
    let (input, values) = separated_list0(
        tag(" "),
        Value::parse,
    )(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = tag("\n")(input)?;
    Ok((input, AssignmentValue::Array(values)))
}

#[cfg(test)]
mod assignment_tests{
    use super::*;


    #[test]
    fn test_parse_literal() {
        let input = Span::new("pkgname=rust\n");
        let (input, token) = parse_assignment(input).unwrap();
        assert_eq!(token.to_string(), "pkgname=rust");
        assert_eq!(input.to_string(), "");
    }

    #[test]
    fn test_parse_array() {
        let input = Span::new("arch=('x86_64' 'aarch64')\n");
        let (input, token) = parse_assignment(input).unwrap();
        assert_eq!(token.to_string(), "arch=('x86_64' 'aarch64')");
        assert_eq!(input.to_string(), "");
    }

}

