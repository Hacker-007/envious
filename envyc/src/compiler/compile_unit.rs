#[derive(Debug)]
pub(crate) struct CompileUnit<'file> {
    pub(crate) file_name: String,
    pub(crate) file_contents: &'file [u8],
}

impl<'file> CompileUnit<'file> {
    pub fn new(file_name: &str, file_contents: &'file [u8]) -> Self {
        Self {
            file_name: file_name.to_string(),
            file_contents,
        }
    }
}
