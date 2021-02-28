use std::str::Chars;

pub(crate) struct Cursor<'a> {
    chars: Chars<'a>,
    initial_len: usize,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(code: &'a str) -> Cursor {
        Cursor {
            chars: code.chars(),
            initial_len: code.len(),
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

    pub(crate) fn len_consumed(&self) -> usize {
        self.initial_len - self.chars.as_str().len()
    }

    fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }

    pub(crate) fn next(&mut self) {
        self.chars.next();
    }

    pub(crate) fn next_while(&mut self, mut pred: impl FnMut(char) -> bool) {
        while pred(self.first()) && !self.is_eof() {
            self.next();
        }
    }
}
