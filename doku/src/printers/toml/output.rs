use super::*;
use crate::common::{layouts, Lines};
use std::collections::BTreeMap;
use std::mem;

#[derive(Debug)]
pub struct Output {
    fmt: Formatting,

    /// Pending line
    line: String,

    /// Pending indent
    indent: usize,

    /// Written, completed lines
    lines: Vec<String>,

    /// Mapping from line number to comments associated with that line
    comments: BTreeMap<usize, Vec<String>>,

    /// Mapping from line number to indenting level present at that line
    indents: BTreeMap<usize, usize>,
}

impl Output {
    pub fn new(fmt: &Formatting) -> Self {
        Self {
            // Borrowing `fmt` makes the code awkward in a few places, so let's
            // just clone it
            fmt: fmt.to_owned(),
            line: Default::default(),
            indent: Default::default(),
            lines: Default::default(),
            comments: Default::default(),
            indents: Default::default(),
        }
    }

    pub fn write_key_and_separator(&mut self, key: impl ToString) {
        self.write(key);
        self.write(" = ");
    }

    pub fn write_table_name(
        &mut self,
        table_name: impl ToString,
        has_array_parent: bool,
    ) {
        self.write_char('[');
        if has_array_parent {
            self.write_char('[');
        }
        self.write(table_name);
        if has_array_parent {
            self.write_char(']');
        }
        self.write_char(']');
        self.ln()
    }

    pub fn write(&mut self, str: impl ToString) {
        for ch in str.to_string().chars() {
            self.write_char(ch);
        }
    }

    pub fn writeln(&mut self, str: impl ToString) {
        self.write(str);
        self.ln();
    }

    pub fn ln(&mut self) {
        if !self.line.is_empty() {
            self.write_char('\n');
        }
    }

    fn ln_multiple(&mut self, count: usize) {
        if !self.lines.is_empty() {
            self.ln();
            for _ in 0..count {
                self.write_char('\n');
            }
        }
    }

    pub fn space_between_fields(&mut self, is_table: bool) {
        if is_table {
            self.space_between_tables();
        } else {
            self.space_between_scalar();
        }
    }

    pub fn space_between_scalar(&mut self) {
        self.ln_multiple(self.fmt.spacing.lines_between_scalar_fields);
    }

    pub fn space_between_tables(&mut self) {
        self.ln_multiple(self.fmt.spacing.lines_between_tables);
    }

    pub fn writeln_comment(&mut self, comment: impl ToString) {
        let comment = comment.to_string();

        for comment in comment.split('\n') {
            let comment = if comment.contains('\t') {
                comment.replace('\t', &" ".repeat(self.fmt.indent_style.size))
            } else {
                comment.to_owned()
            };

            self.comments
                .entry(self.lines.len())
                .or_default()
                .push(comment);
        }
    }

    pub fn append_comment(&mut self, f: impl FnOnce(&mut String)) {
        let mut comment = self
            .comments
            .get_mut(&self.lines.len())
            .and_then(|comments| {
                if comments.len() == 1 {
                    comments.pop()
                } else {
                    None
                }
            })
            .unwrap_or_default();

        f(&mut comment);

        self.writeln_comment(comment);
    }

    pub fn inc_indent(&mut self) {
        self.indent += 1;
    }

    pub fn dec_indent(&mut self) {
        self.indent -= 1;
    }

    pub fn render(mut self) -> String {
        if !self.line.is_empty() {
            self.write_char('\n');
        }

        match self.fmt.layout {
            Layout::OneColumn => layouts::one_column::render(&self),
            Layout::TwoColumns { align, spacing } => {
                layouts::two_columns::render(&self, align, spacing)
            }
        }
    }

    fn write_char(&mut self, ch: char) {
        match ch {
            '\t' => {
                self.write(" ".repeat(self.fmt.indent_style.size));
            }

            '\r' => {
                //
            }

            '\n' => {
                self.lines.push(mem::take(&mut self.line));
            }

            ch => {
                if self.line.is_empty() {
                    self.indents.insert(self.lines.len(), self.indent);
                }

                self.line.push(ch);
            }
        }
    }
}

impl<'a> common::Output<'a> for Output {
    fn comment_separator(&self) -> &str {
        &self.fmt.comments_style.separator
    }

    fn lines(&'a self) -> Lines<'a> {
        Lines::new(
            &self.lines,
            self.fmt.indent_style.size,
            &self.indents,
            &self.comments,
        )
    }
}
