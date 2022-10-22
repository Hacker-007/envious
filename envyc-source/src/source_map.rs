use crate::source::Source;

pub(crate) struct SourceId(usize);

pub(crate) struct SourceMap {
    sources: Vec<Source>,
}

impl SourceMap {
    pub fn new(sources: Vec<Source>) -> Self {
        Self { sources }
    }

    /// Finds the source associated with the given source id. It is assumed
    /// that the provided source id is valid, or within the bounds of the sources
    /// within the map. This assumption is always valid if the source ids used
    /// are *not* user-generated.
    pub fn find_source_by_id(&self, source_id: SourceId) -> &Source {
        return &self.sources[source_id.0];
    }
}

pub(crate) struct SourceMapBuilder {
    sources: Vec<Source>
}