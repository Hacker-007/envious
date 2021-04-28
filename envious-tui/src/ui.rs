use tui::{Frame, backend::Backend, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style}, text::{Span, Spans, Text}, widgets::{Block, Borders, List, ListItem, Paragraph}};
use unicode_width::UnicodeWidthStr;

use crate::app::{App, FocusedBlock};

pub fn render_help_message<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let (msg, style) = match app.focused_block {
        FocusedBlock::Output => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        FocusedBlock::CodeEditor => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing"),
            ],
            Style::default(),
        ),
    };

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, area);
}

pub fn render_editor_generated_output<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(34),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(area);

    let input = Paragraph::new(app.code.as_ref())
        .style(match app.focused_block {
            FocusedBlock::Output => Style::default(),
            FocusedBlock::CodeEditor => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Code Editor"));
    f.render_widget(input, chunks[0]);

    if let FocusedBlock::CodeEditor = app.focused_block {
        f.set_cursor(
            chunks[0].x + app.line_width + 1,
            chunks[0].y + app.current_line,
        )
    }

    let generated_code = Paragraph::new(app.generated_code.as_ref()).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Generated Code"),
    );

    f.render_widget(generated_code, chunks[1]);

    let output: Vec<ListItem> = app
        .output
        .iter()
        .map(|item| ListItem::new(item.as_str()))
        .collect();

    let output = List::new(output).block(Block::default().borders(Borders::ALL).title("Output"));

    f.render_widget(output, chunks[2]);
}

pub fn get_current_line_width(app: &App) -> u16 {
    get_lines(&app.code)
        .nth((app.current_line - 1) as usize)
        .unwrap()
        .width() as u16
}

pub fn get_lines(code: &str) -> impl Iterator<Item = &str> {
    code.split('\n').map(|line| {
        let l = line.len();
        if l > 0 && line.as_bytes()[l - 1] == b'\r' {
            &line[0..l - 1]
        } else {
            line
        }
    })
}
