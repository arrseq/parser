use parser::{Parser, Parsable, Node};

#[derive(Debug)]
struct VariableDeclaration {
    // name: Node<Ident>
}

#[derive(Debug)]
pub enum VarError {
    ExpectedLet
}

#[derive(Debug)]
struct Ident {}

impl Parsable for Ident {
    type Error = ();

    fn parse(parser: &mut Parser) -> Result<Self, Self::Error> {
        parser.parse_while(|char| char != ' ');
        Ok(Ident {})
    }
}

impl Parsable for VariableDeclaration {
    type Error = VarError;
    
    fn parse(parser: &mut Parser) -> Result<Self, VarError> {
        let keyword = parser.parse_while(|char| char != ' ');
        if keyword != "let" { return Err(VarError::ExpectedLet) }
        // parser.expect_char(' ');
        // let name = parser.parse::<Ident>().unwrap();
        // Ok(Self { name })
        
        Ok(Self {})
    }
}

fn main() {
    let source = r#"let x = 10;"#;
    let mut parser = Parser::new(source);

    let var = parser.parse::<VariableDeclaration>().unwrap();
    dbg!(var.slice());
}