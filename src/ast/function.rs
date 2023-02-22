use nom::{
    bytes::complete::{is_not, tag, take_until},
    character::complete::char,
    combinator::opt,
    sequence::delimited,
    IResult,
};

use super::token::Token;
use super::pkgbuild::Span;


// function can be defined as
// ```bash
// fname () compound-command
// ```
// or
// ```bash
// function fname [()] compound-command
// ```
pub fn parse_function(input: Span) -> IResult<Span, Token> {
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

#[cfg(test)]
mod test_function {
    use super::*;

    #[test]
    fn test_parse_function() {
        let input = Span::new("function fname ()\n");
        let (input, token) = parse_function(input).unwrap();
        assert_eq!(token.to_string(), "function fname ()");
        assert_eq!(input.to_string(), "");
    }
}
