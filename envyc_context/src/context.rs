use std::collections::hash_map::{self, HashMap};

use envyc_source::source::{SourceId, Source};

#[derive(Debug, Default)]
pub struct CompilationContext {
    source_map: HashMap<SourceId, Source>,
}

impl CompilationContext {
    pub fn get_sources(&self) -> hash_map::Iter<usize, Source> {
        self.source_map.iter()
    }
}