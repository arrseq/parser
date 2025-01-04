use std::ops::Deref;
use parser::{Parsable, Parser};
use parser::error::Error;

#[derive(Debug, Clone, Copy)]
enum OperatorToken {
    Plus,
    Minus
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Operator(OperatorToken),
    Whitespace
}

struct Number { value: usize }

impl Parsable for Number {
    type Error = ();
    type Token = Token;
    type Data = ();

    fn parse(parser: &mut Parser<Self::Token>, data: &mut Self::Data) -> Result<Self, Error<Self::Error>> {
        let number = parser.parse_while(|char| char.is_alphanumeric());
        Ok(Number { value: number.parse().unwrap() })
    }
}

struct Whitespace {}

impl Parsable for Whitespace {
    type Error = ();
    type Token = Token;
    type Data = ();

    fn parse(parser: &mut Parser<Self::Token>, data: &mut Self::Data) -> Result<Self, Error<Self::Error>> {
        parser.parse_while(|char| char.is_whitespace());
        Ok(Whitespace {})
    }
}

struct Operator {
    operator: OperatorToken
}

impl Parsable for Operator {
    type Error = ();
    type Token = Token;
    type Data = ();

    fn parse(parser: &mut Parser<Self::Token>, data: &mut Self::Data) -> Result<Self, Error<Self::Error>> {
        let mut operator = parser.parse_while(|char| matches!(char, '+' | '-'));
        if operator.len() != 1 {
            panic!("expected operator to be exactly one character");
        }
        
        let token = match operator.token().as_deref().cloned() {
            Some(t) => t,
            None => operator.try_internalize(|| )
        }
        
        dbg!(&*operator);
        let str = operator.try_internalize(|| None).unwrap();
        
        todo!()
    }
}

fn main() {
    let source = "+";
    let mut parser = Parser::new(source);
    
    let operator = parser.parse::<Operator>(&mut ());
}