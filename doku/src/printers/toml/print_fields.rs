mod named;
mod newtype;
mod unnamed;

use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn print_fields(
        &mut self,
        fields: &Fields,
        variant: Option<&Variant>,
    ) {
        if self.try_printing_newtype_fields(fields) {
            return;
        }

        match fields {
            Fields::Named { fields } => {
                self.print_named_fields(&fields, variant);
            }
            Fields::Unnamed { fields } => {
                self.print_unnamed_fields(&fields);
            }
            Fields::Unit => {}
        }
    }
}
