use parser::{Parsable, Parser, ParserString};
use parser::error::Error;

#[derive(Debug, Clone)]
enum Token {
    Identifier,
    ProtocolDiv,
    Directory,
    Dot
}

#[derive(Debug, Clone)]
struct Protocol;

impl Parsable for Protocol {
    type Error = ();
    type Token = Token;
    type Data = ();

    fn parse(parser: &mut Parser<Self::Token>, data: &mut Self::Data) -> Result<Self, Error<Self::Error>> {
        let mut symbols = parser.parse_while(|character| matches!(character, ':' | '/'));
        let token = symbols.try_internalize(|_| None).unwrap();
        
        if let None = token {
            Err(Error::new_syntax_temp())
        } else {
           Ok(Self) 
        }
    }
}

#[derive(Debug, Clone)]
struct Word { 
    word: ParserString<Token>, 
    protocol_word: bool
}

impl Parsable for Word {
    type Error = ();
    type Token = Token;
    type Data = ();

    fn parse(parser: &mut Parser<Self::Token>, data: &mut Self::Data) -> Result<Self, Error<Self::Error>> {
        let mut word = parser.parse_while(|character| character.is_alphabetic() || matches!(character, '_' | '-'));
        word.try_internalize(|_| Some(Token::Identifier));
        if word.len() == 0 {
            return Err(Error::new_syntax_temp());
        }
        
        Ok(Self {
            word,
            protocol_word: false
        })
    }
}

fn main() {
    let source = "hihi://rust.rs";
    let mut parser = Parser::<Token>::new(source);
    
    // setup parser interning
    parser.internalize("https", Token::Identifier).expect("Failed to intern");
    parser.internalize("http", Token::Identifier).expect("Failed to intern");
    parser.internalize("ws", Token::Identifier).expect("Failed to intern");
    parser.internalize("wss", Token::Identifier).expect("Failed to intern");
    
    parser.internalize("://", Token::ProtocolDiv).expect("Failed to intern");
    parser.internalize("/", Token::Directory).expect("Failed to intern");
    parser.internalize(".", Token::Dot).expect("Failed to intern");
    
    // parse url
    let start = parser.parse::<Word>(&mut ()).unwrap();
    dbg!(&*start.word.token().unwrap());
    let proto = parser.parse::<Protocol>(&mut ()).unwrap();
    dbg!(proto.slice());
}