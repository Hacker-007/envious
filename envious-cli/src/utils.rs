use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

pub fn clean_file(file: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    if file.is_file() {
        Ok(file.canonicalize()?)
    } else {
        Err(error("Compilation of directories is not yet supported."))
    }
}

pub fn get_source(file: &Path) -> Result<String, Box<dyn Error>> {
    let source = fs::read_to_string(file)?;
    Ok(source)
}

pub fn path_to_str(file: &Path) -> Result<&str, Box<dyn Error>> {
    file.to_str().ok_or_else(|| {
        error(format!(
            "Could not convert the file `{}` into a string.",
            file.display()
        ))
    })
}

pub fn get_stem(file: &Path) -> Result<&str, Box<dyn Error>> {
    let stem = file.file_stem().ok_or_else(|| {
        error(format!(
            "Could not get the file stem of file `{}`.",
            file.display()
        ))
    })?;

    stem.to_str().ok_or_else(|| {
        error(format!(
            "Could not convert the file `{}` into a string.",
            file.display()
        ))
    })
}

pub fn replace_last<S: AsRef<str>>(file: &Path, replacement: S) -> Result<PathBuf, Box<dyn Error>> {
    let parent = file
        .parent()
        .ok_or_else(|| error("Could not find a parent for this file"))?;
    Ok(parent.join(replacement.as_ref()))
}

pub fn error<S: AsRef<str>>(message: S) -> Box<dyn Error> {
    Box::<dyn Error + Send + Sync>::from(message.as_ref())
}
