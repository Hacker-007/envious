use std::io::{Write, Stdout};

pub trait Repl {
    fn evaluate_submission(&mut self, stdout: &mut Stdout, text: &String) -> crossterm::Result<()>;
    
    fn render_line(&mut self, stdout: &mut Stdout, lines: &[String], line_index: usize) -> crossterm::Result<()> {
        write!(stdout, "{}", lines.get(line_index).unwrap())?;
        Ok(())
    }
}