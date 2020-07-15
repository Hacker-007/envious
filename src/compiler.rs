//! The Compiler struct compiles the AST. It generates a dark file that can then be invoked by the DarkVM.
//! For now, the Compiler does not return any errors because there is no semantic checking.
//! The Compiler expects that the code written is correct.
//!
//! The compiler must be the last thing that is invoked because it requires the AST from the parsing stage.
//!
//! # Example
//! ```
//! # fn run() -> Result<(), Error> {
//! let contents = "1 + 1";
//! let tokens = Lexer::default().lex(contents)?;
//! let ast = Parser::new(tokens).parse()?;
//! Compiler::new(ast, "test.dark").compile()?;
//! # Ok(())
//! # }
//! ```

use crate::{
    ast::{
        expression::Expression,
        expression_kind::{
            BinaryEqualityOperation, BinaryOperation, ExpressionKind, UnaryOperation,
        },
    },
    errors::{error::Error, error_kind::ErrorKind},
};
use std::{fs::File, io::Write};

pub struct Compiler {
    label_value: usize,
    token_idx: usize,
}

impl Compiler {
    /// Constructs a new Compiler.
    pub fn new() -> Compiler {
        Compiler {
            label_value: 0,
            token_idx: 0,
        }
    }

    /// Compiles the AST into the .dark file specified by the file name.
    /// This returns an error if it could not compile some of the AST.
    ///
    /// # Arguments
    /// * `dark_file_path` - The path to the .dark file
    pub fn compile(&mut self, dark_file_path: &str, ast: Vec<Expression>) -> Result<(), Error> {
        let dark_file = Compiler::create_dark_file(dark_file_path)?;
        let mut contents = "@main".to_owned();
        self.token_idx += 1;
        let iter = ast.iter();
        for expression in iter {
            contents = format!("{}\n{}", contents, self.compile_expression(expression));
        }

        contents.push_str("\nend");
        self.token_idx += 1;
        Compiler::write_to_dark_file(dark_file, contents, dark_file_path)
    }

    /// Converts the expression provided into a String. Internally, this function performs a match on the kind
    /// and delegates the work to a seperate function. This recursive function helps reduce the code to write.
    ///
    /// # Arguments
    /// `expression` - The expression to convert.
    fn compile_expression(&mut self, expression: &Expression) -> String {
        match &expression.kind {
            ExpressionKind::Int(value) => self.compile_int_expression(*value),
            ExpressionKind::Float(value) => self.compile_float_expression(*value),
            ExpressionKind::Boolean(value) => self.compile_boolean_expression(*value),
            ExpressionKind::String(value) => self.compile_string_expression(value),
            ExpressionKind::Identifier(name) => self.compile_identifier_expression(name),
            ExpressionKind::InfixBinaryExpression(operation, left, right) => {
                self.compile_infix_binary_expression(operation, left, right)
            }
            ExpressionKind::UnaryExpression(operation, expression) => {
                self.compile_unary_expression(operation, expression)
            }
            ExpressionKind::BinaryEqualityExpression(operation, left, right) => {
                self.compile_binary_equality_expression(operation, left, right)
            }
            ExpressionKind::LetExpression(name, _, value) => {
                self.compile_let_expression(name, value)
            }
            ExpressionKind::BlockExpression(expressions) => {
                self.compile_block_expression(expressions)
            }
            ExpressionKind::IfExpression(condition, expression) => {
                self.compile_if_expression(condition, expression)
            }
            _ => todo!(),
        }
    }

    /// Converts the Int expression provided into a String.
    ///
    /// # Arguments
    /// `value` - The value of the int expression.
    fn compile_int_expression(&mut self, value: i64) -> String {
        self.token_idx += 2;
        format!("push {}", value)
    }

    /// Converts the Float expression provided into a String.
    ///
    /// # Arguments
    /// `value` - The value of the Float expression.
    fn compile_float_expression(&mut self, value: f64) -> String {
        self.token_idx += 2;
        format!("push {}", value)
    }

    /// Converts the Boolean expression provided into a String.
    ///
    /// # Arguments
    /// `value` - The value of the Boolean expression.
    fn compile_boolean_expression(&mut self, value: bool) -> String {
        self.token_idx += 2;
        format!("push {}", value)
    }

    /// Converts the String expression provided into a String.
    ///
    /// # Arguments
    /// `value` - The value of the String expression.
    fn compile_string_expression(&mut self, value: &str) -> String {
        self.token_idx += 2;
        format!("push '{}'", value)
    }

    /// Converts the Identifier expression provided into a String.
    ///
    /// # Arguments
    /// `name` - The name of the Identifier expression.
    fn compile_identifier_expression(&mut self, name: &str) -> String {
        self.token_idx += 2;
        format!("push {}", name)
    }

