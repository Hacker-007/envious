use std::{
    error::Error,
    path::{Path, PathBuf},
    process,
    time::Instant,
};

use clap::{App, AppSettings, Arg, SubCommand};
use command::Command;
use envious_tui::run_tui;
use envyc::{
    compile,
    environment::Environment,
    error::reporter::{ErrorReporter, Reporter},
    filter_tokens,
    function_table::FunctionTable,
    interner::Interner,
    lex, parse,
    semantic_analyzer::types::Type,
    type_check, Config,
};
use home::home_dir;

use crate::{
    command::compile_command,
    utils::{error, get_stem, path_to_str, replace_last},
};

pub mod command;
pub mod utils;

pub fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("envious")
        .version("0.0.1")
        .author("Revanth Pothukuchi <revanthpothukuchi123@gmail.com>")
        .about("The CLI for the Envious Programming Language")
        .arg(
            Arg::with_name("tui")
                .short("t")
                .long("tui")
                .help("Starts the terminal editor"),
        )
        .subcommand(
            SubCommand::with_name("compile")
                .about("Compiles the files without linking them")
                .arg(
                    Arg::with_name("files")
                        .short("f")
                        .long("files")
                        .min_values(1)
                        .value_delimiter(";")
                        .required(true)
                        .help("The files to compile"),
                ),
        )
        .subcommand(
            SubCommand::with_name("build")
                .about("Compiles the files while linking them")
                .arg(
                    Arg::with_name("files")
                        .short("f")
                        .long("files")
                        .min_values(1)
                        .value_delimiter(";")
                        .required(true)
                        .help("The files to compile and link"),
                ),
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Run the main file after compiling and linking")
                .arg(
                    Arg::with_name("files")
                        .short("f")
                        .long("files")
                        .min_values(1)
                        .value_delimiter(";")
                        .required(true)
                        .help("The files to run after compiling and linking"),
                ),
        )
        .settings(&[AppSettings::ArgRequiredElseHelp, AppSettings::ColorAlways])
        .get_matches();

    let command = Command::from(matches);
    match command {
        Command::Tui => run_tui()?,
        Command::Compile { files } => {
            compile_command(files)?;
        }
        Command::Build { files } => {
            let (files, main_file) = compile_command(files)?;
            if let Some(ref main_file) = main_file {
                build_static_files(&files, main_file)?;
            } else {
                return Err(error("No main method could be found."));
            }
        }
        Command::Run { files } => {
            let (files, main_file) = compile_command(files)?;
            if let Some(ref main_file) = main_file {
                build_static_files(&files, main_file)?;
                run(path_to_str(&replace_last(
                    main_file,
                    get_stem(main_file)?,
                )?)?)?;
            } else {
                return Err(error("No main method could be found."));
            }
        }
        Command::Unknown => return Err(error("Unrecognized command")),
    }

    Ok(())
}

fn compile_code(
    error_reporter: &ErrorReporter,
    interner: &mut Interner<String>,
    module_name: &str,
    file_path: &str,
    output_file_path: &str,
    bytes: &[u8],
) -> Option<bool> {
    let tokens = time("Lexing", &error_reporter, || {
        lex(file_path, bytes, interner)
    })?;

    let filtered_tokens = filter_tokens(tokens);
    let program = time("Parsing", &error_reporter, || parse(filtered_tokens))?;

    let mut type_env = Environment::default();
    let mut function_table = FunctionTable::default();
    let typed_program = time("Checking", &error_reporter, || {
        type_check(program, &mut type_env, &mut function_table)
    })?;

    time("Compiling", &error_reporter, || {
        let config = Config {
            writing_to_file: true,
            output_file_path,
        };

        compile(&typed_program, module_name, interner, Some(config))
    })?;

    let contains_main = typed_program.functions.iter().any(|function| {
        function.prototype.name == interner.insert("main".to_string())
            && function.prototype.parameters.is_empty()
            && function.prototype.return_type == Type::Void
    });

    Some(contains_main)
}

fn time<O: Reporter>(
    name: &str,
    error_reporter: &ErrorReporter,
    function: impl FnOnce() -> O,
) -> Option<O::Output> {
    let start = Instant::now();
    let value = function();
    println!(
        "Process `{}` took {} seconds.",
        name,
        start.elapsed().as_secs_f64()
    );
    value.report(error_reporter, true)
}

fn build_static_files(files: &[PathBuf], main_file_path: &Path) -> Result<(), Box<dyn Error>> {
    let mut command = process::Command::new("g++");
    for file in files {
        let file_stem = get_stem(file)?;
        command.arg(replace_last(file, format!("{}.o", file_stem))?);
    }

    let executable_path = replace_last(main_file_path, get_stem(main_file_path)?.to_string())?;
    let std_path = home_dir().ok_or("Could not find home directory.")?.join(".envious/std/std.o");
    let output = command
        .arg(std_path)
        .arg("-o")
        .arg(&executable_path)
        .output()?;

    if !output.status.success() {
        return Err(error("Failed to link files"));
    }

    Ok(())
}

fn run(executable_path: &str) -> Result<(), Box<dyn Error>> {
    let output = process::Command::new(executable_path).output()?;

    if !output.stdout.is_empty() {
        print!("{}", String::from_utf8(output.stdout)?);
    }

    if !output.stderr.is_empty() {
        print!("{}", String::from_utf8(output.stderr)?);
    }

    Ok(())
}
