use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use envyc_source::{source::Source, span::SourcePos};

pub struct SourceIter<'source> {
    source_len: usize,
    reached_eof: bool,
    chars: Peekable<Enumerate<Chars<'source>>>,
}

impl<'source> SourceIter<'source> {
    pub fn new(source: &'source Source) -> Self {
        Self {
            source_len: source.text.len(),
            reached_eof: false,
            chars: source.text.chars().enumerate().peekable(),
        }
    }

    pub fn peek(&mut self) -> Option<(SourcePos, char)> {
        match self.chars.peek().copied() {
            None if !self.reached_eof => {
                self.reached_eof = true;
                Some((self.source_len, '\0'))
            }
            item => item,
        }
    }

    pub fn next(&mut self) -> Option<(SourcePos, char)> {
        match self.chars.next() {
            None if !self.reached_eof => {
                self.reached_eof = true;
                Some((self.source_len, '\0'))
            }
            item => item,
        }
    }
}
