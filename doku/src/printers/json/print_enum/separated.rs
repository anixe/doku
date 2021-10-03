mod condensed;
mod multiline;

use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn print_separated_enum(
        &mut self,
        tag: Tag,
        variants: &[&'ty Variant],
    ) {
        // We can enter inline-mode only when printing commented-enums, which
        // shouldn't happen here
        debug_assert!(!self.inline);

        if !condensed::print(self, tag, variants) {
            multiline::print(self, tag, variants);
        }
    }
}
