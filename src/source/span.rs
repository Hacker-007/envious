use std::ops::{Deref, DerefMut};

use crate::source::SourceId;

pub type SourcePos = usize;

/// Represents a range of characters in the source text. Nodes in the
/// compiler process are augmented with `Span`s to accomodate error
/// reporting.
///
/// # Notes
/// A `Span`s range is exclusive: the last character is at an index of
/// `end` - 1 in the `Source` text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    source_id: SourceId,
    start: SourcePos,
    end: SourcePos,
}

impl Span {
    pub fn new(source_id: SourceId, start: SourcePos, end: SourcePos) -> Self {
        Self {
            source_id,
            start,
            end,
        }
    }

    pub fn source_id(&self) -> SourceId {
        self.source_id
    }

    pub fn start(&self) -> SourcePos {
        self.start
    }

    pub fn end(&self) -> SourcePos {
        self.end
    }
}

/// Utility type to represent a node with associated `Span` data.
/// The easiest way to create a `Spanned` structure is to call the
/// `attach_span` method on any value.
///
/// # Examples
/// ```
/// use envyc::source::{Span, WithSpan};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// enum TokenKind {
///     Integer,
///     Boolean,
/// }
///
/// let kind = TokenKind::Integer.with_span(Span::new(0, 0, 0));
/// assert_eq!(kind.span(), Span::new(0, 0, 0));
/// assert_eq!(*kind, TokenKind::Integer);
#[derive(Debug)]
pub struct Spanned<T> {
    span: Span,
    item: T,
}

impl<T> Spanned<T> {
    pub fn new(span: Span, item: T) -> Self {
        Self { span, item }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn item(&self) -> &T {
        &self.item
    }
}

impl<T: Copy> Copy for Spanned<T> {}
impl<T: Clone> Clone for Spanned<T> {
    fn clone(&self) -> Self {
        Self {
            span: self.span,
            item: self.item.clone(),
        }
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}

pub trait WithSpan {
    fn with_span(self, span: Span) -> Spanned<Self>
    where
        Self: Sized;
}

impl<T> WithSpan for T {
    fn with_span(self, span: Span) -> Spanned<Self>
    where
        Self: Sized,
    {
        Spanned::new(span, self)
    }
}
