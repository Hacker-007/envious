use std::hash::Hash;

use bimap::BiMap;

pub struct Interner<T: Hash + Eq> {
    next_id: usize,
    intern_map: BiMap<usize, T>,
}

impl<T: Hash + Eq> Interner<T> {
    pub fn insert(&mut self, value: T) -> usize {
        if let Some(id) = self.intern_map.get_by_right(&value) {
            *id
        } else {
            self.intern_map.insert(self.next_id, value);
            self.next_id += 1;
            self.next_id - 1
        }
    }
}

impl<T> Default for Interner<T> where T: Hash + Eq {
    fn default() -> Self {
        Self {
            next_id: 0,
            intern_map: BiMap::new(),
        }
    }
}