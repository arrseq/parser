#![feature(round_char_boundary)]

use std::cell::RefCell;
use std::rc::Rc;
use parser::{Parser, Parsable};

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

#[derive(Debug)]
struct Hi {}

impl Parsable for Hi {
    fn parse(parser: &mut Parser) -> Self {
        parser.expect_char('h').expect("expected h");
        parser.expect_char('i').expect("expected i");
        
        Self {}
    }
}

impl Parsable for Operator {
    fn parse(parser: &mut Parser) -> Self {
        parser.parse::<Hi>();
        
        if let Ok(_) = parser.expect_char('+') { Self::Add }
        else if let Ok(_) = parser.expect_char('-') { Self::Subtract }
        else if let Ok(_) = parser.expect_char('*') { Self::Multiply }
        else if let Ok(_) = parser.expect_char('/') { Self::Divide }
        else { Operator::Add }
    }
}

fn main() {
    let source = r#"hi*"#;
    let mut builder = Parser::new(source);
    
    dbg!(builder.parse_to_char('*'));
    // dbg!(builder.parse::<Operator>());
}