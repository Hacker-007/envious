use std::ops::Range;

use crate::source::SourceId;

pub type SourcePos = usize;

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub source_id: SourceId,
    pub start_pos: SourcePos,
    pub end_pos: SourcePos,
}

impl Span {
    pub fn new(source_id: SourceId, start_pos: SourcePos, end_pos: SourcePos) -> Self {
        Self {
            source_id,
            start_pos,
            end_pos,
        }
    }
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        span.start_pos..span.end_pos
    }
}

#[derive(Debug)]
pub struct Spanned<T> {
    pub span: Span,
    pub node: T,
}

impl<T> Spanned<T> {
    pub fn new(span: Span, node: T) -> Self {
        Self { span, node }
    }
}
