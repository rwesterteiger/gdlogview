use crate::log_entry::LogEntry;
use std::collections::HashSet;

pub struct Filters {
    pub logger: String,
    pub message: String,
}

impl Filters {
    pub fn new() -> Self {
        Self { logger: String::new(), message: String::new() }
    }
}

pub struct TableModel {
    pub all_entries: Vec<LogEntry>,
    /// Indices into `all_entries` that pass the current filters.
    pub filtered_indices: Vec<usize>,
    /// Indices (into `all_entries`) of entries currently expanded.
    pub expanded: HashSet<usize>,
}

impl TableModel {
    pub fn new(entries: Vec<LogEntry>) -> Self {
        let filtered_indices = (0..entries.len()).collect();
        Self { all_entries: entries, filtered_indices, expanded: HashSet::new() }
    }

    pub fn apply_filters(&mut self, filters: &Filters) {
        let logger_lower = filters.logger.to_lowercase();
        let message_lower = filters.message.to_lowercase();

        self.filtered_indices = self
            .all_entries
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                let logger_ok = logger_lower.is_empty()
                    || e.logger.to_lowercase().contains(&logger_lower);
                let message_ok = message_lower.is_empty()
                    || e.text.to_lowercase().contains(&message_lower);
                logger_ok && message_ok
            })
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
