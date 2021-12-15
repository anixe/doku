mod condensed;
mod multiline;

use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn print_separated_enum(
        &mut self,
        tag: Tag,
        variants: &[&'ty Variant],
    ) {
        // TODO figure out what to do if `self.inline == true` (can happen for enums nested in enums)

        if !condensed::print(self, tag, variants) {
            multiline::print(self, tag, variants);
        }
    }
}
