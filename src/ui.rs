use crate::app::{App, Focus};
use crate::widgets::filter_input::FilterInput;
use crate::widgets::level_select::LevelSelect;
use crate::widgets::log_table::LogTable;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let size = f.area();

    // Layout: title (1 line) | filter bar (3 lines) | table | help bar (1 line)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // title
            Constraint::Length(3), // filters
            Constraint::Min(5),    // table
            Constraint::Length(1), // help
        ])
        .split(size);

    draw_title(f, app, chunks[0]);
    draw_filters(f, app, chunks[1]);
    draw_table(f, app, chunks[2]);
    draw_help(f, app, chunks[3]);
}

fn draw_title(f: &mut Frame, app: &App, area: Rect) {
    let title = Paragraph::new(Line::from(vec![Span::styled(
        app.filename.as_str(),
        Style::default().fg(Color::White),
    )]))
    .alignment(Alignment::Center);
    f.render_widget(title, area);
}

fn draw_filters(f: &mut Frame, app: &App, area: Rect) {
    let filter_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(40),
            Constraint::Percentage(20),
        ])
        .split(area);

    let logger = FilterInput {
        text: &app.filters.logger,
        title: " Logger Filter (l) ",
        focused: app.focus == Focus::LoggerFilter,
    };
    if logger.focused {
        f.set_cursor_position((logger.cursor_x(filter_chunks[0]), logger.cursor_y(filter_chunks[0])));
    }
    f.render_widget(logger, filter_chunks[0]);

    let message = FilterInput {
        text: &app.filters.message,
        title: " Message Filter (/) ",
        focused: app.focus == Focus::MessageFilter,
    };
    if message.focused {
        f.set_cursor_position((message.cursor_x(filter_chunks[1]), message.cursor_y(filter_chunks[1])));
    }
    f.render_widget(message, filter_chunks[1]);

    let level = LevelSelect { level: app.filters.min_level };
    f.render_widget(level, filter_chunks[2]);
}

fn draw_table(f: &mut Frame, app: &mut App, area: Rect) {
    let widget = LogTable {
        all_entries: &app.model.all_entries,
        filtered_indices: &app.model.filtered_indices,
        expanded: &app.model.expanded,
        selected: app.scroll.selected,
        scroll_offset: app.scroll.scroll_offset,
    };
    f.render_stateful_widget(widget, area, &mut app.scroll.visible_rows);
}

fn draw_help(f: &mut Frame, app: &App, area: Rect) {
    match app.focus {
        Focus::SavePrompt => {
            let spans = vec![
                Span::styled(" Save to: ", Style::default().fg(Color::Yellow)),
                Span::styled(app.save_prompt.as_str(), Style::default().fg(Color::White)),
                Span::styled("_", Style::default().fg(Color::Yellow)),
                Span::styled("  Esc:Cancel", Style::default().fg(Color::DarkGray)),
            ];
            f.render_widget(Paragraph::new(Line::from(spans)), area);
            let cursor_x = area.x + " Save to: ".len() as u16 + app.save_prompt.len() as u16;
            f.set_cursor_position((cursor_x, area.y));
        }
        _ => {
            let help_text = if let Some(status) = &app.save_status {
                status.as_str()
            } else {
                match app.focus {
                    Focus::Table => " q:Quit  ↑↓/jk:Navigate  Space:Expand/Collapse  l:Logger filter  /:Message filter  v:Level filter  s:Save  g/G:Top/Bottom  PgUp/PgDn",
                    Focus::LoggerFilter | Focus::MessageFilter => " Enter/Esc:Back to table  Tab:Switch filter",
                    Focus::SavePrompt => unreachable!(),
                }
            };
            let color = if app.save_status.is_some() { Color::Green } else { Color::DarkGray };
            f.render_widget(
                Paragraph::new(Line::from(vec![Span::styled(help_text, Style::default().fg(color))])),
                area,
            );
        }
    }
}
