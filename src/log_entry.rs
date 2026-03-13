use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct LogEntry {
    pub time: String,
    pub level: String,
    pub logger: String,
    pub text: String,
}

impl LogEntry {
    /// Returns true if the text field contains embedded newlines.
    pub fn is_multiline(&self) -> bool {
        self.text.contains('\n')
    }

    /// Count of display lines when expanded (the text split on `\n`).
    pub fn line_count(&self) -> usize {
        self.text.lines().count().max(1)
    }

    /// First line of text (for the collapsed / summary view).
    pub fn first_line(&self) -> &str {
        self.text.lines().next().unwrap_or(&self.text)
    }

    /// All lines of text as a Vec (for expanded view).
    pub fn lines(&self) -> Vec<&str> {
        self.text.lines().collect()
    }
}

pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<LogEntry>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        match serde_json::from_str::<LogEntry>(trimmed) {
            Ok(mut entry) => {
                let t = entry.text.trim_matches('\n');
                if t.len() != entry.text.len() {
                    entry.text = t.to_string();
                }
                entries.push(entry);
            }
            Err(e) => {
                eprintln!("Warning: skipping line {}: {e}", i + 1);
            }
        }
    }

    Ok(entries)
}
