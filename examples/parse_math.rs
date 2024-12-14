#![feature(round_char_boundary)]

use std::cell::RefCell;
use std::rc::Rc;
use parser::{Parser, Parsable};
use parser::span::{BranchSpan, Span};

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl Parsable for Operator {
    fn parse(mut parser: &mut Parser) -> Self {
        parser.test(|char| char == 'a');
        Operator::Add
    }
}

// #[derive(CharacterNode)]
// enum Operator {
//     #[character("+")]
//     Add,
//     #[character("-")]
//     Subtract,
//     #[character("*")]
//     Multiply,
//     #[character("/")]
//     Divide
// }

fn main() {
    // let source = r#"abc"#;
    // let mut builder = Parser::new(source);
    // 
    // dbg!("Бbc".ceil_char_boundary(1));
    // dbg!(builder.parse::<Operator>());

    let source = "Бbcd";
    let mut span = Rc::new(RefCell::new(Span::new(source.char_indices())));
    span.borrow_mut().expand(1);
    
    let inner = span.derive();
    inner.borrow_mut().expand(2).expect("WHAT");
    dbg!(&source[span.borrow().slice_bounds[0]..span.borrow().slice_bounds[1]]);
    dbg!(&source[inner.borrow().slice_bounds[0]..inner.borrow().slice_bounds[1]]);
}