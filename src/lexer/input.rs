use std::{iter::Peekable, str::Chars};

use crate::utils::*;

#[derive(Debug)]
pub struct Cursor<'a> {
    iter: Peekable<Chars<'a>>,
    pos: usize,
    row: usize,
    col: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            iter: input.chars().peekable(),
            pos: 0,
            row: 0,
            col: 0,
        }
    }

    fn advance(&mut self, c: char) {
        if let '\n' = c {
            self.col += 1;
            self.row = 0;
        } else {
            self.row += 1;
        }
        self.pos += 1;
    }

    pub fn peek_char(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    #[must_use]
    pub fn next_if<F>(&mut self, cond: F) -> Option<char>
    where
        F: Fn(&char) -> bool,
    {
        let c = self.iter.next_if(cond)?;
        self.advance(c);
        Some(c)
    }

    pub fn skip_if<F>(&mut self, cond: F) 
    where
        F: Fn(&char) -> bool,
    {
        if let Some(c) = self.iter.next_if(cond) {
            self.advance(c);
        }
    }

    pub fn skip_while<F>(&mut self, cond: F)
    where
        F: Fn(&char) -> bool,
    {
        while self.next_if(&cond).is_some() {}
        self.next();
    }

    pub fn next(&mut self) -> Option<char> {
        let c = self.iter.next()?;
        self.advance(c);
        Some(c)
    }

    pub fn loc(&self) -> Loc {
        Loc {
            pos: self.pos,
            row: self.row,
            col: self.col,
        }
    }

    pub fn span(&self, start: Loc) -> Span {
        let end = Loc {
            pos: self.pos,
            row: self.row,
            col: self.col,
        };
        Span::new(start, end)
    }
}
