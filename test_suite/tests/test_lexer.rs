use libastral::lexer::{NumberKind, Token, TokenKind::*, tokenize};

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

#[test]
fn tokenize_some_identifiers() {
    let expected = vec![
        Token::new(Ident, 1),
        Token::new(Whitespace, 2),
        Token::new(Ident, 4),
    ];

    assert_eq!(tokenize("a a'"), expected);
}

#[test]
fn tokenize_some_binary_literal() {
    let expected = vec![
        Token::new(Number { kind: NumberKind::Binary }, 3),
        Token::new(Whitespace, 4),
        Token::new(Number { kind: NumberKind::Binary }, 8),
    ];

    assert_eq!(tokenize("0b0 0_b1"), expected);
}

#[test]
fn tokenize_some_generic_number_literal() {
    let expected = vec![
        Token::new(Number { kind: NumberKind::Generic }, 3),
        Token::new(Whitespace, 4),
        Token::new(Number { kind: NumberKind::Generic }, 8),
    ];

    assert_eq!(tokenize("123 0123"), expected);
}
