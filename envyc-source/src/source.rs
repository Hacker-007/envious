use std::{fs, io, path::Path};

/// An abstraction over dealing with the sources given at the provided path.
/// This trait is also used to lazily load the contents of the given source.
pub(crate) trait SourceLoader {
    fn exists(&self) -> bool;
    fn get_source(&self) -> io::Result<String>;
}

/// Represents the type of the source. This used to improve the error
/// messages shown to the consumer.
pub(crate) struct FileSourceLoader<'path> {
    path: &'path Path,
}

impl<'path> SourceLoader for FileSourceLoader<'path> {
    fn exists(&self) -> bool {
        self.path.exists()
    }

    fn get_source(&self) -> io::Result<String> {
        fs::read_to_string(self.path)
    }
}

pub(crate) struct Source {
    name: String,
    cached_fetched_contents: Option<String>,
    content_loader: Box<dyn SourceLoader>,
}

impl Source {
    pub fn new(name: String, content_loader: Box<dyn SourceLoader>) -> Self {
        Self {
            name,
            cached_fetched_contents: None,
            content_loader,
        }
    }

    fn get_content(&mut self) -> Result<&str, io::Error> {
        if let Some(ref cached_contents) = self.cached_fetched_contents {
            return Ok(cached_contents);
        }

        match self.content_loader.get_source() {
            Ok(source_contents) => {
                self.cached_fetched_contents = Some(source_contents);
                Ok(self.cached_fetched_contents.as_ref().unwrap())
            }
            Err(error) => {
                // Report the error within the session context as a fatal error, preventing any
                // further computation.
                self.cached_fetched_contents = None;
                Err(error)
            }
        }
    }
}