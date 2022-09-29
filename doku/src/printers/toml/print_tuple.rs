use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn print_tuple(&mut self, fields: &'ty [Type]) {
        let fields: Vec<_> = fields
            .iter()
            .filter(|field| {
                self.vis.allows(field.serializable, field.deserializable)
            })
            .collect();

        if fields.is_empty() {
            self.out.write("[]");
            return;
        }

        self.out.writeln("[");
        self.out.inc_indent();

        for (field_id, field) in fields.iter().enumerate() {
            if field_id > 0 {
                self.out.writeln(",");
            }

            self.nested().with_ty(&field).print();
        }

        self.out.ln();
        self.out.dec_indent();
        self.out.write("]");
    }
}
