use std::ops::{Sub, Add};

use crate::source::SourceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourcePos(pub usize);

impl Add<usize> for SourcePos {
    type Output = SourcePos;

    fn add(self, rhs: usize) -> Self::Output {
        SourcePos(self.0 + rhs)
    }
}

impl Add for SourcePos {
    type Output = SourcePos;

    fn add(self, rhs: Self) -> Self::Output {
        SourcePos(self.0 + rhs.0)
    }
}

impl Sub for SourcePos {
    type Output = SourcePos;

    fn sub(self, rhs: Self) -> Self::Output {
        SourcePos(self.0 - rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Snippet {
    pub source_id: SourceId,
    pub start: SourcePos,
    pub end: SourcePos,
}

impl Snippet {
    pub fn new(source_id: SourceId, start: SourcePos, end: SourcePos) -> Self {
        Self {
            source_id,
            start,
            end,
        }
    }

    pub fn extend_back(self, previous_position: SourcePos) -> Self {
        Self {
            start: previous_position,
            ..self
        }
    }

    pub fn extend(self, extended_position: SourcePos) -> Self {
        Self {
            end: extended_position,
            ..self
        }
    }
}
