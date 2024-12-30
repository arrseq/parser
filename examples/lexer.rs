use std::ops::Deref;
use parser::{InternString, Parsable, Parser};
use parser::error::Error;
use parser::string::TokenGuard;

#[derive(Debug, Clone, Copy)]
enum Token {
    Hello,
    Plus,
    World,
    Identifier
}

fn documentation_example(parser: &mut Parser<Token>) {
    let ir_string = parser.parse_while(|c| c.is_alphabetic());

    // to either match to a token and get it or create as a new token.
    // either returns an InternString which is independent and valid.

    // internal steps:
    // -   get an entry for the string in question
    // -   if its vacant, run the token callback so the user can internalize it and set a token.
    // -   if its occupied, do nothing
    // in the end an InternString will be returned that references the string and its token.

    // loss for performance:
    // Do any computation necessary for a ParserString in that phase, you should only 
    // turn a ParserString into an InternString when you need to store it in a struct. Because 
    // reading an InternString isn't infinitely cheap and wasteful when done in scale. 
    // 
    // IndexString is still just a number however, so it is very cheap but dont be wasteful.
    
    let i_str = ir_string.try_internalize(|| None);
}

struct Data {
    hello: InternString<Token>
}

#[derive(Debug)]
struct SomeAst {
    first_word: InternString<Token>
}

impl Parsable for SomeAst {
    type Error = ();
    type Token = Token;
    type Data = ();

    fn parse(parser: &mut Parser<Self::Token>, data: &mut Self::Data) -> Result<Self, Error<Self::Error>> {
        let mut ps = parser.parse_while(|c| c.is_alphabetic());
        
        Ok(SomeAst {
            first_word: ps.try_internalize(|| Some(Token::Identifier)).unwrap()
        })
    }
}

fn main() {
    let source = "world+world";
    let mut parser = Parser::<Token>::new(source);
    
    parser.internalize("hello", Token::Hello).unwrap();
    parser.internalize("+", Token::Plus).unwrap();
    parser.internalize("world", Token::World).unwrap();
    
    let ast = parser.parse::<SomeAst>(&mut ()).unwrap();
    let fw = ast.first_word.token();
    dbg!(&*fw);
}