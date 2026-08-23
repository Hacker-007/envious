use std::ops::{Add, Index, IndexMut};

/// A wrapper type around the indices within a dense
/// vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DenseIndex(u32);

impl DenseIndex {
    pub const ZERO: Self = Self(0);

    pub fn saturating_add(self, rhs: u32) -> Self {
        Self(self.0.saturating_add(rhs))
    }
}

/// A wrapper type around a contiguous range of elements within
/// a dense vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DenseRange {
    start: DenseIndex,
    end: DenseIndex,
}

/// A wrapper type around vectors providing "dense"
/// storage of values; that is, a vector that is used
/// in conjunction with others to minimize the overall
/// padding used.
#[derive(Debug, Clone)]
pub struct DenseVec<T>(Vec<T>);

impl<T> DenseVec<T> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Appends `item` to the end of the vector and
    /// returns the 32-bit integer index at which it
    /// was inserted.
    ///
    /// Note that this panics if there is not enough space
    /// in the vector.
    pub fn push(&mut self, item: T) -> DenseIndex {
        debug_assert!(self.0.len() < u32::MAX as usize);
        let idx = self.0.len() as u32;
        self.0.push(item);
        DenseIndex(idx)
    }

    /// Appends all `items` to the end of the vector and
    /// returns the index range at which the elements were
    /// inserted.
    ///
    /// Note that this panics if there are is not enough space
    /// in the vector.
    pub fn push_all(&mut self, items: impl IntoIterator<Item = T>) -> DenseRange {
        let start = self.0.len() as u32;
        self.0.extend(items);
        debug_assert!(self.0.len() < u32::MAX as usize);
        let end = self.0.len() as u32;
        DenseRange {
            start: DenseIndex(start),
            end: DenseIndex(end),
        }
    }

    pub fn get(&self, idx: DenseIndex) -> Option<&T> {
        self.0.get(idx.0 as usize)
    }
}

impl<T> Default for DenseVec<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Index<DenseIndex> for DenseVec<T> {
    type Output = T;

    fn index(&self, idx: DenseIndex) -> &Self::Output {
        debug_assert!((idx.0 as usize) < self.0.len());
        self.0.get(idx.0 as usize).unwrap()
    }
}

impl<T> IndexMut<DenseIndex> for DenseVec<T> {
    fn index_mut(&mut self, idx: DenseIndex) -> &mut Self::Output {
        debug_assert!((idx.0 as usize) < self.0.len());
        self.0.get_mut(idx.0 as usize).unwrap()
    }
}

impl<T> Index<DenseRange> for DenseVec<T> {
    type Output = [T];

    fn index(&self, DenseRange { start, end }: DenseRange) -> &Self::Output {
        &self.0[(start.0 as usize)..(end.0 as usize)]
    }
}

impl<T> IndexMut<DenseRange> for DenseVec<T> {
    fn index_mut(&mut self, DenseRange { start, end }: DenseRange) -> &mut Self::Output {
        &mut self.0[(start.0 as usize)..(end.0 as usize)]
    }
}
