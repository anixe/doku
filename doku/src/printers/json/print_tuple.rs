use super::*;

impl<'ty> Ctxt<'ty, '_> {
    pub fn print_tuple(&mut self, fields: &'ty [ty::Type]) {
        let fields: Vec<_> = fields
            .iter()
            .filter(|field| self.mode.allows(field.serializable, field.deserializable))
            .collect();

        if fields.is_empty() {
            self.out.text("[]");
            return;
        }

        if self.inline {
            self.out.text("[ ");
        } else {
            self.out.line("[");
            self.out.inc_indent();
        }

        for (field_id, field) in fields.iter().enumerate() {
            if field_id > 0 {
                if self.inline {
                    self.out.text(",");
                } else {
                    self.out.line(",");
                }
            }

            self.with_ty(&field).print();
        }

        if self.inline {
            self.out.text(" ]");
        } else {
            self.out.dec_indent();
            self.out.newline();
            self.out.text("]");
        }
    }
}
