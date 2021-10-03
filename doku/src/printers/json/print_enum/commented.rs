mod comment;
mod sketch;

use super::*;

impl Ctxt<'_, '_, '_> {
    pub(super) fn print_commented_enum(
        &mut self,
        tag: Tag,
        variants: &[&Variant],
    ) {
        comment::comment(self, tag, variants);
        sketch::sketch(self, tag, variants);
    }
}
