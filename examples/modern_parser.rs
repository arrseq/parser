#![feature(adt_const_params)]

use parser::{Node, Parsable, Parser};
use parser::error::Error;

#[derive(Debug)]
struct Ident {}

impl Parsable for Ident {
    type Error = ();
    
    fn parse(parser: &mut Parser) -> Result<Self, Error<Self::Error>> {
        parser.parse_while(|c| c.is_alphabetic());
        Ok(Self {})
    }
}

#[derive(Debug)]
struct WhiteSpace {}

impl Parsable for WhiteSpace {
    type Error = ();

    fn parse(parser: &mut Parser) -> Result<Self, Error<Self::Error>> {
        dbg!(parser.parse_while(|c| c.is_whitespace()));
        Ok(Self {})
    }
}

#[derive(Debug)]
struct Keyword {}

impl Parsable for Keyword {
    type Error = ();

    fn parse(parser: &mut Parser) -> Result<Self, Error<Self::Error>> {
        todo!()
    }
}

fn main() {
    let source = r#"hhhllo world"#;
    let mut parser = Parser::new(source);
    
    dbg!(parser.parse::<Ident>().unwrap().slice());
    dbg!(parser.parse::<WhiteSpace>().unwrap().slice());
    dbg!(parser.parse::<Ident>().unwrap().slice());
    dbg!(parser.parse::<WhiteSpace>().unwrap().slice());
}