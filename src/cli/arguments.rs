use crate::errors::{error::Error, error_kind::ErrorKind};
use std::{env, fs, time::Instant};

pub struct Arguments {
    path: Option<String>,
    show_time: bool,
}

impl Arguments {
    pub fn new() -> Result<Arguments, Error> {
        let args = env::args().skip(1);
        let mut arguments = Arguments {
            path: None,
            show_time: false,
        };

        for arg in args {
            match arg.as_str() {
                "-t" | "--show-time" => arguments.show_time = true,
                _ if arguments.path.is_none() => arguments.path = Some(arg),
                _ => return Err(Error::message_only(ErrorKind::UnrecognizedArgument(arg))),
            }
        }

        Ok(arguments)
    }

    pub fn run<F: Fn(&str, &Arguments) -> Result<(), String>>(&self, f: F) -> Result<(), String> {
        if self.path.is_none() {
            generate_error("Expected The Path To The Envious File.")
        } else if self
            .path
            .as_ref()
            .filter(|path| path.ends_with(".envy"))
            .is_some()
        {
            let contents = fs::read_to_string(self.path.as_ref().unwrap())
                .map_err(|_| "An Error Occurred.\nThe Path Provided Is Not Valid.".to_owned())?;
            let start = Instant::now();
            if let Err(error) = f(&contents, &self) {
                return Err(error);
            } else if self.show_time {
                println!("Time Taken: {:#?}", start.elapsed())
            }

            Ok(())
        } else {
            generate_error("Expected The File Passed In To Be An Envious File.")
        }
    }

    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }
}

fn generate_error(error_message: &str) -> Result<(), String> {
    Err(format!("An Error Occurred.\n{}", error_message))
}
