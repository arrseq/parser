use crate::{Parsable, Parser};
use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Space;

impl Parsable for Space {
    type Error = ();
    type Token = ();
    type Data = ();

    fn parse(parser: &mut Parser<Self::Token>, _: &mut Self::Data) -> Result<Self, Error<Self::Error>> {
        parser.parse_while(|character| character.is_whitespace());
        Ok(Self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line;

impl Parsable for Line {
    type Error = ();
    type Token = ();
    type Data = ();

    fn parse(parser: &mut Parser<Self::Token>, _: &mut Self::Data) -> Result<Self, Error<Self::Error>> {
        parser.parse_while(|character| character.is_whitespace() && character != '\n');
        Ok(Self)
    }
}