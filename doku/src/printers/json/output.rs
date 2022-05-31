mod layouts;

use super::*;
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
        self.write_char('\n');
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
            Layout::OneColumn => layouts::one_column::render(self),
            Layout::TwoColumns { align, spacing } => {
                layouts::two_columns::render(self, align, spacing)
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

    fn lines(&self) -> impl Iterator<Item = Line<'_>> + '_ {
        debug_assert!(self.line.is_empty());

        self.lines.iter().enumerate().map(move |(id, body)| {
            let indent = self.fmt.indent_style.size
                * self.indents.get(&id).copied().unwrap_or_default();

            let comments = self
                .comments
                .get(&id)
                .map(|comments| &comments[..])
                .unwrap_or(&[]);

            Line {
                id,
                indent,
                body,
                comments,
            }
        })
    }
}

struct Line<'a> {
    id: usize,
    indent: usize,
    body: &'a str,
    comments: &'a [String],
}

impl Line<'_> {
    fn len(&self) -> usize {
        self.indent + self.body.chars().count()
    }
}
