use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, tag, take_until, take_while1},
    character::complete::{char, multispace0, one_of},
    combinator::opt,
    multi::{many0, separated_list0},
    sequence::delimited,
    IResult,
};
use nom_locate::LocatedSpan;
type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq)]
enum Token {
    Comment(String),
    Assignment(String, AssignmentValue),
    Function(String, String),
}

#[derive(Debug, PartialEq)]
enum AssignmentValue {
    Literal(Value),
    Array(Vec<Value>),
}

#[derive(Debug, PartialEq)]
enum Value {
    Doublequoted(String),
    Singlequoted(String),
    Unquoted(String),
}

fn parse_comment(input: Span) -> IResult<Span, Token> {
    let (input, _) = tag("#")(input)?;
    let (input, comment) = take_until("\n")(input)?;
    let (input, _) = tag("\n")(input)?;
    Ok((input, Token::Comment(comment.to_string())))
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

fn parse(input: &str) -> IResult<Span, Vec<Token>> {
    let input = Span::new(input);
    many0(parse_token)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_comment() {
        let input = Span::new("# this is a comment\npkgname=rust\n");
        let expected = Token::Comment(" this is a comment".to_string());
        let (input, token) = parse_comment(input).unwrap();
        assert_eq!(token, expected);
        assert_eq!(input.to_string(), "pkgname=rust\n");
    }

    #[test]
    fn test_parse_literal() {
        let input = Span::new("pkgname=rust\n");
        let expected = Token::Assignment(
            "pkgname".to_string(),
            AssignmentValue::Literal(Value::Unquoted("rust".to_string())),
        );
        let (input, token) = parse_assignment(input).unwrap();
        assert_eq!(token, expected);
        assert_eq!(input.to_string(), "");
    }

    #[test]
    fn test_parse_array() {
        let input = Span::new("arch=('x86_64' 'aarch64')\n");
        let expected = Token::Assignment(
            "arch".to_string(),
            AssignmentValue::Array(vec![
                Value::Singlequoted("x86_64".to_string()),
                Value::Singlequoted("aarch64".to_string()),
            ]),
        );
        let (input, token) = parse_assignment(input).unwrap();
        assert_eq!(token, expected);
        assert_eq!(input.to_string(), "");
    }

    #[test]
    fn test_parse_function() {
        let input = Span::new("function fname ()\n");
        let expected = Token::Function("fname".to_string(), "".to_string());
        let (input, token) = parse_function(input).unwrap();
        assert_eq!(token, expected);
        assert_eq!(input.to_string(), "");
    }

    #[test]
    fn parse_pkgbuild() {
        let input = r#"pkgname=rust
pkgver=1.51.0
pkgrel=1
arch=('x86_64' 'aarch64')
"#;
        let expected = vec![
            Token::Assignment(
                "pkgname".to_string(),
                AssignmentValue::Literal(Value::Unquoted("rust".to_string())),
            ),
            Token::Assignment(
                "pkgver".to_string(),
                AssignmentValue::Literal(Value::Unquoted("1.51.0".to_string())),
            ),
            Token::Assignment(
                "pkgrel".to_string(),
                AssignmentValue::Literal(Value::Unquoted("1".to_string())),
            ),
            Token::Assignment(
                "arch".to_string(),
                AssignmentValue::Array(vec![
                    Value::Singlequoted("x86_64".to_string()),
                    Value::Singlequoted("aarch64".to_string()),
                ]),
            ),
        ];
        let (input, tokens) = parse(input).unwrap();
        assert_eq!(tokens, expected);
        assert_eq!(input.to_string(), "");
    }
}
