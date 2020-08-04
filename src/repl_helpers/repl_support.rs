use crossterm::{event::{Event, read, KeyEvent, KeyCode, KeyModifiers}};
use std::{fmt::Display, io::{stdout, Write, Stdout}, cmp::max};
use super::{repl_trait::Repl, submission_view::SubmissionView};

pub struct ReplSupport {
    submission_history: Vec<String>,
    submission_history_index: usize,
    done: bool
}

impl ReplSupport {
    pub fn new() -> crossterm::Result<ReplSupport> {
        Ok(ReplSupport {
            submission_history: vec![],
            submission_history_index: 0,
            done: false,
        })
    }

    pub fn run<T: Repl>(&mut self, mut repl: T) -> crossterm::Result<()> {
        let mut stdout = stdout();
        loop {
            let text = self.edit_submission(&mut stdout, &mut repl)?;
            if text.is_empty() {
                break;
            }

            repl.evaluate_submission(&mut stdout, &text).ok();
            self.submission_history.push(text);
            self.submission_history_index = 0;
        }

        Ok(())
    }

    fn edit_submission<T: Repl>(&mut self, stdout: &mut Stdout, repl: &mut T) -> crossterm::Result<String> {
        self.done = false;
        let mut view = SubmissionView::new(vec![String::new()], stdout, repl)?;
        while !self.done {
            let event = read()?;
            if let Event::Key(event) = event {
                self.handle_key(stdout, &mut view, event, repl)?;
            }
        }

        view.set_current_line(stdout, view.document.len() - 1)?;
        view.set_current_char(stdout, view.document.get(view.current_line).map_or(0, |line| line.len()))?;
        print(stdout, "\n")?;

        Ok(view.document.join("\n"))
    }

    fn handle_key<T: Repl>(&mut self, stdout: &mut Stdout, view: &mut SubmissionView, event: KeyEvent, repl: &mut T) -> crossterm::Result<()> {
        if event.modifiers == KeyModifiers::NONE {
            match event.code {
                KeyCode::Esc => self.handle_escape(stdout, view, repl)?,
                KeyCode::Enter => self.handle_enter(stdout, view, repl)?,
                KeyCode::Left => self.handle_left_arrow(stdout, view)?,
                KeyCode::Right => self.handle_right_arrow(stdout, view)?,
                KeyCode::Up => self.handle_up_arrow(stdout, view)?,
                KeyCode::Down => self.handle_down_arrow(stdout, view)?,
                KeyCode::Backspace => self.handle_backspace(stdout, view, repl)?,
                KeyCode::Delete => self.handle_delete(stdout, view, repl)?,
                KeyCode::Home => self.handle_home(stdout, view)?,
                KeyCode::End => self.handle_end(stdout, view)?,
                KeyCode::Tab => self.handle_tab(stdout, view)?,
                KeyCode::PageUp => self.handle_page_up(stdout, view, repl)?,
                KeyCode::PageDown => self.handle_page_down(stdout, view, repl)?,
                KeyCode::Char(c) => self.handle_typing(stdout, view, c, repl)?,
                _ => {},
            }
        } else if event.modifiers == KeyModifiers::SHIFT {
            match event.code {
                KeyCode::Char(c) => {
                    self.handle_typing(stdout, view, c, repl)?;
                }
                _ => {},
            }
        } else if event.modifiers == KeyModifiers::ALT {
            match event.code {
                KeyCode::Enter => self.done = true,
                _ => {},
            }
        }

        Ok(())
    }

    fn handle_escape<T: Repl>(&mut self, stdout: &mut Stdout, view: &mut SubmissionView, repl: &mut T) -> crossterm::Result<()> {
        view.document.clear();
        view.document.push(String::new());
        view.render(stdout, repl)?;
        view.set_current_line(stdout, 0)?;
        view.set_current_char(stdout, 0)?;
        Ok(())
    }

    fn handle_enter<T: Repl>(&mut self, stdout: &mut Stdout, view: &mut SubmissionView, repl: &mut T) -> crossterm::Result<()> {
        self.insert_line(stdout, view, repl)?;
        Ok(())
    }

    fn handle_left_arrow(&mut self, stdout: &mut Stdout, view: &mut SubmissionView) -> crossterm::Result<()> {
        if view.current_char > 0 {
            view.set_current_char(stdout, view.current_char - 1)?;
        }

        Ok(())
    }

    fn handle_right_arrow(&mut self, stdout: &mut Stdout, view: &mut SubmissionView) -> crossterm::Result<()> {
        let len = view.document.get(view.current_line).unwrap().len();
        if len > 0 && view.current_char <= len - 1 {
            view.set_current_char(stdout, view.current_char + 1)?;
        }

        Ok(())
    }

    fn handle_up_arrow(&mut self, stdout: &mut Stdout, view: &mut SubmissionView) -> crossterm::Result<()> {
        if view.current_line > 0 {
            view.set_current_line(stdout, view.current_line - 1)?;
        }

        Ok(())
    }

    fn handle_down_arrow(&mut self, stdout: &mut Stdout, view: &mut SubmissionView) -> crossterm::Result<()> {
        let len = view.document.len();
        if len > 0 && view.current_line < len - 1 {
            view.set_current_line(stdout, view.current_line + 1)?;
        }

        Ok(())
    }

