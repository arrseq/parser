use parser::{Builder, Parsable};

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl Parsable for Operator {
    fn parse(mut builder: Builder) -> Self {
        builder.test(|char| char == 'a');
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
    let mut builder = Builder::new(source);
    
    dbg!(builder.parse::<Operator>());
}