use crossterm::{terminal, cursor, style::Colorize, ExecutableCommand};
use std::{fmt::Display, io::{Write, Stdout}, cmp::min};
use super::repl_trait::Repl;

static PROMPT: &'static str          = "envious -> ";
static NEW_LINE_PROMPT: &'static str = "         | ";

pub struct SubmissionView {
    pub document: Vec<String>,
    pub current_row: usize,
    pub current_line: usize,
    pub current_char: usize,
    pub rendered_lines: usize,
}

impl SubmissionView {
    pub fn new<T: Repl>(document: Vec<String>, stdout: &mut Stdout, repl: &mut T) -> crossterm::Result<SubmissionView> {
        let mut submission_view = SubmissionView {
            document,
            current_row: cursor::position()?.1 as usize,
            current_line: 0,
            current_char: 0,
            rendered_lines: 0,
        };

        submission_view.render(stdout, repl)?;
        Ok(submission_view)
    }

    pub fn render<T: Repl>(&mut self, stdout: &mut Stdout, repl: &mut T) -> crossterm::Result<()> {
        stdout.execute(cursor::Hide)?;
        let mut line_count = 0;
        let (width, height) = terminal::size().map(|(w, h)| (w as usize, h as usize))?;
        for line in &self.document {
            if self.current_row + line_count >= height {
                stdout.execute(cursor::MoveTo(0, (height - 1) as u16))?;
                print(stdout, "\n")?;
                if self.current_row > 0 {
                    self.current_row -= 1;
                }
            }

            stdout.execute(cursor::MoveTo(0, (self.current_row + line_count) as u16))?;
            if line_count == 0 {
                print(stdout, PROMPT.dark_grey())?;
            } else {
                print(stdout, NEW_LINE_PROMPT.dark_grey())?;
            }
            
            repl.render_line(stdout, &self.document, line_count)?;
            print(stdout, " ".repeat(width - line.len() - PROMPT.len()))?;
            line_count += 1;
        }

        if self.rendered_lines > line_count {
            let num_blank_lines = self.rendered_lines - line_count;
            if num_blank_lines > 0 {
                let blank_line = " ".repeat(width);
                for i in 0..num_blank_lines {
                    stdout.execute(cursor::MoveTo(0, (self.current_row + line_count + i) as u16))?;
                    println(stdout, &blank_line)?;
                }
            }
        }

        self.rendered_lines = line_count;
        stdout.execute(cursor::Show)?;
        self.update_cursor_position(stdout)?;

        Ok(())
    }

    pub fn set_current_line(&mut self, stdout: &mut Stdout, value: usize) -> crossterm::Result<()> {
        if self.current_line != value {
            self.current_line = value;
            self.current_char = min(self.document.get(self.current_line).map_or(0, |line| line.len()), self.current_char);
            self.update_cursor_position(stdout)?;
        }

        Ok(())
    }

    pub fn set_current_char(&mut self, stdout: &mut Stdout, value: usize) -> crossterm::Result<()> {
        if self.current_char != value {
            self.current_char = value;
            self.update_cursor_position(stdout)?;
        }

        Ok(())
    }
 
    pub fn update_cursor_position(&mut self, stdout: &mut Stdout) -> crossterm::Result<()> {
        stdout.execute(cursor::MoveTo((self.current_char + PROMPT.len()) as u16, (self.current_row + self.current_line) as u16))?;
        Ok(())
    }
}

fn print<T: Display>(stdout: &mut Stdout, text: T) -> crossterm::Result<()> {
    write!(stdout, "{}", text)?;
    stdout.flush()?;
    Ok(())
}

fn println<T: Display>(stdout: &mut Stdout, text: T) -> crossterm::Result<()> {
    writeln!(stdout, "{}", text)?;
    stdout.flush()?;
    Ok(())
}