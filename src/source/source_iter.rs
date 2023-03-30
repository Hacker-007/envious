use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use super::{Source, span::{Span, SourcePos}};

/// Provides an interfaace to iterate through the characters in
/// a source text. When iterating, both the position and the
/// character are returned, with one caveat: when the end of the
/// source is reached, the null character ('\0') is returned.
///
/// # Examples
/// ```
/// use envyc::source::{Source, SourceIter};
///
/// let text = "Source Text";
/// let source = Source::new(0, "test.txt", text);
/// for (position, ch) in SourceIter::new(&source) {
///     assert_eq!(text.chars().nth(position).unwrap_or('\0'), ch)
/// }
/// ```
#[derive(Debug)]
pub struct SourceIter<'source, 'text> {
    len: usize,
    chars: Peekable<Enumerate<Chars<'text>>>,
    reached_eof: bool,
    source: &'source Source<'text>,
}

impl<'source, 'text> SourceIter<'source, 'text> {
    pub fn new(source: &'source Source<'text>) -> Self {
        Self {
            len: source.text.len(),
            chars: source.text.chars().enumerate().peekable(),
            reached_eof: false,
            source,
        }
    }

    pub fn get_text(&self, span: Span) -> &str {
        &self.source[span]
    }

    pub fn span(&self, start: SourcePos, end: SourcePos) -> Span {
        Span::new(self.source.id, start, end)
    }

    pub fn peek(&mut self) -> Option<(SourcePos, char)> {
        match self.chars.peek().copied() {
            None if !self.reached_eof => {
                self.reached_eof = true;
                Some((self.len, '\0'))
            }
            item => item,
        }
    }
}

impl<'source, 'text> Iterator for SourceIter<'source, 'text> {
    type Item = (SourcePos, char);

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next() {
            None if !self.reached_eof => {
                self.reached_eof = true;
                Some((self.len, '\0'))
            }
            item => item,
        }
    }
}
