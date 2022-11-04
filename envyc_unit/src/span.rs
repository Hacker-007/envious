use crate::source_map::SourcePos;

pub struct Span {
	low: SourcePos,
	high: SourcePos,
}

impl Span {
	pub fn new(low: SourcePos, high: SourcePos) -> Self {
		Self {
			low,
			high,
		}
	}
}