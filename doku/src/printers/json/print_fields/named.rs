use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
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
    pub(super) fn print_named_fields(
        &mut self,
        fields: &'ty [(&'static str, Field)],
        variant: Option<&'ty Variant>,
    ) {
        let fields: Vec<_> = fields
            .iter()
            .filter(|(_, field)| {
                self.vis
                    .allows(field.ty.serializable, field.ty.deserializable)
            })
            .collect();

        if !self.flat {
            if fields.is_empty() {
                self.out.write("{}");
                return;
            }

            if self.inline {
                self.out.write("{ ");
            } else {
                self.out.writeln("{");
                self.out.inc_indent();
            }
        }

        for (field_id, (field_name, field)) in fields.into_iter().enumerate() {
            if field_id > 0 {
                if self.inline {
                    self.out.write(", ");
                } else {
                    self.out.writeln(",");
                }
            }

            self.print_named_field(field_name, field, variant);
        }

        if !self.flat {
            if self.inline {
                self.out.write(" }");
            } else {
                self.out.ln();
                self.out.dec_indent();
                self.out.write("}");
            }
        }
    }

    fn print_named_field(
        &mut self,
        field_name: &str,
        field: &'ty Field,
        variant: Option<&'ty Variant>,
    ) {
        let field_val = self
            .val
            .and_then(|val| val.as_struct_named_field(field_name));

        // Edge case: fields that model adjacently-tagged enums; for more
        // information, please refer to `print_array.rs`
        if let Some(tag) = field.ty.tag {
            let variant = variant.expect(
                "Invalid internal state: At this point we should be expanding \
                 variants, but it seems like we're not",
            );

            // First things first, let's print the tag's name (e.g.:
            // `"enum_tag": "Foo"`)
            self.out.writeln(format!(r#""{}": "{}","#, tag, variant.id));

            // Second things second, let's print the tag's enum (e.g.
            // `"enum_inner": { ... }`)
            self.out.write(format!(r#""{}": "#, field_name,));

            self.print_fields(&variant.fields, None);
            return;
        }

        // Edge case: flattened fields
        if field.flattened {
            self.nested()
                .with_ty(&field.ty)
                .with_val(field_val)
                .with_flat()
                .print();

            return;
        }

        // Regular case: `"field-name": { ... }`
        self.out.write(format!("\"{}\": ", field_name));
        self.nested().with_ty(&field.ty).with_val(field_val).print();
    }
}
