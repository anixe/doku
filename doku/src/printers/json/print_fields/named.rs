use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
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

            self.out.writeln("{");
            self.out.inc_indent();
        }

        for (field_id, (field_name, field)) in fields.into_iter().enumerate() {
            self.print_named_field(field_id, field_name, field, variant);
        }

        if !self.flat {
            self.out.ln();
            self.out.dec_indent();
            self.out.write("}");
        }
    }

    fn print_named_field(
        &mut self,
        field_id: usize,
        field_name: &str,
        field: &'ty Field,
        variant: Option<&'ty Variant>,
    ) {
        if field_id > 0 {
            self.out.writeln(",");
        }

        let field_val = self
            .val
            .and_then(|val| val.as_struct_named_field(field_name));

        if let Some(tag) = field.ty.tag {
            let variant = variant.expect(
                "Invalid internal state: At this point we should be expanding \
                 variants, but it seems like we're not",
            );

            self.out.writeln(format!(r#""{}": "{}","#, tag, variant.id));
            self.out.write(format!(r#""{}": "#, field_name));
            self.print_fields(&variant.fields, None);

            return;
        }

        if field.flattened {
            self.nested()
                .with_ty(&field.ty)
                .with_val(field_val)
                .with_flat()
                .print();
        } else {
            self.out.write(format!("\"{}\": ", field_name));
            self.nested().with_ty(&field.ty).with_val(field_val).print();
        }
    }
}
