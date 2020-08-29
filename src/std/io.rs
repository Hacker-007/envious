//! All of the functions related to IO are here. This may change to become a folder if the number of functions increases.

use super::standard_library::Return;
use crate::errors::{error::Error, error_kind::ErrorKind};

/// This function prints out the parameter.
///
/// # Arguments
/// `pos` - The position where this function was called.
/// `indent` - The current level of indent.
/// `parameters` - The parameters provided.
pub fn print(pos: usize, indent: &str, parameters: &[String]) -> Return {
    if parameters.len() != 1 {
        Err(Error::new(ErrorKind::WrongNumberOfParameters, pos))
    } else {
        Ok((format!("{}\n{}print pop", parameters.get(0).unwrap(), indent), 2))
    }
}

/// This function prints out the given parameter. Additionally, the new line character is printed.
///
/// # Arguments
/// `pos` - The position where this function was called.
/// `indent` - The current level of indent.
/// `parameters` - The parameters provided.
pub fn println(pos: usize, indent: &str, parameters: &[String]) -> Return {
    if parameters.len() != 1 {
        Err(Error::new(ErrorKind::WrongNumberOfParameters, pos))
    } else {
        Ok((format!("{}\n{}printn pop", parameters.get(0).unwrap(), indent), 2))
    }
}

// /// This function reads the contents of a file and returns it.
// ///
// /// # Arguments
// /// `pos` - The position where this function was called.
// /// `indent` - The current level of indent.
// /// `parameters` - The parameters provided.
// pub fn read_file(pos: usize, indent: &str, parameters: &[String]) -> Return {
//     if parameters.len() != 1 {
//         Err(Error::new(ErrorKind::WrongNumberOfParameters, pos))
//     } else {
//         Ok(format!("{}\n{}call read_file pop\n", parameters.get(0).unwrap(), indent))
//     }
// }