    /// Converts an Infix Binary expression provided into a String.
    /// It takes the different parts of the expression and recursively generates the left and the right before performing the operation,
    /// thus resembling a postfix traversal.
    ///
    /// # Arguments
    /// `operation` - The Binary Operation to compile.
    /// `left` - The left sub-expression to compile.
    /// `right` - The right sub-expression to compile.
    /// `label_value` - The value of the current temporary label.
    fn compile_infix_binary_expression(
        &mut self,
        operation: &BinaryOperation,
        left: &Expression,
        right: &Expression,
    ) -> String {
        let operation_instruction = match operation {
            BinaryOperation::Plus => "add",
            BinaryOperation::Minus => "sub",
            BinaryOperation::Multiply => "mul",
            BinaryOperation::Divide => "div",
        };

        let compiled = format!(
            "{}\n{}\npush {}",
            self.compile_expression(right),
            self.compile_expression(left),
            operation_instruction
        );

        self.token_idx += 2;
        compiled
    }

    /// Converts an Unary expression provided into a String.
    /// It takes the different parts of the expression and recursively generates the expression before performing the operation,
    /// thus resembling a postfix traversal.
    ///
    /// # Arguments
    /// `operation` - The Binary Operation to compile.
    /// `expression` - The left expression to compile.
    /// `label_value` - The value of the current temporary label.
    fn compile_unary_expression(
        &mut self,
        operation: &UnaryOperation,
        expression: &Expression,
    ) -> String {
        let operation_instruction = match operation {
            UnaryOperation::Positive => "",
            UnaryOperation::Negative => {
                self.token_idx += 4;
                "push -1\npush mul"
            }
        };

        format!(
            "{}\n{}",
            self.compile_expression(expression),
            operation_instruction
        )
    }

    /// Converts a Binary Equality expression provided into a String.
    /// It depends on both of the sub-expressions to be pushed on to the stack.
    /// This is because the DarkVM code generated pops the top two values.
    ///
    /// # Arguments
    /// `operation` - The Binary Equality operation to compile.
    /// `left` - The left sub-expression to compile.
    /// `right` - The right sub-expression to compile.
    /// `label_value` - The value of the current temporary label.
    fn compile_binary_equality_expression(
        &mut self,
        operation: &BinaryEqualityOperation,
        left: &Expression,
        right: &Expression,
    ) -> String {
        let operation_instruction = match operation {
            BinaryEqualityOperation::Equals => "eq",
        };

        let compiled = format!(
            "{}\n{}\npush {} pop pop",
            self.compile_expression(right),
            self.compile_expression(left),
            operation_instruction
        );

        self.token_idx += 4;
        compiled
    }

    /// Converts a Let expression provided into a String.
    /// It will take the value and generate the code necessary for that first.
    /// In the case that a value is not provided, the default is set to void.
    ///
    /// # Arguments
    /// `name` - The name of the variable.
    /// `value` - The value of the variable. This is optional.
    /// `label_value` - The value of the current temporary label.
    fn compile_let_expression(&mut self, name: &str, value: &Option<Box<Expression>>) -> String {
        let compiled = if let Some(expression) = value {
            format!("{}\nset {} pop", self.compile_expression(expression), name)
        } else {
            format!("set {} void", name)
        };

        self.token_idx += 3;
        compiled
    }

    /// Converts a Block expression provided into a String.
    /// It takes the vector of expressions and places it into a label in the dark code.
    ///
    /// # Arguments
    /// `expressions` - The expressions in the block statement.
    /// `label_value` - The value of the current temporary label.
    fn compile_block_expression(&mut self, expressions: &[Expression]) -> String {
        let label_value = self.label_value;
        let mut created_label = format!("@__{}__", label_value);
        self.label_value += 1;
        self.token_idx += 1;
        for expression in expressions {
            created_label = format!("{}\n{}", created_label, self.compile_expression(expression));
        }

        self.token_idx += 3;
        format!("{}\nend\ncall __{}__", created_label, label_value)
    }

    /// Converts an If expression provided into a String.
    /// It takes the condition and the expression and first generates the code for the condition.
    /// If the condition is true, then a jump instruction is used to jump to the correct location.
    /// Otherwise, it skips the expression.
    ///
    /// # Arguments
    /// `condition` - The condition of the if expression.
    /// `expression` - The expression to execute if the condition is true.
    fn compile_if_expression(&mut self, condition: &Expression, expression: &Expression) -> String {
        let compiled_condition = self.compile_expression(condition);
        let compiled_expression = self.compile_expression(expression);

        format!(
            "{}\njmpf {}\n{}",
            compiled_condition,
            self.token_idx + 2,
            compiled_expression
        )
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
        })?;

        dark_file.flush().map_err(|_| {
            Error::message_only(ErrorKind::SystemError(format!(
                "An Error Occurred When Writing To The File {}.",
                dark_file_path
            )))
        })?;

        Ok(())
    }
}
