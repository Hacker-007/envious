use std::ops::Range;

use codespan_reporting::files::Files;

use crate::source::{Source, SourceId};

#[derive(Debug, Default)]
pub struct SourceMap {
    sources: Vec<Source>,
}

impl SourceMap {
    pub fn push<N: ToString, S: ToString>(&mut self, name: N, text: S) -> usize {
        self.sources.push(Source::new(
            self.sources.len(),
            name.to_string(),
            text.to_string(),
        ));
        self.sources.len() - 1
    }

    pub fn get(
        &self,
        source_id: SourceId,
    ) -> Result<&Source, codespan_reporting::files::Error> {
        self.sources
            .get(source_id)
            .ok_or_else(|| codespan_reporting::files::Error::FileMissing)
    }
}

impl<'a> Files<'a> for SourceMap {
    type FileId = SourceId;
    type Name = &'a str;
    type Source = &'a str;

    fn name(&'a self, id: Self::FileId) -> Result<Self::Name, codespan_reporting::files::Error> {
        self.get(id).map(|source| source.name.as_ref())
    }

    fn source(
        &'a self,
        id: Self::FileId,
    ) -> Result<Self::Source, codespan_reporting::files::Error> {
        self.get(id).map(|source| source.text.as_ref())
    }

    fn line_index(
        &'a self,
        id: Self::FileId,
        byte_idx: usize,
    ) -> Result<usize, codespan_reporting::files::Error> {
        Ok(self
            .get(id)?
            .precomputed_line_starts
            .binary_search(&byte_idx)
            .unwrap_or_else(|next_line| next_line - 1))
    }

    fn line_range(
        &'a self,
        id: Self::FileId,
        line_idx: usize,
    ) -> Result<Range<usize>, codespan_reporting::files::Error> {
        let source = self.get(id)?;
        let line_start = source.line_start(line_idx)?;
        let next_line_start = source.line_start(line_idx + 1)?;
        Ok(line_start..next_line_start)
    }
}