    fn handle_backspace<T: Repl>(&mut self, stdout: &mut Stdout, view: &mut SubmissionView, repl: &mut T) -> crossterm::Result<()> {
        let start = view.current_char;
        if start == 0 {
            if view.current_line == 0 {
                Ok(())
            } else {
                let current_line = view.document.get(view.current_line).unwrap().to_owned();
                let mut previous_line = view.document.get(view.current_line - 1).unwrap().to_owned();
                view.document.remove(view.current_line);
                view.set_current_line(stdout, view.current_line - 1)?;
                previous_line.push_str(&current_line);
                let len = previous_line.len();
                view.document[view.current_line] = previous_line;
                view.set_current_char(stdout, len)?;
                view.render(stdout, repl)
            }
        } else {
            let line_index = view.current_line;
            let line = view.document.get(line_index).unwrap();
            let before = line.get(0..start - 1).unwrap();
            let after = line.get(start..).unwrap();
            view.document[line_index] = format!("{}{}", before, after);
            view.set_current_char(stdout, view.current_char - 1)?;
            view.render(stdout, repl)
        }
    }

    fn handle_delete<T: Repl>(&mut self, stdout: &mut Stdout, view: &mut SubmissionView, repl: &mut T) -> crossterm::Result<()> {
        let line_index = view.current_line;
        let line = view.document.get(line_index).unwrap();
        let start = view.current_char;
        if start >= line.len() {
            if view.document.len() > 0 && view.current_line == view.document.len() - 1 {
                Ok(())
            } else {
                let next_line = view.document.get(view.current_line + 1).unwrap().to_owned();
                view.document[view.current_line].push_str(&next_line);
                view.document.remove(view.current_line + 1);
                view.render(stdout, repl)
            }
        } else {
            let before = line.get(0..start).unwrap();
            let after = line.get(start + 1..).unwrap();
            view.document[line_index] = format!("{}{}", before, after);
            Ok(())
        }
    }

    fn handle_home(&mut self, stdout: &mut Stdout, view: &mut SubmissionView) -> crossterm::Result<()> {
        view.set_current_char(stdout, 0)
    }

    fn handle_end(&mut self, stdout: &mut Stdout, view: &mut SubmissionView) -> crossterm::Result<()> {
        view.set_current_char(stdout, view.document.get(view.current_line).map_or(0, |line| line.len()))
    }

    fn handle_tab(&mut self, stdout: &mut Stdout, view: &mut SubmissionView) -> crossterm::Result<()> {
        let tab_width = 4;
        let start = view.current_char;
        let remaining_spaces = tab_width - start % tab_width;
        let mut line = view.document.get(view.current_line).unwrap().to_owned();
        line.insert_str(start, &" ".repeat(remaining_spaces));
        view.document[view.current_line] = line;
        view.set_current_char(stdout, view.current_char + remaining_spaces)
    }

    fn handle_page_up<T: Repl>(&mut self, stdout: &mut Stdout, view: &mut SubmissionView, repl: &mut T) -> crossterm::Result<()> {
        if self.submission_history_index == 0 {
            self.submission_history_index = max(1, self.submission_history.len()) - 1;
        } else {
            self.submission_history_index -= 1;
        }

        self.update_document_from_history(stdout, view, repl)
    }

    fn handle_page_down<T: Repl>(&mut self, stdout: &mut Stdout, view: &mut SubmissionView, repl: &mut T) -> crossterm::Result<()> {
        if self.submission_history_index == self.submission_history.len() - 1 {
            self.submission_history_index = 0;
        } else {
            self.submission_history_index += 1;
        }

        self.update_document_from_history(stdout, view, repl)
    }

    fn handle_typing<T: Repl>(&mut self, stdout: &mut Stdout, view: &mut SubmissionView, text: char, repl: &mut T) -> crossterm::Result<()> {
        let line_index = view.current_line;
        let start = view.current_char;
        let mut line = view.document.get(line_index).unwrap().to_owned();
        line.insert(start, text);
        view.document[line_index] = line;
        view.set_current_char(stdout, view.current_char + 1)?;
        view.render(stdout, repl)
    }

    fn insert_line<T: Repl>(&mut self, stdout: &mut Stdout, view: &mut SubmissionView, repl: &mut T) -> crossterm::Result<()> {
        let remainder = view.document.get(view.current_line).unwrap().get(view.current_char..).unwrap().to_owned();
        view.document[view.current_line] = view.document.get(view.current_line).unwrap().get(0..view.current_char).unwrap().to_owned();
        let line_index = view.current_line + 1;
        view.document.insert(line_index, remainder);
        view.set_current_char(stdout, 0)?;
        view.set_current_line(stdout, line_index)?;
        view.render(stdout, repl)?;
        Ok(())
    }

    fn update_document_from_history<T: Repl>(&mut self, stdout: &mut Stdout, view: &mut SubmissionView, repl: &mut T) -> crossterm::Result<()> {
        if self.submission_history.len() == 0 {
            Ok(())
        } else {
            view.document.clear();
            let history_item = self.submission_history.get(self.submission_history_index).unwrap();
            let lines = history_item.split('\n');
            for line in lines {
                view.document.push(line.to_owned());
            }

            view.set_current_line(stdout, view.document.len() - 1)?;
            view.set_current_char(stdout, view.document.get(view.current_line).map_or(0, |line| line.len()))?;
            view.render(stdout, repl)
        }
    }
}

fn print<T: Display>(stdout: &mut Stdout, text: T) -> crossterm::Result<()> {
    write!(stdout, "{}", text)?;
    stdout.flush()?;
    Ok(())
}