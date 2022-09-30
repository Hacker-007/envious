#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct Location {
    pub(crate) start_index: usize,
    pub(crate) end_index: usize,
}

impl Location {
    pub fn new(start_index: usize, end_index: usize) -> Self {
        Self {
            start_index,
            end_index,
        }
    }
}
