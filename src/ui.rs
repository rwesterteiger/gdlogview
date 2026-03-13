use crate::app::{App, Focus};
use crate::widgets::filter_input::FilterInput;
use crate::widgets::log_table::LogTable;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let size = f.area();

    // Layout: filter bar (3 lines) | table | help bar (1 line)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // filters
            Constraint::Min(5),   // table
            Constraint::Length(1), // help
        ])
        .split(size);

    draw_filters(f, app, chunks[0]);
    draw_table(f, app, chunks[1]);
    draw_help(f, app, chunks[2]);
}

fn draw_filters(f: &mut Frame, app: &App, area: Rect) {
    let filter_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
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
    let help_text = match app.focus {
        Focus::Table => {
            " q:Quit  ↑↓/jk:Navigate  Space:Expand/Collapse  l:Logger filter  /:Message filter  g/G:Top/Bottom  PgUp/PgDn"
        }
        Focus::LoggerFilter | Focus::MessageFilter => {
            " Enter:Apply & return  Esc:Back to table  Tab:Switch filter"
        }
    };

    let help = Paragraph::new(Line::from(vec![Span::styled(
        help_text,
        Style::default().fg(Color::DarkGray),
    )]));
    f.render_widget(help, area);
}
