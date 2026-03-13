use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

/// A single-line text input with a labelled border that highlights when focused.
pub struct FilterInput<'a> {
    pub text: &'a str,
    pub title: &'a str,
    pub focused: bool,
}

impl<'a> FilterInput<'a> {
    /// Cursor x position within the parent area (for `Frame::set_cursor_position`).
    pub fn cursor_x(&self, area: Rect) -> u16 {
        area.x + 1 + self.text.len() as u16
    }

    /// Cursor y position within the parent area (for `Frame::set_cursor_position`).
    pub fn cursor_y(&self, area: Rect) -> u16 {
        area.y + 1
    }
}

impl<'a> Widget for FilterInput<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_style = if self.focused {
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(self.title);

        Paragraph::new(self.text).block(block).render(area, buf);
    }
}
