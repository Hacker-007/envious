//! The Compiler struct compiles the AST. It generates a dark file that can then be invoked by the DarkVM.
//! The Compiler may prematurely return an error if it could not compile part of the AST.
//!
//! The compiler must be the last thing that is invoked because it requires the AST from the parsing stage.
//!
//! # Example
//! ```
//! # fn run() -> Result<(), Error> {
//! let contents = "print(1)";
//! let tokens = Lexer::default().lex(contents)?;
//! let ast = Parser::new(tokens).parse()?;
//! Compiler::new(ast, "test.dark").compile()?;
//! # Ok(())
//! # }
//! ```

use crate::{
    ast::{expression::Expression, expression_kind::{BinaryOperation, ExpressionKind, UnaryOperation}},
    errors::{error::Error, error_kind::ErrorKind},
};
use std::{fs::File, io::Write};

pub struct Compiler {
    ast: Vec<Expression>,
}

impl Compiler {
    /// Constructs a new Compiler with the given AST. This AST comes from the parser and, therefore must be invoked after the parser.
    ///
    /// # Arguments
    /// * `ast` - The AST to compile.
    pub fn new(ast: Vec<Expression>) -> Compiler {
        Compiler { ast }
    }

    /// Compiles the AST into the .dark file specified by the file name.
    /// This returns an error if it could not compile some of the AST.
    ///
    /// # Arguments
    /// * `dark_file_path` - The path to the .dark file
    pub fn compile(&self, dark_file_path: &str) -> Result<(), Error> {
        let dark_file = Compiler::create_dark_file(dark_file_path)?;
        let mut contents = "@main\n".to_owned();
        let mut iter = self.ast.iter();
        while let Some(expression) = iter.next() {
            contents = format!("{}{}\n", contents, self.compile_expression(expression));
        }

        contents.push_str("end");
        Compiler::write_to_dark_file(dark_file, contents, dark_file_path)
    }

    /// Converts the expression provided into a String. Internally, this function performs a match on the kind
    /// and delegates the work to a seperate function. This recursive function helps reduce the code to write.
    /// 
    /// # Arguments
    /// `expression` - The expression to convert.
    fn compile_expression(&self, expression: &Expression) -> String {
        match &expression.kind {
            ExpressionKind::Int(value) => self.compile_int_expression(value),
            ExpressionKind::Float(value) => self.compile_float_expression(value),
            ExpressionKind::Boolean(value) => self.compile_boolean_expression(value),
            ExpressionKind::String(value) => self.compile_string_expression(value),
            ExpressionKind::Identifier(name) => self.compile_identifier_expression(name),
            ExpressionKind::InfixBinaryExpression(operation, left, right) => self.compile_infix_binary_expression(operation, left, right),
            ExpressionKind::UnaryExpression(operation, expression) => self.compile_unary_expression(operation, expression),
            _ => todo!(),
        }
    }

    /// Converts the Int expression provided into a String.
    /// 
    /// # Arguments
    /// `value` - The value of the int expression.
    fn compile_int_expression(&self, value: &i64) -> String {
        format!("push {}", value)
    }

    /// Converts the Float expression provided into a String.
    /// 
    /// # Arguments
    /// `value` - The value of the Float expression.
    fn compile_float_expression(&self, value: &f64) -> String {
        format!("push {}", value)
    }

    /// Converts the Boolean expression provided into a String.
    /// 
    /// # Arguments
    /// `value` - The value of the Boolean expression.
    fn compile_boolean_expression(&self, value: &bool) -> String {
        format!("push {}", value)
    }

    /// Converts the String expression provided into a String.
    /// 
    /// # Arguments
    /// `value` - The value of the String expression.
    fn compile_string_expression(&self, value: &String) -> String {
        // TODO: Change to single quotes.
        format!("push \"{}\"", value)
    }

    /// Converts the Identifier expression provided into a String.
    /// 
    /// # Arguments
    /// `name` - The name of the Identifier expression.
    fn compile_identifier_expression(&self, name: &String) -> String {
        format!("{}", name)
    }

    /// Converts an Infix Binary expression provided into a String.
    /// It takes the different parts of the expression and recursively generates the left and the right before performing the operation,
    /// thus resembling a postfix traversal.
    ///
    /// # Arguments
    /// `operation` - The Binary Operation to compile
    /// `left` - The left sub-expression to compile
    /// `right` - The right sub-expression to compile
    fn compile_infix_binary_expression(&self, operation: &BinaryOperation, left: &Box<Expression>, right: &Box<Expression>) -> String {
        let operation_instruction = match operation {
            BinaryOperation::Plus => "add",
            BinaryOperation::Minus => "sub",
            BinaryOperation::Multiply => "mul",
            BinaryOperation::Divide => "div",
        };

        format!("{}\n{}\npush {}", self.compile_expression(right), self.compile_expression(left), operation_instruction)
    }

    /// Converts an Unary expression provided into a String.
    /// It takes the different parts of the expression and recursively generates the expression before performing the operation,
    /// thus resembling a postfix traversal.
    ///
    /// # Arguments
    /// `operation` - The Binary Operation to compile
    /// `expression` - The left expression to compile
    fn compile_unary_expression(&self, operation: &UnaryOperation, expression: &Box<Expression>) -> String {
        let operation_instruction = match operation {
            UnaryOperation::Positive => "",
            UnaryOperation::Negative => "\npush -1\npush mul",
        };

        format!("{}{}", self.compile_expression(expression), operation_instruction)
    }

    /// Creates the .dark file based on the path provided
    ///
    /// # Arguments
    /// * `dark_file_path` - The path to the .dark file
    fn create_dark_file(dark_file_path: &str) -> Result<File, Error> {
        File::create(dark_file_path).map_err(|_| {
            Error::message_only(ErrorKind::SystemError(format!(
                "An Error Occurred When Creating The File {}.",
                dark_file_path
            )))
        })
    }

    /// Writes the given contents to the file.
    ///
    /// # Arguments
    /// * `dark_file` - The .dark file to write to
    /// * `contents` - The contents to write
    /// * `dark_file_path` - The path to the .dark file
    fn write_to_dark_file(
        mut dark_file: File,
        contents: String,
        dark_file_path: &str,
    ) -> Result<(), Error> {
        dark_file.write_all(contents.as_bytes()).map_err(|_| {
            Error::message_only(ErrorKind::SystemError(format!(
                "An Error Occurred When Writing To The File {}.",
                dark_file_path
            )))
        })
    }
}
