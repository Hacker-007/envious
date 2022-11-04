use crate::source::Source;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct SourcePos(usize);

#[derive(Default)]
pub struct SourceMap {
    next_source_start: usize,
    source_files: Vec<Source>,
}

impl SourceMap {
    pub fn add_source(&mut self, source: Source) {
        match self.next_source_start.checked_add(source.src.len()) {
            Some(source_position) => {
            },
            None => {

            }
        }
        

        todo!()
    }

    pub fn get_source(&mut self, pos: SourcePos) -> &Source {
        let source_idx = self
            .source_files
            .binary_search_by_key(&pos, |source| source.start_pos)
            .expect(&format!(
                "Bug: source with start pos `{}` is not in source map!",
                pos.0
            ));

        &self.source_files[source_idx]
    }
}
