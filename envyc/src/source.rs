use std::{cmp::Ordering, ops::Range, rc::Rc, str::Chars};

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
    pub(crate) fn register(&mut self, name: impl ToString, text: impl Into<Box<str>>) -> SourceId {
        // Insert the source with a dummy ID which we will overwrite
        let idx = self.sources.push(Source::new(
            SourceId::default(),
            name.to_string(),
            text.into(),
        ));

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
    name: String,
    text: Box<str>,
    /// An eagerly computed list of byte offsets at which line `i`
    /// starts, i.e. the line at index `i` starts at byte offset
    /// `line_starts[i]`.
    line_starts: Vec<usize>,
}

impl Source {
    fn new(id: SourceId, name: String, text: Box<str>) -> Self {
        let line_starts = core::iter::once(0)
            .chain(text.match_indices('\n').map(|(i, _)| i + 1))
            .collect();

        Self {
            id,
            name: name.into(),
            text,
            line_starts,
        }
    }

    #[inline]
    pub fn id(&self) -> SourceId {
        self.id
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the starting byte offset of the line at index `index`.
    pub fn line(&self, index: usize) -> Option<usize> {
        match index.cmp(&self.line_starts.len()) {
            Ordering::Less => Some(self.line_starts[index]),
            Ordering::Equal => Some(self.text.len()),
            Ordering::Greater => None,
        }
    }

    #[inline]
    pub fn lines(&self) -> &[usize] {
        &self.line_starts
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
    pub fn slice(&self, span: Span) -> &str {
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

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        Range {
            start: span.start as usize,
            end: span.end as usize,
        }
    }
}
