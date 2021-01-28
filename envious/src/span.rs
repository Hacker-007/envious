#[derive(Debug)]
pub struct Span {
    file_name: String,
    line_start: usize,
    column_start: usize,
    line_end: usize,
    column_end: usize,
}

impl Span {
    pub fn new(
        file_name: String,
        line_start: usize,
        column_start: usize,
        line_end: usize,
        column_end: usize,
    ) -> Self {
        Self {
            file_name,
            line_start,
            column_start,
            line_end,
            column_end,
        }
    }
}
