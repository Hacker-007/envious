//! All of the functions related to IO are here. This may change to become a folder if the number of functions increases.

use crate::errors::{error_kind::ErrorKind, error::Error};
use super::standard_library::Return;

/// This function prints out all of the parameters seperated by a space.
///
/// # Arguments
/// `pos` - The position where this function was called.
/// `indent` - The current level of indent.
/// `parameters` - The parameters provided.
pub fn print(pos: usize, indent: &str, parameters: &[String]) -> Return {
    if parameters.len() > 1 {
        Err(Error::new(ErrorKind::WrongNumberOfParameters, pos))
    } else {
        let mut result = String::new();
        for parameter in parameters {
            result.push_str(parameter);
            result.push('\n');
        }

        Ok(format!("{}{}print pop", result, indent))
    }
}

/// This function prints out all of the parameters seperated by a space. Additionally, the new line character is printed.
///
/// # Arguments
/// `pos` - The position where this function was called.
/// `indent` - The current level of indent.
/// `parameters` - The parameters provided.
pub fn println(pos: usize, indent: &str, parameters: &[String]) -> Return {
    if parameters.len() > 1 {
        Err(Error::new(ErrorKind::WrongNumberOfParameters, pos))
    } else {
        let mut result = String::new();
        for parameter in parameters.iter() {
            result.push_str(parameter);
            result.push('\n');
        }

        Ok(format!("{}{}printn pop", result, indent))
    }
}