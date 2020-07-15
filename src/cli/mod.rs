/// The Arguments module, which holds all of the arguments to the program.
pub mod arguments;

use arguments::Arguments;
use std::{fs, time::Instant};

/// This function runs the function that is passed in. Typically, this function will handle lexing, parsing, and compiling the code,
/// while this function deals with the arguments that were passed in to the program.
/// The function passed in must take is the contents of the provided file and the path to the file.
///
/// # Arguments
/// `envy` - A function that deals with the task of the running the Envious code.
pub fn runner<F: Fn(&str, &str) -> Result<(), String>>(envy: F) -> Result<(), String> {
    let args = Arguments::new().map_err(|error| error.prettify(""))?;
    if args.get_path().is_none() {
        generate_error("The REPL Is Not Yet Supported.")
    } else if let Some(path) = args.get_path().filter(|path| path.ends_with(".envy")) {
        let contents = fs::read_to_string(path)
            .map_err(|_| "An Error Occurred.\nThe Path Provided Is Not Valid.".to_owned())?;
        let start = Instant::now();
        if let Err(error) = envy(&contents, path) {
            return Err(error);
        } else if args.show_time() {
            println!("Time Taken: {:#?}", start.elapsed())
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
