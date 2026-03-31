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

/// Format milliseconds as "S.mmm" with no sign prefix.
fn fmt_ms(ms: i64) -> String {
    let abs = ms.unsigned_abs();
    format!("{}.{:03}", abs / 1000, abs % 1000)
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

        let max_ms = self
            .all_entries
            .last()
            .and_then(|e| e.delta)
            .map(|d| d.num_milliseconds())
            .unwrap_or(0);

        let sel_entry = self
            .filtered_indices
            .get(self.selected)
            .and_then(|&i| self.all_entries.get(i));

        let sel_ms = sel_entry
            .and_then(|e| e.delta)
            .map(|d| d.num_milliseconds())
            .unwrap_or(0);

        let left_label = "0.000";
        let right_label = fmt_ms(max_ms);

        let left_len = left_label.len() as u16;
        let right_len = right_label.len() as u16;

        let used = left_len + right_len + 2;
        if inner.width <= used {
            return;
        }
        let axis_width = (inner.width - used) as usize;

        let fraction = if max_ms > 0 {
            (sel_ms as f64 / max_ms as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let pct = (fraction * 100.0).round() as u32;
        let marker = format!("[{} ({}%)]", fmt_ms(sel_ms), pct);
        let marker_len = marker.len();
        if axis_width <= marker_len {
            return;
        }

        // Center the marker at the fractional position, clamped to axis bounds.
        let center = (fraction * axis_width.saturating_sub(1) as f64).round() as usize;
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
