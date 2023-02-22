use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, tag, take_while1},
    character::complete::one_of,
    IResult,
};

use std::fmt;

use super::pkgbuild::Span;

#[derive(Debug, PartialEq)]
pub enum Value {
    Doublequoted(String),
    Singlequoted(String),
    Unquoted(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Doublequoted(s) => write!(f, "\"{}\"", s),
            Value::Singlequoted(s) => write!(f, "'{}'", s),
            Value::Unquoted(s) => write!(f, "{}", s),
        }
    }
}

impl Value {
    pub fn parse(input: Span) -> IResult<Span, Value> {
        let (input, value) = alt((
                Self::parse_doublequoted,
                Self::parse_singlequoted,
                Self::parse_unquoted
        ))(input)?;
        Ok((input, value))
    }

    fn parse_singlequoted(input: Span) -> IResult<Span, Value> {
        let (input, _) = tag("'")(input)?;
        let (input, value) = escaped(is_not("'"), '\\', one_of("'"))(input)?;
        let (input, _) = tag("'")(input)?;
        Ok((input, Value::Singlequoted(value.to_string())))
    }

    fn parse_doublequoted(input: Span) -> IResult<Span, Value> {
        let (input, _) = tag("\"")(input)?;
        let (input, value) = escaped(is_not("\""), '\\', one_of("\""))(input)?;
        let (input, _) = tag("\"")(input)?;
        Ok((input, Value::Doublequoted(value.to_string())))
    }

    fn parse_unquoted(input: Span) -> IResult<Span, Value> {
        let (input, value) =
            take_while1(|c: char| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')(input)?;
        Ok((input, Value::Unquoted(value.to_string())))
    }
}
