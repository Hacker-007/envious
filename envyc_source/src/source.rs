use std::{cmp::Ordering, ops::Index};

use codespan_reporting::files::line_starts;

use crate::span::Span;

pub type SourceId = usize;
pub type LineStartIdx = usize;

#[derive(Debug)]
pub struct Source {
    pub id: SourceId,
    pub name: String,
    pub text: String,
    pub precomputed_line_starts: Vec<LineStartIdx>,
}

impl Source {
    pub fn new(id: SourceId, name: String, text: String) -> Self {
        let precomputed_line_starts = line_starts(&text).collect();
        Self {
            id,
            name,
            text,
            precomputed_line_starts,
        }
    }

    pub(crate) fn line_start(
        &self,
        line_idx: usize,
    ) -> Result<usize, codespan_reporting::files::Error> {
        match line_idx.cmp(&self.precomputed_line_starts.len()) {
            Ordering::Less => Ok(self
                .precomputed_line_starts
                .get(line_idx)
                .cloned()
                .expect("")),
            Ordering::Equal => Ok(self.text.len()),
            Ordering::Greater => Err(codespan_reporting::files::Error::LineTooLarge {
                given: line_idx,
                max: self.precomputed_line_starts.len() - 1,
            }),
        }
    }
}

impl Index<Span> for Source {
    type Output = str;

    fn index(&self, idx: Span) -> &Self::Output {
        &self.text[idx.start_pos..idx.end_pos]
    }
}