use self::TokenKind::*;
use self::NumberKind::*;
use crate::cursor::Cursor;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    pub fn new(kind: TokenKind, len: usize) -> Token {
        Token { kind, len }
    }
}

#[derive(Debug, PartialEq)]
pub enum NumberKind {
    /// Any number that can be transformed to a specific type.
    Generic,

    /// Any number prefixed with '0b'.
    Binary,
}

// TODO: Don't store character data in TokenKind that can be found from the string.
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Whitespace,
    Ident,
    Number { kind: NumberKind },
    Eof,

    Unknown { char: char },
}

pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer {
            cursor: Cursor::new(input),
        }
    }

    fn next_token(&mut self) -> Token {
        match self.cursor.first() {
            ' ' | '\t' => self.eat_whitespace(),
            c if is_ident_start(c) => self.eat_ident(),
            c if is_digit_start(c) => self.eat_digit(),
            c => self.eat_unknown_char(c),
        }
    }

    fn eat_whitespace(&mut self) -> Token {
        self.cursor.next();
        Token::new(Whitespace, self.cursor.len_consumed())
    }

    fn eat_ident(&mut self) -> Token {
        self.cursor.next();
        self.cursor.next_while(is_still_ident);
        Token::new(Ident, self.cursor.len_consumed())
    }

    // FIXME: Not quite the right algorithm. This will tokenize `1a` as two different tokens. For now, we'll live with it.
    fn eat_digit(&mut self) -> Token {
        let first = self.cursor.first();
        self.cursor.next();
        self.cursor.next_while_char('_');
        let second = self.cursor.first();

        let kind = match (first, second) {
            ('0', 'b') => Binary,
            _ if is_still_digit(second) => Generic,
            _ => todo!("handle error recovery at this point"),
        };

        self.cursor.next();
        self.cursor.next_while_char('_');

        match kind {
            Generic => self.cursor.next_while(is_still_digit),
            Binary => self.cursor.next_while(is_still_binary),
        }

        Token::new(Number { kind }, self.cursor.len_consumed())
    }

    fn eat_unknown_char(&mut self, char: char) -> Token {
        self.cursor.next();
        Token::new(Unknown { char }, self.cursor.len_consumed())
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.cursor.is_eof() {
            return None;
        }

        Some(self.next_token())
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    Lexer::new(input).collect()
}

fn is_ident_start(char: char) -> bool {
    matches!(char, 'a'..='z' | '_')
}

fn is_still_ident(char: char) -> bool {
    is_ident_start(char) || matches!(char, '0'..='9' | '\'')
}

fn is_digit_start(char: char) -> bool {
    matches!(char, '0'..='9')
}

fn is_still_digit(char: char) -> bool {
    is_digit_start(char) || matches!(char, '_')
}

fn is_still_binary(char: char) -> bool {
    matches!(char, '0' | '1' | '_')
}
