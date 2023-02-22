use nom::{
    bytes::complete::{tag, take_until},
    IResult,
};

use nom_locate::position;
use std::fmt;

use super::pkgbuild::Span;
use super::token::Token;

#[derive(Debug, PartialEq)]
pub struct Comment<'a> {
    comment: String,
    span: Span<'a>,
}

impl fmt::Display for Comment<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.comment)
    }
}

impl Comment<'_> {
    pub fn parse(input: Span) -> IResult<Span, Token> {
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
}

#[cfg(test)]
mod comment_tests {
    use super::*;

    #[test]
    fn test_parse_comment() {
        let input = Span::new("# this is a comment\npkgname=rust\n");
        let (input, token) = Comment::parse(input).unwrap();
        assert_eq!(token.to_string(), "# this is a comment");
        assert_eq!(input.to_string(), "pkgname=rust\n");
    }
}
