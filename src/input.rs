use crate::app::{App, Focus};
use crossterm::event::{KeyCode, KeyEvent};

/// Returns `true` if the app should quit.
pub fn handle_key(app: &mut App, key: KeyEvent) -> bool {
    match app.focus {
        Focus::Table => handle_table(app, key),
        Focus::LoggerFilter => handle_filter_logger(app, key),
        Focus::MessageFilter => handle_filter_message(app, key),
        Focus::SavePrompt => handle_save_prompt(app, key),
    }
}

fn handle_table(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('q') => return true,
        KeyCode::Down | KeyCode::Char('j') => app.next_row(),
        KeyCode::Up | KeyCode::Char('k') => app.prev_row(),
        KeyCode::Home | KeyCode::Char('g') => app.first_row(),
        KeyCode::End | KeyCode::Char('G') => app.last_row(),
        KeyCode::PageDown => app.page_down(),
        KeyCode::PageUp => app.page_up(),
        KeyCode::Char(' ') => app.toggle_expand(),
        KeyCode::Char('/') => app.focus = Focus::MessageFilter,
        KeyCode::Char('l') | KeyCode::Tab => app.focus = Focus::LoggerFilter,
        KeyCode::Char('v') => app.cycle_level(),
        KeyCode::Char('s') => {
            app.save_prompt.clear();
            app.save_status = None;
            app.focus = Focus::SavePrompt;
        }
        _ => {}
    }
    false
}

fn handle_filter_logger(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Esc | KeyCode::Enter => app.focus = Focus::Table,
        KeyCode::Tab => app.focus = Focus::MessageFilter,
        KeyCode::Backspace => { app.filters.logger.pop(); app.apply_filters(); }
        KeyCode::Char(c) => { app.filters.logger.push(c); app.apply_filters(); }
        _ => {}
    }
    false
}

fn handle_filter_message(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Esc | KeyCode::Enter => app.focus = Focus::Table,
        KeyCode::Tab => app.focus = Focus::LoggerFilter,
        KeyCode::Backspace => { app.filters.message.pop(); app.apply_filters(); }
        KeyCode::Char(c) => { app.filters.message.push(c); app.apply_filters(); }
        _ => {}
    }
    false
}

fn handle_save_prompt(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Esc => {
            app.save_status = None;
            app.focus = Focus::Table;
        }
        KeyCode::Enter => {
            let path = app.save_prompt.clone();
            app.save_visible(&path);
            app.focus = Focus::Table;
        }
        KeyCode::Backspace => { app.save_prompt.pop(); }
        KeyCode::Char(c) => { app.save_prompt.push(c); }
        _ => {}
    }
    false
}
