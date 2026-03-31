use ratatui::widgets::Table;

use crate::log_entry::{LogEntry, LogLevel};
use std::collections::HashSet;

pub struct FilteredVec<T> {
    entries: Vec<T>,
    visible_indices: Vec<usize>,
}

impl<T> FilteredVec<T> {
    pub fn new(entries: Vec<T>) -> Self {
        let visible_indices = (0..entries.len()).collect();
        Self { entries, visible_indices }
    }

    /// Number of currently visible items.
    pub fn len(&self) -> usize {
        self.visible_indices.len()
    }

    /// Total number of items (unfiltered).
    pub fn total_len(&self) -> usize {
        self.entries.len()
    }

    /// All entries, unfiltered.
    pub fn entries(&self) -> &[T] {
        &self.entries
    }

    /// The indices of visible entries.
    pub fn visible_indices(&self) -> &[usize] {
        &self.visible_indices
    }

    /// Recompute visible indices by keeping only entries that satisfy `predicate`.
    pub fn update_filter(&mut self, predicate: impl Fn(&T) -> bool) {
        self.visible_indices = self
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| predicate(e))
            .map(|(i, _)| i)
            .collect();
    }
}

pub struct Filters {
    pub logger: String,
    pub message: String,
    pub min_level: LogLevel,
}
pub struct FilterPredicate {
  logger_lower : String,
  message_lower : String,
  min_level : LogLevel
}

impl FilterPredicate {
    pub fn is_visible(&self, e : impl AsRef<LogEntry>) -> bool {
      let e = e.as_ref();

      let logger_ok = self.logger_lower.is_empty() || e.logger.to_lowercase().contains(&self.logger_lower);
      let message_ok = self.message_lower.is_empty()  || e.text.to_lowercase().contains(&self.message_lower);  
      let level_ok = e.level >= self.min_level;
        
      logger_ok && message_ok && level_ok
    }
}
impl Filters {
    pub fn new() -> Self {
        Self { logger: String::new(), message: String::new(), min_level: LogLevel::Trace }
    }

    pub fn predicate(&self) -> FilterPredicate {
      FilterPredicate {
        logger_lower : self.logger.to_lowercase(),
        message_lower : self.message.to_lowercase(),
        min_level: self.min_level,
      }
    }
}

pub struct TableLogEntry {
  log_entry : LogEntry,
  expanded : bool
}

impl AsRef<LogEntry> for TableLogEntry {
    fn as_ref(&self) -> &LogEntry {
      &self.log_entry
    }
}
impl From<LogEntry> for TableLogEntry {
    fn from(log_entry: LogEntry) -> Self {
      TableLogEntry { log_entry, expanded : false }
    }
}

pub struct TableModel {
    pub entries: FilteredVec<TableLogEntry>,

    pub all_entries: Vec<LogEntry>,
    /// Indices into `all_entries` that pass the current filters.
    pub filtered_indices: Vec<usize>,
    /// Indices (into `all_entries`) of entries currently expanded.
    pub expanded: HashSet<usize>,
}

impl TableModel {
    pub fn new(all_entries: Vec<LogEntry>) -> Self {
        let filtered_indices = (0..all_entries.len()).collect();

        let entries = FilteredVec::new(
          all_entries
            .iter()
            .cloned()
            .map(TableLogEntry::from)
            .collect()
        );

        Self { entries, all_entries, filtered_indices, expanded: HashSet::new() }
    }

    pub fn apply_filters(&mut self, filters: &Filters) {
        let predicate = filters.predicate();

        self.filtered_indices = self
            .all_entries
            .iter()
            .enumerate()
            .filter(|(_, e)| { predicate.is_visible(e) })
            .map(|(i, _)| i)
            .collect();
    }

    pub fn toggle_expand(&mut self, filtered_row: usize) {
        if let Some(&idx) = self.filtered_indices.get(filtered_row) {
            let entry = &self.all_entries[idx];
            if entry.is_multiline() {
                if self.expanded.contains(&idx) {
                    self.expanded.remove(&idx);
                } else {
                    self.expanded.insert(idx);
                }
            }
        }
    }

    /// Visual lines occupied by a logical row (1 unless expanded multiline).
    pub fn row_height(&self, filtered_row: usize) -> usize {
        self.filtered_indices.get(filtered_row).map_or(1, |&idx| {
            let entry = &self.all_entries[idx];
            if self.expanded.contains(&idx) && entry.is_multiline() {
                entry.line_count()
            } else {
                1
            }
        })
    }
}
