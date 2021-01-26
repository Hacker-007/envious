#[derive(Debug)]
pub struct Span {
    line_start: usize,
    column_start: usize,
    line_end: usize,
    column_end: usize,
}

impl Span {
    pub fn new(line_start: usize, column_start: usize, line_end: usize, column_end: usize) -> Self {
        Self {
            line_start,
            column_start,
            line_end,
            column_end,
        }
    }
}
