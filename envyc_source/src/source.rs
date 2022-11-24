use std::path::PathBuf;
use std::iter::Peekable;
use std::str::Chars;

use crate::snippet::SourcePos;

pub type SourceId = usize;

#[derive(Debug)]
pub enum SourceMeta {
    RealFile(PathBuf),
    String,
}

#[derive(Debug)]
pub struct Source {
    pub id: SourceId,
    source_meta: SourceMeta,
	pub text: String,
}

impl Source {
	pub fn get_range(&self, low: SourcePos, high: SourcePos) -> &str {
		&self.text[low.0..=high.0]
	}
}

#[derive(Debug)]
pub struct SourceIter<'source> {
	chars: Peekable<Chars<'source>>,
}

impl<'source> SourceIter<'source> {
	pub fn new(source: &'source Source) -> Self {
		Self {
			chars: source.text.chars().peekable(),
		}
	}

	pub fn next(&mut self) -> Option<char> {
		self.chars.next()
	}

	pub fn peek(&mut self) -> Option<char> {
		self.chars.peek().copied()
	}
}