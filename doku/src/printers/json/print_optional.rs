use super::*;

impl<'ty> Ctxt<'ty, '_> {
    pub fn print_optional(&mut self, ty: &'ty Type) {
        // Serde treats `Option<..>` as a transparent type (apart from the fact
        // that it might end up being `null`), and so do we:

        self.with_ty(ty).print()
    }
}
