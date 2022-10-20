mod condensed;
mod multiline;

use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn print_separated_enum(
        &mut self,
        tag: Tag,
        variants: &[&'ty Variant],
    ) {
        if !condensed::print(self, tag, variants) {
            multiline::print(self, tag, variants);
        }
    }
}
