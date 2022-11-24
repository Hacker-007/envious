use std::collections::hash_map::{self, HashMap};

use envyc_error::error::Diagnostic;
use envyc_source::source::{Source, SourceId};

#[derive(Debug, Default)]
pub struct CompilationContext {
    source_map: HashMap<SourceId, Source>,
}

impl CompilationContext {
    pub fn get_sources(&self) -> hash_map::Iter<usize, Source> {
        self.source_map.iter()
    }

    pub fn emit_diagnostic(&self, diagnostic: Diagnostic) {
        println!("{:#?}", diagnostic);
        todo!("Emit the diagnostic to dedicated error handler rather than to STDOUT directly!")
    }
}
