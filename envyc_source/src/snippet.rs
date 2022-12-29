use crate::source::SourceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourcePos(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Snippet {
    source_id: SourceId,
    pub low: SourcePos,
    pub high: SourcePos,
}

impl Snippet {
    pub fn new(source_id: SourceId, low: SourcePos, high: SourcePos) -> Self {
        Self {
            source_id,
            low,
            high,
        }
    }

    pub fn with_low(self, low: SourcePos) -> Self {
        Self { low, ..self }
    }
}
