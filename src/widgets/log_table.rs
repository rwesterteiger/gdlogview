use crate::log_entry::{LogEntry, LogLevel};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Row, StatefulWidget, Table, Widget},
};
use std::collections::HashSet;

fn level_color(level: LogLevel) -> Color {
    match level {
        LogLevel::Error => Color::Red,
        LogLevel::Warn => Color::Yellow,
        LogLevel::Info => Color::Green,
        LogLevel::Debug => Color::Cyan,
        LogLevel::Trace => Color::DarkGray,
    }
}

fn multiline_marker(entry: &LogEntry, expanded: bool) -> &'static str {
    if !entry.is_multiline() {
        "  "
    } else if expanded {
        "▼ "
    } else {
        "▶ "
    }
}

fn build_expanded_row<'a>(
    entry: &LogEntry,
    lines: &[&str],
    marker: &str,
    level_col: Color,
    base_style: Style,
    _truncated: bool,
) -> Row<'a> {
    let time_text: String = {
        let mut s = entry.time.clone();
        for _ in 1..lines.len() {
            s.push_str("\n ");
        }
        s
    };

    let delta_text: String = {
        let mut s = entry.delta.clone();
        for _ in 1..lines.len() {
            s.push_str("\n ");
        }
        s
    };

    let level_text: String = {
        let mut s = format!("{:<5}", entry.level.as_str());
        for _ in 1..lines.len() {
            s.push_str("\n     ");
        }
        s
    };

    let logger_text: String = {
        let mut s = entry.logger.clone();
        for _ in 1..lines.len() {
            s.push_str("\n ");
        }
        s
    };

    let msg_text: String = {
        let mut parts = Vec::with_capacity(lines.len());
        for (i, line) in lines.iter().enumerate() {
            if i == 0 {
                parts.push(format!("{}{}", marker, line));
            } else {
                parts.push(format!("  {}", line));
            }
        }
        parts.join("\n")
    };

    Row::new(vec![
        Cell::from(time_text),
        Cell::from(Span::styled(delta_text, Style::default().fg(Color::DarkGray))),
        Cell::from(Span::styled(level_text, Style::default().fg(level_col))),
        Cell::from(logger_text),
        Cell::from(msg_text),
    ])
    .style(base_style)
}

/// Widget for the log table. State is `visible_rows` (written back during render
/// so navigation logic in App can use it on the next frame).
pub struct LogTable<'a> {
    pub all_entries: &'a [LogEntry],
    pub filtered_indices: &'a [usize],
    pub expanded: &'a HashSet<usize>,
    pub selected: usize,
    pub scroll_offset: usize,
}

impl<'a> StatefulWidget for LogTable<'a> {
    /// The only value mutated by rendering: the number of usable table rows.
    type State = usize;

    fn render(self, area: Rect, buf: &mut Buffer, visible_rows: &mut usize) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(
                " Logs [{}/{}] ",
                if self.filtered_indices.is_empty() { 0 } else { self.selected + 1 },
                self.filtered_indices.len()
            ));

        let inner = block.inner(area);
        let table_height = inner.height.saturating_sub(1) as usize;
        *visible_rows = table_height;

        let mut rows: Vec<Row> = Vec::new();
        let mut visual_lines_used = 0;
        let mut row_idx = self.scroll_offset;

        while row_idx < self.filtered_indices.len() && visual_lines_used < table_height {
            let entry_idx = self.filtered_indices[row_idx];
            let entry = &self.all_entries[entry_idx];
            let expanded = self.expanded.contains(&entry_idx);
            let is_selected = row_idx == self.selected;

            let level_col = level_color(entry.level);
            let marker = multiline_marker(entry, expanded);

            let base_style = if is_selected {
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            if expanded && entry.is_multiline() {
                let lines = entry.lines();
                let height = lines.len();

                if visual_lines_used + height > table_height {
                    let can_show = table_height - visual_lines_used;
                    let row = build_expanded_row(entry, &lines[..can_show], marker, level_col, base_style, true);
                    rows.push(row.height(can_show as u16));
                    visual_lines_used += can_show;
                } else {
                    let row = build_expanded_row(entry, &lines, marker, level_col, base_style, false);
                    rows.push(row.height(height as u16));
                    visual_lines_used += height;
                }
            } else {
                let text_display = if entry.is_multiline() {
                    format!("{}{} ...", marker, entry.first_line())
                } else {
                    format!("{}{}", marker, entry.first_line())
                };

                rows.push(
                    Row::new(vec![
                        Cell::from(entry.time.clone()),
                        Cell::from(Span::styled(
                            entry.delta.clone(),
                            Style::default().fg(Color::DarkGray),
                        )),
                        Cell::from(Span::styled(
                            format!("{:<5}", entry.level.as_str()),
                            Style::default().fg(level_col),
                        )),
                        Cell::from(entry.logger.clone()),
                        Cell::from(text_display),
                    ])
                    .style(base_style)
                    .height(1),
                );
                visual_lines_used += 1;
            }

            row_idx += 1;
        }

        let header_style = Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD);
        let header = Row::new(vec![
            Cell::from(Span::styled("Time", header_style)),
            Cell::from(Span::styled("Delta", header_style)),
            Cell::from(Span::styled("Level", header_style)),
            Cell::from(Span::styled("Logger", header_style)),
            Cell::from(Span::styled("Message", header_style)),
        ])
        .height(1)
        .style(Style::default().bg(Color::Rgb(40, 40, 60)));

        let widths = [
            Constraint::Length(13),
            Constraint::Length(8),
            Constraint::Length(7),
            Constraint::Length(14),
            Constraint::Min(20),
        ];

        Widget::render(
            Table::new(rows, widths)
                .header(header)
                .block(block)
                .column_spacing(1),
            area,
            buf,
        );
    }
}
