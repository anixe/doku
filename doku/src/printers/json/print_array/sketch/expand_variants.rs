mod for_adjacently_tagged_enum;
mod for_externally_tagged_enum;
mod for_untagged_enum;

use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn try_expanding_variants(&mut self, ty: &'ty Type) -> bool {
        self.try_expanding_adjacently_tagged_variants(ty)
            || self.try_expanding_externally_tagged_variants(ty)
            || self.try_expanding_untagged_variants(ty)
    }
}
