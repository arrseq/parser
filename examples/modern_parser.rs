use parser::error::Error;
use parser::{Parsable, Parser};
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
enum KeywordToken {
    Function,
    Variable,
    Structure
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Keyword(KeywordToken)
}

#[derive(Debug, Clone, Copy)]
struct Keyword {
    word: KeywordToken
}

#[derive(Debug, Error)]
enum KeywordError {
    #[error("Invalid keyword")]
    InvalidKeyword
}

impl Parsable for Keyword {
    type Error = KeywordError;
    type Token = Token;
    type Data = ();

    fn parse(parser: &mut Parser<Self::Token>, data: &mut Self::Data) -> Result<Self, Error<Self::Error>> {
        let mut word = parser.parse_while(|character| character.is_alphabetic());
        let token = word.try_internalize(|word| Some(Token::Keyword(match word {
            "fn" => KeywordToken::Function,
            "let" => KeywordToken::Variable,
            "struct" => KeywordToken::Structure,
            _ => return None
        }))).unwrap();

        if let Some(token) = token {
            let Token::Keyword(keyword_token) = *token;
            Ok(Self { word: keyword_token })
        } else {
            Err(Error::new_syntax_temp())
        }
    }
}

fn main() {
    let source = "struct";
    let mut parser = Parser::new(source);

    let token = parser.parse::<Keyword>(&mut ()).expect("Failed to parse keyword token");
    dbg!(token.word);
}