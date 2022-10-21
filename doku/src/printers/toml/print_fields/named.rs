use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn print_named_fields(
        &mut self,
        has_preceding_field: bool,
        fields: &[(&'static str, Field)],
        variant: Option<&Variant>,
    ) {
        let mut fields: Vec<_> = fields
            .iter()
            .filter(|(_, field)| {
                self.vis
                    .allows(field.ty.serializable, field.ty.deserializable)
            })
            .collect();

        // We sort the values before the tables, because the result gets pretty
        // confusing otherwise.
        //
        // The `toml` crate does not support values after tables at all.
        fields.sort_by_key(|(_, f)| f.is_table());

        if !self.flat && self.should_indent() {
            self.out.inc_indent();
        }

        for (field_id, (field_name, field)) in fields.into_iter().enumerate() {
            let has_preceding_field = if field_id > 0 {
                true
            } else {
                has_preceding_field
            };

            self.print_named_field(
                has_preceding_field,
                field_name,
                field,
                variant,
            );
        }

        if !self.flat && self.should_indent() {
            self.out.ln();
            self.out.dec_indent();
        }
    }

    fn print_named_field(
        &mut self,
        has_preceding_field: bool,
        field_name: &str,
        field: &Field,
        variant: Option<&Variant>,
    ) {
        let mut already_added_spacing = 0;

        if has_preceding_field {
            self.out.ln();

            if field.ty.comment.is_some() && !field.is_table() {
                for _ in 0..self.fmt.spacing.lines_between_scalar_field_comments
                {
                    self.out.force_ln();
                    already_added_spacing += 1;
                }
            }
        }

        let field_val = self
            .val
            .and_then(|val| val.as_struct_named_field(field_name));

        if let Some(tag) = field.ty.tag {
            let variant = variant.expect(
                "Invalid internal state: At this point we should be expanding \
                 variants, but it seems like we're not",
            );

            self.out.space_between_scalar(already_added_spacing);
            self.out.write_key_and_separator(tag);
            self.out.write(format!(r#""{}""#, variant.id));
            self.out.ln();

            if !matches!(variant.fields, Fields::Unit) {
                let is_table = variant.fields.is_table();

                self.out
                    .space_between_fields(is_table, already_added_spacing);
                self.print_child_name(field_name, is_table);
                self.print_fields(&variant.fields, None, false);
            }

            return;
        }

        if field.flattened {
            self.nested()
                .with_ty(&field.ty)
                .with_val(field_val)
                .with_flat()
                .print();
        } else {
            let child_name = self.name_for_child(field_name);
            let mut nested =
                self.nested().with_ty(&field.ty).with_val(field_val);

            if nested.is_table() {
                nested = nested.with_name(Some(child_name));
            } else {
                nested.out.space_between_scalar(already_added_spacing);
                nested.out.write_key_and_separator(field_name);
            }

            nested.print();
        }
    }
}
