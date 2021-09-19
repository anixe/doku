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

#[derive(Debug)]
pub struct JsonPrinter {
    value:    Option<val::Value>,
    mode:     TypePrinterMode,
    comments: bool,
    indent:   usize,
}

impl JsonPrinter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_value(&mut self, value: impl Into<Option<val::Value>>) {
        self.value = value.into();
    }

    pub fn with_value(mut self, value: impl Into<Option<val::Value>>) -> Self {
        self.set_value(value);
        self
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

    pub fn print(&self, ty: &ty::Type) -> String {
        let mut out = Paragraph::new(self.indent, self.comments);

        Ctxt {
            ty,
            val: self.value.as_ref(),
            out: &mut out,
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
            value:    None,
            mode:     TypePrinterMode::default(),
            comments: true,
            indent:   2,
        }
    }
}
