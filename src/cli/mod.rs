/// The arguments module, which holds all of the arguments to the program.
pub mod arguments;

use crate::repl_helpers::{envy_repl::EnvyRepl, repl_support::ReplSupport};
use arguments::Arguments;
use std::{fs, time::Instant};

/// This function runs the function that is passed in. Typically, this function will handle lexing, parsing, and compiling the code,
/// while this function deals with the arguments that were passed in to the program.
/// The function passed in must take is the contents of the provided file and the path to the file.
///
/// # Arguments
/// `envy` - A function that deals with the task of the running the Envious code.
pub fn runner<F: Fn(&str, &str, &Arguments) -> Result<String, String>>(
    envy: F,
) -> Result<(), String> {
    let args = Arguments::new().map_err(|error| error.prettify(""))?;
    if args.get_path().is_none() {
        ReplSupport::new()
            .map_err(|_| "An Error Occurred When Interacting With The REPL.".to_owned())?
            .run(EnvyRepl::new())
            .map_err(|_| "An Error Occurred When Interacting With The REPL.".to_owned())?;
        Ok(())
    } else if let Some(path) = args.get_path().filter(|path| path.ends_with(".envy")) {
        let contents = fs::read_to_string(path)
            .map_err(|_| "An Error Occurred.\nThe Path Provided Is Not Valid.".to_owned())?;
        let start = Instant::now();
        let result = envy(&contents, path, &args);
        let elapsed = start.elapsed();
        if let Err(error) = result {
            return Err(error);
        } else if args.show_time() {
            println!("Time Taken: {:#?}", elapsed)
        }

        Ok(())
    } else {
        generate_error("Expected The File Passed In To Be An Envious File.")
    }
}

/// This function generates an error based on the error message provided.
///
/// # Arguments
/// `error_message` - The error message to return.
fn generate_error(error_message: &str) -> Result<(), String> {
    Err(format!("An Error Occurred.\n{}", error_message))
}
