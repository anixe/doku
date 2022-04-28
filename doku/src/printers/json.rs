mod ctxt;
mod formatting;
mod output;
mod print_array;
mod print_comment;
mod print_enum;
mod print_fields;
mod print_map;
mod print_optional;
mod print_scalar;
mod print_struct;
mod print_tuple;
mod value_to_string;

use self::{ctxt::*, output::*};
use crate::printers::prelude::*;
use std::borrow::Cow;

pub use self::formatting::*;

#[derive(Debug, Default)]
pub struct Printer<'a> {
    visibility: Visibility,
    formatting: Option<&'a Formatting>,
    value: Option<&'a Value>,
}

impl<'a> Printer<'a> {
    /// Specifies how `skip_serializing` / `skip_deserializing` should be
    /// understood:
    ///
    /// ```
    /// use doku::Document;
    /// use doku::json::*;
    /// use doku::Visibility;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize, Document)]
    /// struct Person {
    ///     name: String,
    ///     #[serde(skip_serializing)]
    ///     password: String,
    ///     surname: String,
    /// }
    ///
    /// let doc = Printer::default()
    ///     .with_visibility(Visibility::SerializableOnly)
    ///     .print(&Person::ty());
    ///
    /// doku::assert_doc!(r#"
    ///   {
    ///     "name": "string",
    ///     "surname": "string"
    ///   }
    /// "#, doc);
    /// ```
    pub fn set_visibility(&mut self, value: Visibility) {
        self.visibility = value;
    }

    /// A consuming variant of [`Self::set_visibility()`].
    pub fn with_visibility(mut self, value: Visibility) -> Self {
        self.set_visibility(value);
        self
    }

    pub fn set_formatting(&mut self, value: &'a Formatting) {
        self.formatting = Some(value);
    }

    /// A consuming variant of [`Self::set_formatting()`].
    pub fn with_formatting(mut self, value: &'a Formatting) -> Self {
        self.set_formatting(value);
        self
    }

    pub fn set_value(&mut self, value: &'a Value) {
        self.value = Some(value);
    }

    /// A consuming variant of [`Self::set_value()`].
    pub fn with_value(mut self, value: &'a Value) -> Self {
        self.set_value(value);
        self
    }

    pub fn print(&self, ty: &'a Type) -> String {
        let fmt = self
            .formatting
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Owned(Default::default()));

        let mut out = Output::new(fmt.as_ref());

        Ctxt {
            ty,
            val: self.value,
            vis: self.visibility,
            fmt: fmt.as_ref(),
            out: &mut out,
            parent: Default::default(),
            example: Default::default(),
            flat: Default::default(),
            depth: Default::default(),
        }
        .print();

        out.render()
    }
}
