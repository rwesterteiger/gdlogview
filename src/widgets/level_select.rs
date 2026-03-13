use crate::log_entry::LogLevel;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

/// A dropdown-style selector for the minimum log level filter.
pub struct LevelSelect {
    pub level: LogLevel,
    pub focused: bool,
}

impl Widget for LevelSelect {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_style = if self.focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(" Min Level (v) ");

        let levels = LogLevel::all();
        let spans: Vec<Span> = levels
            .iter()
            .map(|&l| {
                let label = l.as_str();
                if l == self.level {
                    Span::styled(
                        format!("[{}]", label),
                        Style::default().fg(level_color(l)).add_modifier(ratatui::style::Modifier::BOLD),
                    )
                } else {
                    Span::styled(
                        format!(" {} ", label),
                        Style::default().fg(Color::DarkGray),
                    )
                }
            })
            .collect();

        Paragraph::new(Line::from(spans)).block(block).render(area, buf);
    }
}

fn level_color(level: LogLevel) -> Color {
    match level {
        LogLevel::Error => Color::Red,
        LogLevel::Warn => Color::Yellow,
        LogLevel::Info => Color::Green,
        LogLevel::Debug => Color::Cyan,
        LogLevel::Trace => Color::DarkGray,
    }
}
