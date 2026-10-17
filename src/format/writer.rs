use super::model::Writer;
impl Writer<'_> {
    pub(super) fn push(&mut self, text: &str) {
        self.text.push_str(text);
        for ch in text.chars() {
            match ch {
                '\n' => {
                    self.column = 0;
                    self.trailing_breaks += 1;
                }
                '\r' => {}
                '\t' => {
                    self.column +=
                        self.options.indent_width - self.column % self.options.indent_width;
                    self.trailing_breaks = 0;
                }
                _ => {
                    self.column += 1;
                    self.trailing_breaks = 0;
                }
            }
        }
    }
    pub(super) fn breaks(&mut self, count: usize) {
        for _ in self.trailing_breaks..count {
            self.text.push_str(self.newline);
        }
        self.trailing_breaks = self.trailing_breaks.max(count);
        self.column = 0;
    }
    pub(super) fn line(&mut self) {
        if self.column > 0 {
            self.breaks(1);
        }
    }
    pub(super) fn indent(&mut self, level: usize) {
        if self.options.insert_spaces {
            self.push(&" ".repeat(level * self.options.indent_width));
        } else {
            self.push(&"\t".repeat(level));
        }
    }
    pub(super) fn finish(mut self) -> String {
        while self.text.ends_with(['\n', '\r']) {
            self.text.pop();
        }
        if !self.text.is_empty() {
            self.text.push_str(self.newline);
        }
        self.text
    }
}
