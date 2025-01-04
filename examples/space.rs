use parser::node::Space;
use parser::node::space::Line;
use parser::Parser;

fn main() {
    let source = "    \t\n\t hi";
    let mut parser = Parser::new(source);

    let result = parser.parse::<Space>(&mut ()).expect("Expected whitespace");
    dbg!(result.slice());
}