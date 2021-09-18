mod for_adjacently_tagged_enum;
mod for_externally_tagged_enum;
mod for_untagged_enum;

use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn expand_variants(&mut self, ty: &'ty Type) -> bool {
        if self.inline {
            return false;
        }

        self.expand_variants_for_adjacently_tagged_enum(ty)
            || self.expand_variants_for_externally_tagged_enum(ty)
            || self.expand_variants_for_untagged_enum(ty)
    }
}
