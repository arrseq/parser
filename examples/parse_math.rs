#![feature(round_char_boundary)]

use parser::{Parser, Parsable};

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
    let source = r#"abc"#;
    let mut builder = Parser::new(source);
    
    dbg!("Бbc".ceil_char_boundary(1));
    // dbg!(builder.parse::<Operator>());
}