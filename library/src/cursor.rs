use std::str::Chars;

pub(crate) struct Cursor<'a> {
    chars: Chars<'a>,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(code: &'a str) -> Cursor {
        Cursor {
            chars: code.chars(),
        }
    }

    fn nth(&self, index: usize) -> char {
        self.chars().nth(index).unwrap_or('\0')
    }

    pub(crate) fn first(&self) -> char {
        self.nth(0)
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub(crate) fn next(&mut self) {
        self.chars.next();
    }

    fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }

    #[allow(dead_code)] // Will eventually be used.
    fn eat_while(&mut self, mut f: impl FnMut(char) -> bool) {
        while f(self.first()) && !self.is_eof() {
            self.next();
        }
    }
}
