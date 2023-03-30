use std::ops::Range;

use codespan_reporting::files::{Error as CodespanError, Files};

use super::{Source, SourceId};

/// Stores all of the sources in the compilation process for retrieval
/// at a later point. This is mostly used in the beginning stage of
/// analysis and during error reporting.
/// 
/// # Examples
/// use envyc::source::SourceMap;
/// 
/// let mut source_map = SourceMap::default();
/// let source_id = source_map.insert("name of source", "text of source");
/// assert!(source_map.get(source_id).is_some())
#[derive(Debug, Default)]
pub struct SourceMap<'text> {
    sources: Vec<Source<'text>>,
}

impl<'text> SourceMap<'text> {
    pub fn insert(&mut self, name: &str, text: &'text str) -> SourceId {
        let id = self.sources.len();
        self.sources.push(Source::new(id, name, text));
        id
    }

    pub fn get(&self, id: SourceId) -> Option<&Source<'text>> {
        self.sources.get(id)
    }
}

impl<'text> Files<'text> for SourceMap<'text> {
    type FileId = SourceId;
    type Name = String;
    type Source = &'text str;

    fn name(&self, id: Self::FileId) -> Result<Self::Name, CodespanError> {
        self.get(id)
            .map(|source| source.name.clone())
            .ok_or(CodespanError::FileMissing)
    }

    fn source(&self, id: Self::FileId) -> Result<Self::Source, CodespanError> {
        self.get(id)
            .map(|source| source.text)
            .ok_or(CodespanError::FileMissing)
    }

    fn line_index(&self, id: Self::FileId, byte_index: usize) -> Result<usize, CodespanError> {
        self.get(id)
            .map(|source| source.line_index(byte_index))
            .ok_or(CodespanError::FileMissing)
    }

    fn line_range(
        &self,
        id: Self::FileId,
        line_index: usize,
    ) -> Result<Range<usize>, CodespanError> {
        let source = self.get(id).ok_or(CodespanError::FileMissing)?;
        let current_line_start = source
            .line_start_index(line_index)
            .map_err(|max_line_idx| CodespanError::IndexTooLarge {
                given: line_index,
                max: max_line_idx,
            })?;

        let next_line_start = source
            .line_start_index(line_index + 1)
            .map_err(|max_line_idx| CodespanError::IndexTooLarge {
                given: line_index + 1,
                max: max_line_idx,
            })?;

        Ok(current_line_start..next_line_start)
    }
}
