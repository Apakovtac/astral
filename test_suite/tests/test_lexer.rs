use libastral::lexer::{
    Lexer,
    Token,
    TokenKind::*,
    tokenize
};

#[test]
fn tokenize_empty_input() {
    let expected = vec![];
    assert_eq!(tokenize(""), expected);
}

#[test]
fn tokenize_whitespace_input() {
    let expected = vec![Token::new(Whitespace), Token::new(Whitespace)];
    assert_eq!(tokenize(" \t"), expected)
}

#[test]
fn lexer_implements_iterator_trait() {
    let lexer = Lexer::new("  ");

    for token in lexer {
        assert_eq!(Token::new(Whitespace), token);
    }
}
