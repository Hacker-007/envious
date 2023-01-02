use crate::source::SourceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourcePos(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Snippet {
    source_id: SourceId,
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
        assert!(self.start > previous_position);
        Self {
            start: previous_position,
            ..self
        }
    }
}
