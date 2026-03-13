mod log_entry;
mod app;
mod input;
mod model;
mod scroll;
mod ui;
mod widgets;

use std::io;
use std::path::{Path, PathBuf};
use app::App;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

#[derive(Parser, Debug)]
#[command(name = "logview", about = "TUI viewer for NDJSON log files")]
struct Cli {
    /// Path to the NDJSON log file
    file: String,
}

fn most_recent_file(dir: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut best: Option<(PathBuf, std::time::SystemTime)> = None;
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let meta = entry.metadata()?;
        if !meta.is_file() {
            continue;
        }
        let modified = meta.modified()?;
        if best.as_ref().map_or(true, |(_, t)| modified > *t) {
            best = Some((entry.path(), modified));
        }
    }
    best.map(|(p, _)| p).ok_or_else(|| "directory is empty".into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let input_path = Path::new(&cli.file);
    let resolved = if input_path.is_dir() {
        most_recent_file(input_path)?
    } else {
        input_path.to_path_buf()
    };
    let filename = resolved
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| cli.file.clone());
    let entries = log_entry::load_from_file(&resolved)?;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(model::TableModel::new(entries), filename);
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {err}");
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                return Ok(());
            }
            if input::handle_key(app, key) {
                return Ok(());
            }
        }
    }
}
