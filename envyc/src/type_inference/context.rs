use std::collections::HashMap;

use super::monotype::MonotypeRef;

#[derive(Debug)]
pub struct Context {
    vars: HashMap<usize, MonotypeRef>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn get(&self, id: usize) -> Option<MonotypeRef> {
        self.vars.get(&id).cloned()
    }

    pub fn define(&mut self, id: usize, ty: MonotypeRef) {
        self.vars.insert(id, ty);
    }
}
