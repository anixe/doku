use super::*;

impl<'ty> Ctxt<'ty, '_> {
    pub fn print_fields(&mut self, fields: &'ty ty::Fields, variant: Option<&'ty ty::Variant>) {
        match fields {
            ty::Fields::Named { fields } => self.print_named_fields(&fields, variant),
            ty::Fields::Unnamed { fields } => self.print_unnamed_fields(&fields),
            ty::Fields::Unit => self.out.text("null"),
        }
    }

    /// Prints named fields of given enum or struct.
    ///
    /// # Example
    ///
    /// ```ignore
    /// struct Foo {
    ///     foo: String,
    ///     bar: usize,
    /// }
    ///
    /// // gets rendered as:
    ///
    /// {
    ///   "foo": "string",
    ///   "bar": 123,
    /// }
    /// ```
    pub fn print_named_fields(&mut self, fields: &'ty [(&'static str, ty::Field)], variant: Option<&'ty ty::Variant>) {
        let fields: Vec<_> = fields
            .iter()
            .filter(|(_, field)| self.mode.allows(field.ty.serializable, field.ty.deserializable))
            .collect();

        if !self.flat {
            if fields.is_empty() {
                self.out.text("{}");
                return;
            }

            if self.inline {
                self.out.text("{ ");
            } else {
                self.out.line("{");
                self.out.inc_indent();
            }
        }

        for (field_id, (field_name, field)) in fields.into_iter().enumerate() {
            if field_id > 0 {
                if self.inline {
                    self.out.text(", ");
                } else {
                    self.out.line(",");
                }
            }

            self.print_named_field(field_name, field, variant);
        }

        if !self.flat {
            if self.inline {
                self.out.text(" }");
            } else {
                self.out.dec_indent();
                self.out.newline();
                self.out.text("}");
            }
        }
    }

    fn print_named_field(&mut self, field_name: &str, field: &'ty ty::Field, variant: Option<&'ty ty::Variant>) {
        // Edge case: fields that model adjacently-tagged enums; for more
        // information, please refer to `print_array.rs`
        if let Some(tag) = field.ty.tag {
            let variant = variant.expect(
                "Invalid internal state: At this point we should be expanding variants, but it seems like we're not",
            );

            // First things first, let's print the tag's name (e.g.: `"enum_tag": "Foo"`)
            self.out.line(format!(r#""{}": "{}","#, tag, variant.id,));

            // Second things second, let's print the tag's enum (e.g.
            // `"enum_inner": { ... }`)
            self.out.text(format!(r#""{}": "#, field_name,));

            self.print_fields(&variant.fields, None);
            return;
        }

        // Edge case: flattened fields
        if field.flattened {
            self.with_ty(&field.ty).with_flat().print();
            return;
        }

        // Regular case: `"field-name": { ... }`
        self.out.text(format!("\"{}\": ", field_name));
        self.with_ty(&field.ty).print();
    }

    /// Prints unnamed fields of given enum or struct.
    ///
    /// # Example
    ///
    /// ```ignore
    /// struct Foo(String);
    ///
    /// // gets rendered as:
    ///
    /// "string"
    /// ```
    ///
    /// ```ignore
    /// struct Bar(String, usize);
    ///
    /// // gets rendered as:
    ///
    /// ["string", 123]
    /// ```
    pub fn print_unnamed_fields(&mut self, fields: &'ty [ty::Field]) {
        let fields: Vec<_> = fields
            .iter()
            .filter(|field| self.mode.allows(field.ty.serializable, field.ty.deserializable))
            .collect();

        // Edge case: when there's just one field, Serde automatically "inlines" it
        if fields.len() > 1 {
            self.out.text("[");
        }

        for (field_id, field) in fields.iter().enumerate() {
            if field_id > 0 {
                self.out.text(",");
            }

            self.with_ty(&field.ty).print();
        }

        if fields.len() > 1 {
            self.out.text("]");
        }
    }
}
