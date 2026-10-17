//! In-memory text emission and column/newline bookkeeping for [`FormatWriter`].
//! Methods change owned text only; they never access streams or files.
//!
//! Contains AI-generated documentation additions.
use super::model::FormatWriter;
impl FormatWriter<'_> {
    /// Append exact text and update scalar/tab column and trailing-break counts. No I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: appends text and updates column/break counters.
    /// - `text` `[in]`: bytes copied unchanged, including literal newlines.
    ///
    /// # Returns
    /// `()`; output text grows by the supplied bytes.
    ///
    /// # Panics
    /// Indent width must be nonzero for tab expansion; the public renderer validates it.
    ///
    /// # Algorithm
    /// Append bytes unchanged, then count Unicode scalar columns. LF resets the column
    /// and increments trailing breaks; CR has no column effect; tabs advance to the next
    /// configured stop. Other characters advance one column and clear trailing breaks.
    ///
    /// # Flow
    /// ```text
    ///  [append text bytes] ---> [next character] <-------+
    ///                                |                 |
    ///                             <kind?>              |
    ///                       /       |       \          |
    ///                     LF       CR      tab/other   |
    ///                      |        |          |       |
    ///               [reset col] [keep] [advance column] |
    ///                      |        |          |       |
    ///                      +--------+----------+-------+
    ///                                | exhausted
    ///                               [()]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// out.push("set(A 1)");
    /// ```
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
    /// Ensure at least the requested tracked trailing LF breaks using the selected newline style. No I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: appends only missing breaks, raises trailing count and resets column to zero.
    /// - `count` `[in]`: minimum tracked trailing line-break count, not an additional count.
    ///
    /// # Returns
    /// `()`; existing text is never removed.
    ///
    /// # Examples
    /// ```text
    /// out.breaks(2);
    /// ```
    pub(super) fn breaks(&mut self, count: usize) {
        for _ in self.trailing_breaks..count {
            self.text.push_str(self.newline);
        }
        self.trailing_breaks = self.trailing_breaks.max(count);
        self.column = 0;
    }
    /// End a nonempty current line in memory; do nothing when column is zero. No I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: may append one missing newline and reset column/break state.
    ///
    /// # Returns
    /// `()`; existing blank lines remain.
    ///
    /// # Examples
    /// ```text
    /// out.line();
    /// ```
    pub(super) fn line(&mut self) {
        if self.column > 0 {
            self.breaks(1);
        }
    }
    /// Append spaces or tabs for a block depth in memory. No I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: indentation appends to text and updates columns/breaks.
    /// - `level` `[in]`: nonnegative indentation depth.
    ///
    /// # Returns
    /// `()`; appends level*indent_width spaces or level tabs.
    ///
    /// # Panics
    /// Space indentation can overflow for impractically large level*indent_width.
    ///
    /// # Examples
    /// ```text
    /// out.indent(2);
    /// ```
    pub(super) fn indent(&mut self, level: usize) {
        if self.options.insert_spaces {
            self.push(&" ".repeat(level * self.options.indent_width));
        } else {
            self.push(&"\t".repeat(level));
        }
    }
    /// Consume the writer, remove trailing CR/LF bytes and add one selected newline if nonempty. No I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: owned writer consumed; its buffer is trimmed rather than copied.
    ///
    /// # Returns
    /// Owned complete String; empty output stays empty, nonempty output has exactly one final newline.
    ///
    /// # Examples
    /// ```text
    /// let formatted = out.finish();
    /// ```
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
