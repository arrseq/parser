use std::ops::Deref;
use parser::{Parsable, Parser, ParserString};
use parser::error::Error;
use parser::string::TokenGuard;

#[derive(Debug, Clone, Copy)]
enum Token {
    Hello,
    Plus,
    World,
    Identifier
}

struct Data {
    hello: ParserString<Token>
}

#[derive(Debug)]
struct SomeAst {
    first_word: ParserString<Token>
}

impl Parsable for SomeAst {
    type Error = ();
    type Token = Token;
    type Data = ();

    fn parse(parser: &mut Parser<Self::Token>, data: &mut Self::Data) -> Result<Self, Error<Self::Error>> {
        let mut ps = parser.parse_while(|c| c.is_alphabetic());
        ps.try_internalize(|s| Some(Token::Identifier)).unwrap();
        
        Ok(SomeAst {
            first_word: ps
        })
    }
}

fn main() {
    let source = "hello+world";
    let mut parser = Parser::<Token>::new(source);
    
    parser.internalize("hello", Token::Hello).unwrap();
    parser.internalize("+", Token::Plus).unwrap();
    parser.internalize("world", Token::World).unwrap();
    
    let mut ast = parser.parse::<SomeAst>(&mut ()).unwrap();
    let fw = ast.first_word.token().unwrap();
    dbg!(&*fw);
}