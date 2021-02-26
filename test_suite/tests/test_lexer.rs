use libastral::lexer::{tokenize, Token, TokenKind::*};

#[test]
fn tokenize_empty_input() {
    let expected = vec![];
    assert_eq!(tokenize(""), expected);
}

#[test]
fn tokenize_whitespace_input() {
    let expected = vec![Token::new(Whitespace, 1), Token::new(Whitespace, 2)];
    assert_eq!(tokenize(" \t"), expected);
}

#[test]
fn tokenize_unknown_input() {
    let expected = vec![Token::new(Unknown { char: '@' }, 1)];
    assert_eq!(tokenize("@"), expected);
}
