use self::TokenKind::*;
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
pub enum TokenKind {
    Whitespace,
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
            c => self.eat_unknown_char(c),
        }
    }

    fn eat_whitespace(&mut self) -> Token {
        self.cursor.next();
        Token::new(Whitespace, self.cursor.len_consumed())
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
