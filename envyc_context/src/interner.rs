use std::{collections::HashMap, ops::Index};

use crate::symbol::{Symbol, SymbolId};

#[derive(Debug, Default)]
pub struct Interner {
    next_intern_id: SymbolId,
    identifier_pool: Vec<String>,
    id_relation: HashMap<String, Symbol>,
}

impl Interner {
    pub fn add(&mut self, identifier: &str) -> Symbol {
        match self.id_relation.get(identifier) {
            Some(id) => *id,
            None => {
                let intern_id = self.next_intern_id;
                let symbol = Symbol(intern_id);
                self.next_intern_id = intern_id + 1;
                self.identifier_pool.push(identifier.to_string());
                self.id_relation.insert(identifier.to_string(), symbol);
                symbol
            }
        }
    }
}

impl Index<SymbolId> for Interner {
    type Output = String;

    fn index(&self, index: SymbolId) -> &Self::Output {
        &self.identifier_pool[index]
    }
}
