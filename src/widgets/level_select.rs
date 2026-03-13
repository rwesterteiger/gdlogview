use crate::log_entry::LogLevel;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct LevelSelect {
    pub level: LogLevel,
}

impl Widget for LevelSelect {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(" Min Level (v) ");

        let levels = LogLevel::all();
        let spans: Vec<Span> = levels
            .iter()
            .map(|&l| {
                let label = l.as_str();
                if l == self.level {
                    Span::styled(
                        format!("[{}]", label),
                        Style::default().fg(level_color(l)).add_modifier(Modifier::BOLD),
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
