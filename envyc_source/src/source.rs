use std::iter::{Enumerate, Peekable};
use std::str::Chars;

use crate::snippet::SourcePos;

pub type SourceId = usize;

#[derive(Debug)]
pub struct Source {
    pub id: SourceId,
    pub text: String,
}

impl Source {
    pub fn new(id: SourceId, text: String) -> Self {
        Self { id, text }
    }

    pub fn get_text(&self, start: SourcePos, end: SourcePos) -> &str {
        &self.text[start.0..=end.0]
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }
}

#[derive(Debug)]
pub struct SourceIter<'source> {
    source_len: usize,
    reached_eof: bool,
    chars: Peekable<Enumerate<Chars<'source>>>,
}

impl<'source> SourceIter<'source> {
    pub fn new(source: &'source Source) -> Self {
        Self {
            source_len: source.len(),
            reached_eof: false,
            chars: source.text.chars().enumerate().peekable(),
        }
    }

    pub fn next(&mut self) -> Option<(SourcePos, char)> {
        match self.chars.next() {
            None if !self.reached_eof => {
                self.reached_eof = true;
                Some((SourcePos(self.source_len), '\0'))
            }
            item => item.map(|(pos, ch)| (SourcePos(pos), ch)),
        }
    }

    pub fn peek(&mut self) -> Option<(SourcePos, char)> {
        match self.chars.peek().copied() {
            None if !self.reached_eof => {
                self.reached_eof = true;
                Some((SourcePos(self.source_len), '\0'))
            }
            item => item.map(|(pos, ch)| (SourcePos(pos), ch)),
        }
    }
}
