/// Viewport/selection state for a scrollable list with variable-height rows.
pub struct ScrollState {
    /// Currently selected logical row.
    pub selected: usize,
    /// Index of the first visible logical row.
    pub scroll_offset: usize,
    /// Number of visual lines available in the viewport (written back from render).
    pub visible_rows: usize,
}

impl ScrollState {
    pub fn new() -> Self {
        Self { selected: 0, scroll_offset: 0, visible_rows: 20 }
    }

    pub fn next(&mut self, total: usize) {
        if total > 0 && self.selected < total - 1 {
            self.selected += 1;
        }
    }

    pub fn prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn first(&mut self) {
        self.selected = 0;
        self.scroll_offset = 0;
    }

    pub fn last(&mut self, total: usize) {
        if total > 0 {
            self.selected = total - 1;
        }
    }

    pub fn page_down(&mut self, total: usize) {
        let jump = self.visible_rows.saturating_sub(2).max(1);
        self.selected = (self.selected + jump).min(total.saturating_sub(1));
    }

    pub fn page_up(&mut self) {
        let jump = self.visible_rows.saturating_sub(2).max(1);
        self.selected = self.selected.saturating_sub(jump);
    }

    /// Adjust `scroll_offset` so that `selected` is within the visible window.
    /// `row_height(i)` returns the number of visual lines row `i` occupies.
    pub fn clamp(&mut self, total: usize, row_height: impl Fn(usize) -> usize) {
        if self.selected < self.scroll_offset {
            self.scroll_offset = self.selected;
        }

        let mut visual_used = 0;
        let mut last_visible = self.scroll_offset;
        for row in self.scroll_offset..total {
            let h = row_height(row);
            if visual_used + h > self.visible_rows && row > self.scroll_offset {
                break;
            }
            visual_used += h;
            last_visible = row;
        }

        if self.selected > last_visible {
            let mut vis = 0;
            let mut new_offset = self.selected;
            for row in (0..=self.selected).rev() {
                let h = row_height(row);
                if vis + h > self.visible_rows {
                    break;
                }
                vis += h;
                new_offset = row;
            }
            self.scroll_offset = new_offset;
        }
    }
}
