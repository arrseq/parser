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
    type Data = ();
    
    fn parse(parser: &mut Parser, data: &mut Self::Data) -> Result<Self, Self::Error> {
        parser.parse_while(|char| char != ' ');
        Ok(Ident {})
    }
}

impl Parsable for VariableDeclaration {
    type Error = VarError;
    type Data = ();
    
    fn parse(parser: &mut Parser, data: &mut Self::Data) -> Result<Self, VarError> {
        let keyword = parser.parse_while(|char| char != ' ');
        if keyword != "let" { return Err(VarError::ExpectedLet) }
        parser.expect_char(' ');
        let name = parser.parse::<Ident>(data).unwrap();
        
        dbg!(name);
        
        Ok(Self {})
    }
}

fn main() {
    let source = r#"let x = 10;"#;
    let mut parser = Parser::new(source);

    let var = parser.parse::<VariableDeclaration>(&mut ()).unwrap();
    dbg!(var.slice());
}

// 
//  
// 
// 
// 
// 