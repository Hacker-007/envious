use std::path::PathBuf;

use crate::source_map::SourcePos;

pub enum FileInformation {
    RealFile(PathBuf),
    VirtualFile,
}

// The source text that needs to be fully compiled.
// This structure does not impose any requirements on
// the origin of the source, allowing projects to load
// sources from different origins, such as from the
// file system, a remote library provider, or more.
pub struct Source {
    file_info: FileInformation,
    pub src: String,
	// The starting position of this source in the source map
	pub start_pos: SourcePos,
	// The ending position of this source in the source map
	pub end_pos: SourcePos,
}

impl Source {
    pub fn new(file_info: FileInformation, start_pos: SourcePos, end_pos: SourcePos) -> Self {
        Self {
            file_info,
            src: String::new(),
			start_pos,
			end_pos,
        }
    }
}
