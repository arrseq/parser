use parser::{Node, Parsable, Parser};
use parser::error::Error;

struct VariableDeclaration {
    inner: Option<Box<Node<VariableDeclaration>>>
}

impl Parsable for VariableDeclaration {
    type Error = ();
    
    fn parse(parser: &mut Parser) -> Result<Self, Error<Self::Error>> {
        let out = if let Ok(_) = parser.expect_char('h') {
            Some(Box::new(parser.parse::<VariableDeclaration>().unwrap()))
        } else { None };
        Ok(Self { inner: out })
    }
}

fn main() {
    let source = r#"hhhllo + world"#;
    let mut parser = Parser::new(source);
    
    // Internal behavior:
    // - create parser fork with start = self.start.
    // - call parse with fork
    // - take length of fork parser and add it to self.length
    // - end
    let res = parser.parse::<VariableDeclaration>().unwrap();
    dbg!(res.slice());
}