use nom::multi::many0;

use nom_locate::LocatedSpan;
use std::fmt;
pub type Span<'a> = LocatedSpan<&'a str>;

use super::token::Token;

#[derive(Debug, PartialEq)]
pub struct PkgBuild<'a> {
    tokens: Vec<Token<'a>>,
}

impl <'a> PkgBuild<'a> {
    pub fn new(input: &'a str) -> Self {
        let input = Span::new(input);
        let (_, tokens) = many0(Token::parse)(input).unwrap();
        PkgBuild { tokens }
    }
}

impl fmt::Display for PkgBuild <'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for token in &self.tokens {
            writeln!(f, "{}", token)?;
        }
        Ok(())
    }
}

