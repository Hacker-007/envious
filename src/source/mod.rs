mod span;
mod source_iter;
mod source_map;

pub use span::{SourcePos, Span, Spanned, WithSpan};
pub use source_iter::SourceIter;
pub use source_map::SourceMap;

use std::ops::Index;

use codespan_reporting::files::line_starts;

pub type SourceId = usize;

/// Represents a source text that is processed through the
/// compiler pipeline. The `Source` also contains metadata
/// useful for error reporting, such as the name of the source
/// and its id for quick retrieval.
///
/// # Examples
/// ```
/// use envyc::source::{Span, Source};
///
/// let source = Source::new(0, "test.txt", "This is the text of the source");
/// let source_len = source.len();
/// let span = Span::new(0, 0, source_len);
/// assert_eq!(&source[span], "This is the text of the source");
/// ```
#[derive(Debug)]
pub struct Source<'text> {
    id: SourceId,
    name: String,
    text: &'text str,
    line_start_indices: Vec<usize>,
}

impl<'text> Source<'text> {
    pub fn new(id: SourceId, name: &str, text: &'text str) -> Self {
        Self {
            id,
            name: name.to_string(),
            text,
            line_start_indices: line_starts(text).collect(),
        }
    }

    pub fn id(&self) -> SourceId {
        self.id
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Returns the index of the line on which the byte at index `byte_index` lies.
    /// If the `byte_index` exceeds the length of the source text, then the index
    /// of the last line is returned.
    ///
    /// # Examples
    /// ```
    /// use envyc::source::Source;
    ///
    /// let source = Source::new(0, "test.txt", "This is the text of the source\nThis is a new line.");
    /// assert_eq!(source.line_index(12), 0);
    /// assert_eq!(source.line_index(35), 1);
    /// assert_eq!(source.line_index(100), 1);
    /// ```
    pub fn line_index(&self, byte_index: usize) -> usize {
        self.line_start_indices
            .binary_search(&byte_index)
            .unwrap_or_else(|next_line| next_line - 1)
    }

    /// Returns the starting byte index of the line at index `line_index`.
    /// If the line index exceeds the lines in the source text, then an error
    /// with the maximum possible line index is returned.
    ///
    /// # Examples
    /// ```
    /// use envyc::source::Source;
    ///
    /// let source = Source::new(0, "test.txt", "This is the text of the source\nThis is a new line");
    /// assert_eq!(source.line_start_index(0), Ok(0));
    /// assert_eq!(source.line_start_index(1), Ok(31));
    /// assert_eq!(source.line_start_index(2), Ok(49));
    /// assert_eq!(source.line_start_index(3), Err(1));
    /// ```
    pub fn line_start_index(&self, line_index: usize) -> Result<usize, usize> {
        if line_index == self.line_start_indices.len() {
            Ok(self.text.len())
        } else {
            self.line_start_indices
                .get(line_index)
                .copied()
                .ok_or(self.line_start_indices.len() - 1)
        }
    }

    pub fn iter(&self) -> SourceIter<'_, 'text> {
        SourceIter::new(self)
    }
}

impl<'text> Index<Span> for Source<'text> {
    type Output = str;

    fn index(&self, span: Span) -> &Self::Output {
        &self.text[span.start()..span.end()]
    }
}
