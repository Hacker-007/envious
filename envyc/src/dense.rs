/// A wrapper type around vectors providing "dense"
/// storage of values; that is, a vector that is used
/// in conjunction with others to minimize the overall
/// padding used.
#[derive(Debug, Clone)]
pub struct DenseVec<T>(Vec<T>);

impl<T> DenseVec<T> {
    /// Returns an immutable reference to the item at
    /// `idx`.
    ///
    /// Note that no bounds checks are performed;
    /// thus, if the index is invalid, this function
    /// panics.
    pub fn get(&self, idx: u32) -> &T {
        debug_assert!((idx as usize) < self.0.len());
        self.0.get(idx as usize).unwrap()
    }

    /// Returns a mutable reference to the item at
    /// `idx`.
    ///
    /// Note that no bounds checks are performed;
    /// thus, if the index is invalid, this function
    /// panics.
    pub fn get_mut(&mut self, idx: u32) -> &mut T {
        debug_assert!((idx as usize) < self.0.len());
        self.0.get_mut(idx as usize).unwrap()
    }

    /// Appends `item` to the end of the vector and
    /// returns the 32-bit integer index at which it
    /// was inserted.
    ///
    /// Note that this panics if there are `2^32 - 1`
    /// items in the vector.
    pub fn push(&mut self, item: T) -> u32 {
        debug_assert!(self.0.len() < u32::MAX as usize);
        let idx = self.0.len() as u32;
        self.0.push(item);
        idx
    }
}

impl<T> Default for DenseVec<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
