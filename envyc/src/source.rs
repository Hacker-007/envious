use std::str::Chars;

use crate::dense::{DenseIndex, DenseVec};

#[derive(Debug, Clone, Copy)]
pub struct SourceId(DenseIndex);

#[derive(Debug, Default)]
pub struct SourceMap {
    sources: DenseVec<Source>,
}

impl SourceMap {
    pub(crate) fn register(&mut self, bytes: impl Into<Box<str>>) -> (SourceId, &Source) {
        // Insert the source with a dummy ID which we will overwrite 
        let idx = self
            .sources
            .push(Source::new(SourceId(DenseIndex::default()), bytes));

        self.sources[idx].id = SourceId(idx);
        (SourceId(idx), &self.sources[idx])
    }
}

/// A reference to the original source bytes
/// being compiled.
#[derive(Debug)]
pub struct Source {
    id: SourceId,
    text: Box<str>,
}

impl Source {
    pub fn new(id: SourceId, source: impl Into<Box<str>>) -> Self {
        Self {
            id,
            text: source.into(),
        }
    }

    #[inline]
    pub fn id(&self) -> SourceId {
        self.id
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.text.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    #[inline]
    pub fn chars(&self) -> Chars<'_> {
        self.text.chars()
    }
}

/// A contiguous range of characters within the
/// original source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub(crate) start: u32,
    pub(crate) end: u32,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        debug_assert!(start <= u32::MAX as usize);
        debug_assert!(end <= u32::MAX as usize);
        Self {
            start: start as u32,
            end: end as u32,
        }
    }
}
