use std::path::PathBuf;

pub type SourceId = usize;

#[derive(Debug)]
pub enum SourceMeta {
    RealFile(PathBuf),
    String,
}

#[derive(Debug)]
pub struct Source {
    id: SourceId,
    source_meta: SourceMeta,
    text: String,
}