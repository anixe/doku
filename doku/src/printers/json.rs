mod ctxt;
mod print_array;
mod print_comment;
mod print_enum;
mod print_fields;
mod print_map;
mod print_optional;
mod print_scalar;
mod print_struct;
mod print_tuple;

use self::ctxt::*;
use crate::printers::prelude::*;

#[derive(Clone, Debug)]
pub struct JsonPrinter {
    mode:     TypePrinterMode,
    comments: bool,
    indent:   usize,
}

impl JsonPrinter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_mode(&mut self, mode: TypePrinterMode) {
        self.mode = mode;
    }

    pub fn with_mode(mut self, mode: TypePrinterMode) -> Self {
        self.set_mode(mode);
        self
    }

    pub fn set_comments(&mut self, comments: bool) {
        self.comments = comments;
    }

    pub fn with_comments(mut self, comments: bool) -> Self {
        self.set_comments(comments);
        self
    }

    pub fn set_indent(&mut self, indent: usize) {
        self.indent = indent;
    }

    pub fn with_indent(mut self, indent: usize) -> Self {
        self.set_indent(indent);
        self
    }

    pub fn print(&self, ty: &Type) -> String {
        let mut out = Paragraph::new(self.indent, self.comments);

        Ctxt {
            out: &mut out,
            ty,
            parents: Default::default(),
            mode: self.mode,
            flat: false,
            inline: false,
        }
        .print();

        out.to_string()
    }
}

impl Default for JsonPrinter {
    fn default() -> Self {
        Self {
            mode:     TypePrinterMode::default(),
            comments: true,
            indent:   2,
        }
    }
}
