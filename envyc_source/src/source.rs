use std::iter::{Enumerate, Peekable};
use std::str::Chars;

use crate::snippet::SourcePos;

pub type SourceId = usize;

#[derive(Debug)]
pub struct Source {
    pub id: SourceId,
    pub name: String,
    pub text: String,
}

#[derive(Debug)]
pub struct LineInformation {
    pub line_number: usize,
    pub line_start: SourcePos,
    pub line_end: SourcePos,
}

impl Source {
    pub fn new(id: SourceId, name: String, text: String) -> Self {
        Self { id, name, text }
    }

    pub fn get_text(&self, start: SourcePos, end: SourcePos) -> &str {
        &self.text[start.0..end.0]
    }

    pub fn extend_back(&self, pos: SourcePos) -> SourcePos {
        let line_info = self.get_line_information(pos);
        line_info.line_start
    }

    pub fn extend(&self, pos: SourcePos) -> SourcePos {
        let line_info = self.get_line_information(pos);
        line_info.line_end
    }

    pub fn get_line_information(&self, pos: SourcePos) -> LineInformation {
        let mut line_number = 1;
        let mut line_start = 0;
        let mut line_end = 0;
        for line in self.text.split_inclusive('\n') {
            line_end += line.len();
            if line_start + line.len() >= pos.0 {
                break;
            }

            line_number += 1;
            line_start += line.len();
        }

        LineInformation {
            line_number,
            line_start: SourcePos(line_start),
            line_end: SourcePos(line_end),
        }
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
