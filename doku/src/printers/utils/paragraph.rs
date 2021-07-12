use std::collections::BTreeMap;
use std::iter::once;
use std::{fmt, mem};

#[derive(Clone, Debug, Default)]
pub struct Paragraph {
    lines:            Vec<String>,
    comments:         BTreeMap<usize, Vec<String>>,
    comments_enabled: bool,

    indent_level:   usize,
    indent_size:    usize,
    indent_pending: bool,

    current_line: String,
}

impl Paragraph {
    pub fn new(indent_size: usize, comments_enabled: bool) -> Self {
        Self {
            indent_size,
            comments_enabled,
            ..Default::default()
        }
    }

    pub fn text(&mut self, str: impl ToString) {
        for ch in str.to_string().chars() {
            self.char(ch);
        }
    }

    pub fn newline(&mut self) {
        self.char('\n');
    }

    pub fn line(&mut self, str: impl ToString) {
        self.text(str);
        self.newline();
    }

    pub fn comment(&mut self, comment: impl ToString) {
        if !self.comments_enabled {
            return;
        }

        let comment = comment.to_string();

        for comment in comment.split('\n') {
            let current_line_id = self.lines.len();

            self.comments
                .entry(current_line_id)
                .or_default()
                .push(comment.to_owned());
        }
    }

    pub fn inc_indent(&mut self) {
        self.indent_level += 1;
    }

    pub fn dec_indent(&mut self) {
        self.indent_level -= 1;
    }

    fn char(&mut self, ch: char) {
        match ch {
            '\t' => {
                self.text(" ".repeat(self.indent_size));
            }

            '\r' => {
                //
            }

            '\n' => self.scroll(),

            ch => {
                if self.indent_pending {
                    self.indent_pending = false;

                    for _ in 0..self.indent_level * self.indent_size {
                        self.current_line.push(' ');
                    }
                }

                self.current_line.push(ch)
            }
        }
    }

    fn scroll(&mut self) {
        let line = mem::take(&mut self.current_line);

        self.lines.push(line);
        self.indent_pending = true;
    }

    fn lines(&self) -> Box<dyn Iterator<Item = &String> + '_> {
        if self.current_line.is_empty() {
            Box::new(self.lines.iter())
        } else {
            Box::new(self.lines.iter().chain(once(&self.current_line)))
        }
    }
}

impl fmt::Display for Paragraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let comment_indent = {
            let max_line_length = self.lines().map(|line| line.chars().count()).max().unwrap_or(0);

            max_line_length + 2
        };

        for (line_id, line) in self.lines().enumerate() {
            if line_id > 0 {
                writeln!(f)?;
            }

            write!(f, "{}", line)?;

            if let Some(comments) = self.comments.get(&line_id) {
                // Since current line already contains some text (most likely), we don't have to
                // fill it entirely with spaces - just the remaining space
                for _ in line.chars().count()..comment_indent {
                    write!(f, " ")?;
                }

                for (comment_id, comment) in comments.iter().enumerate() {
                    // ... all further lines are empty though, so they have be filled with spaces
                    // entirely
                    if comment_id > 0 {
                        writeln!(f)?;

                        for _ in 0..comment_indent {
                            write!(f, " ")?;
                        }
                    }

                    write!(f, "{}", comment)?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions as pa;

    #[test]
    fn paragraph_with_single_line_comments() {
        let actual = {
            let mut para = Paragraph::new(2, true);

            para.comment("// first-line comment");
            para.line("first line");
            para.line("second line");
            para.comment("// third-line comment");
            para.line("third line");
            para.to_string()
        };

        let expected = indoc!(
            r#"
            first line   // first-line comment
            second line
            third line   // third-line comment"#
        );

        pa::assert_eq!(expected, actual);
    }

    #[test]
    fn paragraph_with_multi_line_comments() {
        let actual = {
            let mut para = Paragraph::new(2, true);

            para.line("first line");
            para.comment("// This is a big comment:");
            para.comment("// - foo");
            para.comment("// - bar");
            para.line("second line");
            para.line("third line");
            para.to_string()
        };

        let expected = indoc!(
            r#"
            first line
            second line  // This is a big comment:
                         // - foo
                         // - bar
            third line"#
        );

        pa::assert_eq!(expected, actual);
    }

    #[test]
    fn paragraph_with_unfinished_line() {
        let actual = {
            let mut para = Paragraph::new(0, true);

            para.text("hello");
            para.to_string()
        };

        let expected = "hello";

        pa::assert_eq!(expected, actual);
    }
}
