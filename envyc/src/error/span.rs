use std::cmp::{max, min};

/// Struct used by all parts of the program to note the location information
/// of the different tokens and expressions generated by the `Lexer` and the
/// `Parser` respectively.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span<'a> {
    // The name of the file.
    pub file_name: &'a str,
    // The line where this span starts.
    pub line_start: usize,
    // The column where this span starts.
    pub column_start: usize,
    // The line where this span ends.
    pub line_end: usize,
    // The column where this span ends.
    pub column_end: usize,
}

impl<'a> Span<'a> {
    /// Creates a new `Span`.
    pub fn new(
        file_name: &'a str,
        line_start: usize,
        column_start: usize,
        line_end: usize,
        column_end: usize,
    ) -> Self {
        Self {
            file_name,
            line_start,
            column_start,
            line_end,
            column_end,
        }
    }

    /// Combines two spans together.
    /// This function expects that the spans passed
    /// are from the same file. If this is not true,
    /// the generated span may not be correct.
    ///
    /// # Arguments
    /// * `other` - The other span to use when combining.
    pub fn combine(&self, other: Span<'a>) -> Span<'a> {
        if self.file_name != other.file_name {
            panic!("The file names of the Span's do not match!");
        }

        Span::new(
            self.file_name,
            min(self.line_start, other.line_start),
            min(self.column_start, other.column_start),
            max(self.line_end, other.line_end),
            max(self.column_end, other.column_end),
        )
    }
}
