use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
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
    pub(super) fn print_unnamed_fields(&mut self, fields: &'ty [Field]) {
        let fields: Vec<_> = fields
            .iter()
            .filter(|field| {
                self.vis
                    .allows(field.ty.serializable, field.ty.deserializable)
            })
            .collect();

        // Edge case: when there's just one field, Serde automatically inlines
        // it
        if fields.len() > 1 {
            self.out.write("[");
        }

        // TODO values would have to be printed as an array, I think; currently
        // this doesn't look good
        for (field_id, field) in fields.iter().enumerate() {
            let field_val = self
                .val
                .and_then(|val| val.as_struct_unnamed_field(field_id));

            if field_id > 0 {
                self.out.write(", ");
            }

            self.nested().with_ty(&field.ty).with_val(field_val).print();
        }

        if fields.len() > 1 {
            self.out.write("]");
        }
    }
}
