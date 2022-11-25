use std::collections::HashMap;

pub type InternId = usize;

#[derive(Debug, Default)]
pub struct Interner {
    next_intern_id: InternId,
    id_relation: HashMap<String, InternId>,
    pool: HashMap<InternId, String>,
}

impl Interner {
    pub fn insert_value(&mut self, value: String) -> InternId {
        match self.id_relation.get(&value) {
            Some(id) => *id,
            None => {
                self.next_intern_id += 1;
                self.id_relation
                    .insert(value.to_string(), self.next_intern_id - 1);
                self.pool.insert(self.next_intern_id - 1, value);
                self.next_intern_id - 1
            }
        }
    }

    pub fn get_value(&self, id: InternId) -> Option<String> {
        self.pool.get(&id).cloned()
    }
}
