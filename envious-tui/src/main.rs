use std::{cmp::{max, min}, error::Error, io};

use app::App;
use envyc::{
    compile,
    environment::Environment,
    error::reporter::{ErrorReporter, ReporterResult},
    filter_tokens,
    function_table::FunctionTable,
    interner::Interner,
    lex, parse, type_check,
};
use event::{Event, Events};
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};
use ui::{get_current_line_width, render_editor_generated_output, render_help_message};

use crate::app::FocusedBlock;

pub mod app;
pub mod event;
pub mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut events = Events::new();
    let mut app = App::new();
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
                .split(f.size());

            render_help_message(f, &mut app, chunks[0]);
            render_editor_generated_output(f, &mut app, chunks[1]);
        })?;

        if let Event::Input(key) = events.next()? {
            match app.focused_block {
                FocusedBlock::Output => match key {
                    Key::Char('e') => {
                        app.focused_block = FocusedBlock::CodeEditor;
                        events.disable_exit_key();
                        app.generated_code.drain(..);
                    }
                    Key::Esc => {
                        break;
                    }
                    _ => {}
                },
                FocusedBlock::CodeEditor => match key {
                    Key::Char('\t') => {
                        app.add_tab();
                    }
                    Key::Char(ch) => {
                        app.add_char(ch);
                    }
                    Key::Backspace => {
                        let popped = app.code.pop();
                        if let Some('\n') = popped {
                            if app.current_line > 1 {
                                if app.current_line == app.line_count {
                                    app.line_count -= 1;
                                }

                                app.current_line -= 1;
                                let current_line_width = get_current_line_width(&app);
                                app.line_width = max(app.line_width, current_line_width);
                                if let Some('\r') = app.code.chars().nth(app.line_width as usize) {
                                    app.code.pop();
                                    app.line_width -= 1;
                                }
                            }
                        } else if app.line_width != 0 {
                            app.line_width -= 1;
                        }
                    }
                    Key::Left => {
                        if app.line_width != 0 {
                            app.line_width -= 1;
                        }
                    }
                    Key::Right => {
                        if app.line_width != get_current_line_width(&app) {
                            app.line_width += 1;
                        }
                    }
                    Key::Up => {
                        if app.current_line > 1 {
                            app.current_line -= 1;
                            let current_line_width = get_current_line_width(&app);
                            app.line_width = min(app.line_width, current_line_width);
                        }
                    }
                    Key::Down => {
                        if app.current_line < app.line_count {
                            app.current_line += 1;
                            let current_line_width = get_current_line_width(&app);
                            app.line_width = min(app.line_width, current_line_width);
                        }
                    }
                    Key::Esc => {
                        app.focused_block = FocusedBlock::Output;
                        events.enable_exit_key();
                        match compile_code(&app.code) {
                            Ok(generated_code) => {
                                app.generated_code = generated_code;
                                app.output = vec![];
                            }
                            Err(errors) => {
                                app.output = errors;
                                continue;
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
    }

    Ok(())
}

fn compile_code(code: &str) -> Result<String, Vec<String>> {
    let mut error_reporter = ErrorReporter::new(vec![]);
    let mut interner = Interner::default();
    error_reporter.add("editor", code);
    let tokens = lex("editor", code.as_bytes(), &mut interner).report_result(&error_reporter, true)?;
    let filtered_tokens = filter_tokens(tokens);
    let program = parse(filtered_tokens).report_result(&error_reporter, true)?;
    let mut type_env = Environment::default();
    let mut function_table = FunctionTable::default();
    let typed_program =
        type_check(program, &mut type_env, &mut function_table).report_result(&error_reporter, true)?;
    compile(&typed_program, "editor", &mut interner, None).report_result(&error_reporter, true)
}
