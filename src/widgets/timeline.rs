use crate::log_entry::LogEntry;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct Timeline<'a> {
    pub all_entries: &'a [LogEntry],
    pub filtered_indices: &'a [usize],
    pub selected: usize,
}

fn parse_delta_ms(s: &str) -> Option<i64> {
    if s.is_empty() {
        return None;
    }
    let (sign, rest) = if s.starts_with('-') {
        (-1i64, &s[1..])
    } else {
        (1i64, s.strip_prefix('+').unwrap_or(s))
    };
    let mut parts = rest.splitn(2, '.');
    let secs: i64 = parts.next()?.parse().ok()?;
    let millis: i64 = parts.next().and_then(|m| m.parse().ok()).unwrap_or(0);
    Some(sign * (secs * 1000 + millis))
}

impl Widget for Timeline<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Timeline ");
        let inner = block.inner(area);
        block.render(area, buf);

        if inner.width < 10 || self.all_entries.is_empty() {
            return;
        }

        // Time range from all entries (first is always origin "+0.000")
        let left_label = self
            .all_entries
            .first()
            .map(|e| e.delta.as_str())
            .unwrap_or("+0.000");
        let right_label = self
            .all_entries
            .last()
            .map(|e| e.delta.as_str())
            .unwrap_or("+0.000");

        let max_ms = parse_delta_ms(right_label).unwrap_or(0);

        // Selected entry's delta
        let sel_entry = self
            .filtered_indices
            .get(self.selected)
            .and_then(|&i| self.all_entries.get(i));

        let sel_ms = sel_entry
            .and_then(|e| parse_delta_ms(&e.delta))
            .unwrap_or(0);

        let sel_delta = sel_entry.map(|e| e.delta.as_str()).unwrap_or("+0.000");
        let marker = format!("[{}]", sel_delta);
        let marker_len = marker.len();

        let left_len = left_label.len() as u16;
        let right_len = right_label.len() as u16;

        // axis occupies the space between the two labels (plus a space on each side)
        let used = left_len + right_len + 2;
        if inner.width <= used || (inner.width - used) as usize <= marker_len {
            return;
        }
        let axis_width = (inner.width - used) as usize;

        let fraction = if max_ms > 0 {
            (sel_ms as f64 / max_ms as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };

        // Center the marker label at the fractional position
        let center = (fraction * (axis_width.saturating_sub(1)) as f64).round() as usize;
        let left_edge = center
            .saturating_sub(marker_len / 2)
            .min(axis_width.saturating_sub(marker_len));
        let right_edge = left_edge + marker_len;

        let left_axis = "─".repeat(left_edge);
        let right_axis = "─".repeat(axis_width.saturating_sub(right_edge));

        let dim = Style::default().fg(Color::DarkGray);
        let marker_style = Style::default().fg(Color::Yellow);

        let line = Line::from(vec![
            Span::styled(left_label, dim),
            Span::styled(" ", dim),
            Span::styled(left_axis, dim),
            Span::styled(marker, marker_style),
            Span::styled(right_axis, dim),
            Span::styled(" ", dim),
            Span::styled(right_label, dim),
        ]);

        Paragraph::new(line).render(inner, buf);
    }
}
