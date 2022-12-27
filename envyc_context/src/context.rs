use std::{
    collections::hash_map::{self, HashMap},
    sync::RwLock,
};

use envyc_error::{error::Diagnostic, error_handler::ErrorHandler};
use envyc_source::source::{Source, SourceId};

use crate::{shared_resources::SharedResources, symbol::Symbol};

#[derive(Debug)]
pub struct CompilationContext<'shared, E: ErrorHandler> {
    source_map: HashMap<SourceId, Source>,
    shared_resources: &'shared RwLock<SharedResources<E>>,
}

impl<'shared, E: ErrorHandler> CompilationContext<'shared, E> {
    pub fn new(shared_resources: &'shared RwLock<SharedResources<E>>) -> Self {
        Self {
            source_map: HashMap::new(),
            shared_resources,
        }
    }

    pub fn get_sources(&self) -> hash_map::Iter<usize, Source> {
        self.source_map.iter()
    }

    pub fn intern_symbol(&self, id: &str) -> Symbol {
        let mut write_guard = self.shared_resources.write().unwrap();
        write_guard.interner.add(id)
    }

    pub fn emit_diagnostic(&self, diagnostic: Diagnostic) {
        let mut write_guard = self.shared_resources.write().unwrap();
        write_guard.error_handler.handle_diagnostic(diagnostic);
    }
}
