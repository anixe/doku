mod commented;
mod separated;

use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn print_enum(&mut self, tag: Tag, variants: &'ty [Variant]) {
        let variants: Vec<_> = variants
            .iter()
            .filter(|variant| {
                self.vis
                    .allows(variant.serializable, variant.deserializable)
            })
            .collect();

        match self.fmt.enums_style {
            EnumsStyle::Commented => {
                self.print_commented_enum(tag, &variants);
            }
            EnumsStyle::Separated => {
                self.print_separated_enum(tag, &variants);
            }
        }
    }
}
