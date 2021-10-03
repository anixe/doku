use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn print_unnamed_fields(&mut self, fields: &'ty [Field]) {
        let fields: Vec<_> = fields
            .iter()
            .filter(|field| {
                self.vis
                    .allows(field.ty.serializable, field.ty.deserializable)
            })
            .collect();

        if !self.flat {
            self.out.write("[");
        }

        for (field_id, field) in fields.iter().enumerate() {
            if field_id > 0 {
                self.out.write(", ");
            }

            let field_val = self
                .val
                .and_then(|val| val.as_struct_unnamed_field(field_id));

            self.nested().with_ty(&field.ty).with_val(field_val).print();
        }

        if !self.flat {
            self.out.write("]");
        }
    }
}
