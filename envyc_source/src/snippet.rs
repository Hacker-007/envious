use crate::source::SourceId;

#[derive(Debug)]
pub struct SourcePos(usize);

#[derive(Debug)]
pub struct Snippet {
    source_id: SourceId,
    low: SourcePos,
    high: SourcePos,
}