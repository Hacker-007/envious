use std::hash::Hash;

use bimap::BiMap;

/// Struct that caches values and provides a id
/// to reference the value in the future.
pub struct Interner<T: Hash + Eq> {
    next_id: usize,
    intern_map: BiMap<usize, T>,
}

impl<T: Hash + Eq> Interner<T> {
    /// Inserts the given value in the `Interner`. If the value
    /// already exists in the map, then the id of the value is 
    /// returned.
    ///
    /// # Arguments
    /// `value` - The value to insert into the map.
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

impl<T> Default for Interner<T>
where
    T: Hash + Eq,
{
    fn default() -> Self {
        Self {
            next_id: 0,
            intern_map: BiMap::new(),
        }
    }
}
