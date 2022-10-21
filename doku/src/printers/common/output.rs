pub mod layouts;

use crate::printers::prelude::*;
use std::collections::BTreeMap;

pub trait Output<'a> {
    fn comment_separator(&self) -> &str;
    fn lines(&'a self) -> Lines<'a>;
}

pub struct Line<'a> {
    pub id: usize,
    pub indent: usize,
    pub body: &'a str,
    pub comments: &'a [String],
}

impl Line<'_> {
    pub fn len(&self) -> usize {
        self.indent + self.body.chars().count()
    }
}

pub struct Lines<'a> {
    lines: &'a [String],
    indent_size: usize,
    indents: &'a BTreeMap<usize, usize>,
    comments: &'a BTreeMap<usize, Vec<String>>,
}

impl<'a> Lines<'a> {
    pub fn new(
        lines: &'a [String],
        indent_size: usize,
        indents: &'a BTreeMap<usize, usize>,
        comments: &'a BTreeMap<usize, Vec<String>>,
    ) -> Self {
        Self {
            lines,
            indent_size,
            indents,
            comments,
        }
    }
}

impl<'a> IntoIterator for Lines<'a> {
    type Item = Line<'a>;

    type IntoIter = LinesIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LinesIter {
            lines: self,
            index: 0,
        }
    }
}

pub struct LinesIter<'a> {
    lines: Lines<'a>,
    index: usize,
}

impl<'a> Iterator for LinesIter<'a> {
    type Item = Line<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.lines.lines.get(self.index).map(|body| {
            let indent = self.lines.indent_size
                * self
                    .lines
                    .indents
                    .get(&self.index)
                    .copied()
                    .unwrap_or_default();

            let comments = self
                .lines
                .comments
                .get(&self.index)
                .map(|comments| &comments[..])
                .unwrap_or(&[]);

            Line {
                id: self.index,
                indent,
                body,
                comments,
            }
        });
        self.index += 1;
        result
    }
}
