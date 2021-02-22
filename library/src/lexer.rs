#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Whitespace,
}

pub fn tokenize(_input: &str) -> Vec<Token> {
    todo!();
}
