use std::{rc::Rc, str::Chars};

use crate::dense::{DenseIndex, DenseVec};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceId(DenseIndex);

impl Default for SourceId {
    fn default() -> Self {
        Self(DenseIndex::ZERO)
    }
}

#[derive(Debug, Default)]
pub struct SourceMap {
    sources: DenseVec<Source>,
}

impl SourceMap {
    pub(crate) fn register(&mut self, bytes: impl Into<Box<str>>) -> SourceId {
        // Insert the source with a dummy ID which we will overwrite
        let idx = self.sources.push(Source::new(SourceId::default(), bytes));
        self.sources[idx].id = SourceId(idx);
        SourceId(idx)
    }

    pub(crate) fn get(&self, id: SourceId) -> &Source {
        &self.sources[id.0]
    }
}

/// A reference to the original source bytes being compiled.
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

    #[inline]
    pub fn text(&self, span: Span) -> &str {
        &self.text[span.start as usize..span.end as usize]
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
