use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, tag, take_until, take_while1},
    character::complete::{char, multispace0, one_of},
    combinator::opt,
    multi::{many0, separated_list0},
    sequence::delimited,
    IResult,
};
use nom_locate::{position, LocatedSpan};
use std::fmt;
type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq)]
struct PkgBuild<'a> {
    Tokens: Vec<Token<'a>>,
}

impl <'a> PkgBuild<'a> {
    pub fn from_str(input: &'a str) -> Self {
        let input = Span::new(input);
        let (_, tokens) = many0(parse_token)(input).unwrap();
        PkgBuild { Tokens: tokens }
    }
}

impl fmt::Display for PkgBuild <'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for token in &self.Tokens {
            writeln!(f, "{}", token)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum Token<'a> {
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

#[derive(Debug, PartialEq)]
struct Comment<'a> {
    comment: String,
    span: Span<'a>,
}

impl fmt::Display for Comment<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.comment)
    }
}

#[derive(Debug, PartialEq)]
enum AssignmentValue {
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

#[derive(Debug, PartialEq)]
enum Value {
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

fn parse_comment(input: Span) -> IResult<Span, Token> {
    let (input, pos) = position(input)?;
    let (input, _) = tag("#")(input)?;
    let (input, comment) = take_until("\n")(input)?;
    let (input, _) = tag("\n")(input)?;
    Ok((
        input,
        Token::Comment(Comment {
            comment: comment.to_string(),
            span: pos,
        }),
    ))
}

fn parse_assignment(input: Span) -> IResult<Span, Token> {
    let (input, _) = multispace0(input)?; // TODO: Throw a warning if there is whitespace before the assignment
    let (input, key) = take_until("=")(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, value) = alt((parse_array, parse_literal))(input)?;
    Ok((input, Token::Assignment(key.to_string(), value)))
}

fn parse_literal(input: Span) -> IResult<Span, AssignmentValue> {
    let (input, value) = alt((parse_doublequoted, parse_singlequoted, parse_unquoted))(input)?;
    let (input, _) = multispace0(input)?; // remove trailing whitespaces and newlines
    Ok((input, AssignmentValue::Literal(value)))
}

fn parse_array(input: Span) -> IResult<Span, AssignmentValue> {
    let (input, _) = tag("(")(input)?;
    let (input, values) = separated_list0(
        tag(" "),
        alt((parse_doublequoted, parse_singlequoted, parse_unquoted)),
    )(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = tag("\n")(input)?;
    Ok((input, AssignmentValue::Array(values)))
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

// function can be defined as
// ```bash
// fname () compound-command
// ```
// or
// ```bash
// function fname [()] compound-command
// ```
fn parse_function(input: Span) -> IResult<Span, Token> {
    let (input, _) = tag("function ")(input)?;
    let (input, name) = take_until(" ")(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, args) = opt(delimited(char('('), is_not(")"), char(')')))(input)?;
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = tag("\n")(input)?;
    let args = match args {
        Some(args) => args.to_string(),
        None => "".to_string(),
    };
    Ok((input, Token::Function(name.to_string(), args)))
}

fn parse_token(input: Span) -> IResult<Span, Token> {
    alt((parse_comment, parse_assignment, parse_function))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_comment() {
        let input = Span::new("# this is a comment\npkgname=rust\n");
        let (input, token) = parse_comment(input).unwrap();
        assert_eq!(token.to_string(), "# this is a comment");
        assert_eq!(input.to_string(), "pkgname=rust\n");
    }

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

    #[test]
    fn test_parse_function() {
        let input = Span::new("function fname ()\n");
        let (input, token) = parse_function(input).unwrap();
        assert_eq!(token.to_string(), "function fname ()");
        assert_eq!(input.to_string(), "");
    }

    #[test]
    fn parse_pkgbuild() {
        let input = r#"pkgname=rust
pkgver=1.51.0
pkgrel=1
# Comment out of nowhere
arch=('x86_64' 'aarch64')
"#;
        let parsed_pkgbuild = PkgBuild::from_str(input);
        assert_eq!(parsed_pkgbuild.to_string(), input);
    }
}
