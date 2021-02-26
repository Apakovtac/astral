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

// TODO: Don't store character data in TokenKind that can be found from the string.
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Whitespace,
    Ident,
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
