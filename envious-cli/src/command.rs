use std::{error::Error, path::PathBuf, time::Instant};

use clap::ArgMatches;
use envyc::{error::reporter::ErrorReporter, interner::Interner};

use crate::{
    compile_code,
    utils::{clean_file, error, get_source, get_stem, path_to_str, replace_last},
};

#[derive(Debug)]
pub enum Command {
    Tui,
    Compile { files: Vec<PathBuf> },
    Build { files: Vec<PathBuf> },
    Run { files: Vec<PathBuf> },
    Unknown,
}

impl From<ArgMatches<'_>> for Command {
    fn from(matches: ArgMatches) -> Self {
        let start_tui = matches.is_present("tui");
        if start_tui {
            Self::Tui
        } else if let Some(compile_matches) = matches.subcommand_matches("compile") {
            let files = compile_matches.values_of("files").unwrap();
            let mut file_paths = vec![];
            for file in files {
                file_paths.push(PathBuf::from(file));
            }

            Self::Compile { files: file_paths }
        } else if let Some(compile_matches) = matches.subcommand_matches("build") {
            let files = compile_matches.values_of("files").unwrap();
            let mut file_paths = vec![];
            for file in files {
                file_paths.push(PathBuf::from(file));
            }

            Self::Build { files: file_paths }
        } else if let Some(compile_matches) = matches.subcommand_matches("run") {
            let files = compile_matches.values_of("files").unwrap();
            let mut file_paths = vec![];
            for file in files {
                file_paths.push(PathBuf::from(file));
            }

            Self::Run { files: file_paths }
        } else {
            Self::Unknown
        }
    }
}

pub fn compile_command(
    files: Vec<PathBuf>,
) -> Result<(Vec<PathBuf>, Option<PathBuf>), Box<dyn Error>> {
    let mut error_reporter = ErrorReporter::new(vec![]);
    let mut interner = Interner::default();
    let mut clean_files = vec![];
    let mut sources = vec![];
    for file in files {
        let file = clean_file(file)?;
        let source = get_source(&file)?;
        clean_files.push(file);
        sources.push(source);
    }

    let mut main_file = None;
    for (file, source) in clean_files.iter().zip(sources.iter()) {
        error_reporter.add(path_to_str(file)?, source);
        let file_stem = get_stem(file)?;
        let file_path = path_to_str(file)?;
        let output_file = replace_last(file, format!("{}.o", file_stem))?;
        let output_file_path = path_to_str(&output_file)?;
        let bytes = source.as_bytes();
        let compilation_start = Instant::now();
        let result = compile_code(
            &error_reporter,
            &mut interner,
            file_stem,
            file_path,
            output_file_path,
            bytes,
        );

        if let Some(found_main) = result {
            match main_file {
                Some(_) if found_main => return Err(error("Found multiple main methods.")),
                None if found_main => main_file = Some(file.clone()),
                _ => {}
            }
        }

        if result.is_none() {
            println!("Failed to compile file `{}`.", file_path);
            return Ok((clean_files, main_file));
        } else {
            println!(
                "Finished full compilation process for file `{}` after {} seconds.",
                file_path,
                compilation_start.elapsed().as_secs_f64()
            );
        }
    }

    Ok((clean_files, main_file))
}
