use std::{
    cell::RefCell,
    collections::hash_map::{self, HashMap},
};

use envyc_error::error::Diagnostic;
use envyc_source::source::{Source, SourceId};

use crate::interner::{InternId, Interner};

#[derive(Debug)]
pub struct CompilationContext {
    source_map: HashMap<SourceId, Source>,
    interner: RefCell<Interner>,
}

impl CompilationContext {
    pub fn new(interner: RefCell<Interner>) -> Self {
        Self {
            source_map: HashMap::new(),
            interner,
        }
    }

    pub fn get_sources(&self) -> hash_map::Iter<usize, Source> {
        self.source_map.iter()
    }

    pub fn intern_value(&self, value: String) -> InternId {
        self.interner.borrow_mut().insert_value(value)
    }

    pub fn get_interned_value(&self, id: InternId) -> Option<String> {
        self.interner.borrow().get_value(id)
    }

    pub fn emit_diagnostic(&self, diagnostic: Diagnostic) {
        println!("{:#?}", diagnostic);
        todo!("Emit the diagnostic to dedicated error handler rather than to STDOUT directly!")
    }
}
