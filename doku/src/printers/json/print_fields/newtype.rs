use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn try_printing_newtype_fields(
        &mut self,
        fields: &'ty Fields,
    ) -> bool {
        let fields = if let Fields::Unnamed { fields } = fields {
            fields
        } else {
            return false;
        };

        let field = if fields.len() == 1 {
            &fields[0]
        } else {
            return false;
        };

        let field_val = self.val.and_then(|val| val.as_struct_unnamed_field(0));
        let field_example = self.example();
        let is_flat = self.flat;

        let mut ctxt = self
            .nested()
            .with_ty(&field.ty)
            .with_val(field_val)
            .with_example(field_example);

        if is_flat {
            ctxt = ctxt.with_flat();
        }

        ctxt.print();

        true
    }
}
