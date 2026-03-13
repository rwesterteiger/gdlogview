use crate::model::{Filters, TableModel};
use crate::scroll::ScrollState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Table,
    LoggerFilter,
    MessageFilter,
}

pub struct App {
    pub model: TableModel,
    pub scroll: ScrollState,
    pub filters: Filters,
    pub focus: Focus,
    pub filename: String,
}

impl App {
    pub fn new(model: TableModel, filename: String) -> Self {
        App {
            model,
            scroll: ScrollState::new(),
            filters: Filters::new(),
            focus: Focus::Table,
            filename,
        }
    }

    pub fn apply_filters(&mut self) {
        self.model.apply_filters(&self.filters);
        let len = self.model.filtered_indices.len();
        if len == 0 {
            self.scroll.selected = 0;
        } else if self.scroll.selected >= len {
            self.scroll.selected = len - 1;
        }
        self.clamp_scroll();
    }

    pub fn toggle_expand(&mut self) {
        self.model.toggle_expand(self.scroll.selected);
    }

    pub fn next_row(&mut self) {
        self.scroll.next(self.model.filtered_indices.len());
        self.clamp_scroll();
    }

    pub fn prev_row(&mut self) {
        self.scroll.prev();
        self.clamp_scroll();
    }

    pub fn first_row(&mut self) {
        self.scroll.first();
    }

    pub fn last_row(&mut self) {
        self.scroll.last(self.model.filtered_indices.len());
        self.clamp_scroll();
    }

    pub fn page_down(&mut self) {
        self.scroll.page_down(self.model.filtered_indices.len());
        self.clamp_scroll();
    }

    pub fn page_up(&mut self) {
        self.scroll.page_up();
        self.clamp_scroll();
    }

    fn clamp_scroll(&mut self) {
        let total = self.model.filtered_indices.len();
        let model = &self.model;
        self.scroll.clamp(total, |row| model.row_height(row));
    }
}
