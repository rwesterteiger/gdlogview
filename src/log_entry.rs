use chrono::{Duration, NaiveTime};
use serde::{Deserialize, Deserializer};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }

    pub fn all() -> &'static [LogLevel] {
        &[LogLevel::Trace, LogLevel::Debug, LogLevel::Info, LogLevel::Warn, LogLevel::Error]
    }
}

impl<'de> Deserialize<'de> for LogLevel {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        Ok(match s.to_uppercase().as_str() {
            "TRACE" => LogLevel::Trace,
            "DEBUG" => LogLevel::Debug,
            "INFO" => LogLevel::Info,
            "WARN" | "WARNING" => LogLevel::Warn,
            "ERROR" => LogLevel::Error,
            _ => LogLevel::Info,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogEntry {
    pub time: String,
    pub level: LogLevel,
    pub logger: String,
    pub text: String,
    /// Delta from the first entry's timestamp. `None` if the timestamp was unparseable.
    #[serde(skip)]
    pub delta: Option<Duration>,
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

    /// Format delta with a leading `+`/`-` sign for display in the table column.
    pub fn format_delta(&self) -> String {
        let d = match self.delta {
            None => return String::new(),
            Some(d) => d,
        };
        let ms = d.num_milliseconds();
        let sign = if ms < 0 { '-' } else { '+' };
        let abs = ms.unsigned_abs();
        format!("{}{}.{:03}", sign, abs / 1000, abs % 1000)
    }
}

impl AsRef<LogEntry> for LogEntry {
    fn as_ref(&self) -> &LogEntry {
      self
    }
}
fn parse_time(s: &str) -> Option<NaiveTime> {
    NaiveTime::parse_from_str(s, "%H:%M:%S%.3f").ok()
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

    // Compute deltas relative to the first parseable timestamp.
    let origin = entries.iter().find_map(|e| parse_time(&e.time));
    if let Some(origin) = origin {
        for entry in &mut entries {
            if let Some(t) = parse_time(&entry.time) {
                entry.delta = Some(t - origin);
            }
        }
    }

    Ok(entries)
}
