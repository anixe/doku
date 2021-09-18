use super::*;

impl Ctxt<'_, '_> {
    pub fn print_enum(&mut self, tag: ty::Tag, variants: &[ty::Variant]) {
        if let Some(example) = self.example() {
            self.out.text(example);

            // Manually-provided examples always take priority over our
            // automatically inferred ones - so if someone has already provided
            // an example, there's nothing more to do here
            return;
        }

        // If user hasn't provided any example, let's just pick the first
        // possible enum's variant and pretty-print it to the user
        let variant = variants
            .iter()
            .find(|variant| self.mode.allows(variant.serializable, variant.deserializable));

        // From the type system perspective, it's perfectly legal to create a
        // empty enum (`enum Foo {}`) - it shouldn't happen in practice though,
        // because such type makes little to no sense when it comes to using
        // them. So, if we ever encounter such enum, we're just bailing-out
        // with a panic.
        let variant = variant.expect("Found an enum without usable variants");

        match tag {
            ty::Tag::Adjacent { tag, content } => self.print_variant_of_adjacently_tagged_enum(tag, content, variant),
            ty::Tag::Internal { tag } => self.print_variant_of_internally_tagged_enum(tag, variant),
            ty::Tag::External => self.print_variant_of_externally_tagged_enum(variant),
            ty::Tag::None => self.print_variant_of_untagged_enum(variant),
        }
    }

    fn print_variant_of_adjacently_tagged_enum(&mut self, tag: &str, content: &str, variant: &ty::Variant) {
        match variant.fields {
            ty::Fields::Named { .. } | ty::Fields::Unnamed { .. } => {
                self.out
                    .text(format!(r#"{{ "{}": "{}", "{}": ... }}"#, tag, variant.id, content));
            }

            ty::Fields::Unit => {
                self.out.text(format!(r#"{{ "{}": "{}" }}"#, tag, variant.id));
            }
        }
    }

    fn print_variant_of_externally_tagged_enum(&mut self, variant: &ty::Variant) {
        match variant.fields {
            ty::Fields::Named { .. } | ty::Fields::Unnamed { .. } => {
                self.out.text(format!(r#"{{ "{}": ... }}"#, variant.id));
            }

            ty::Fields::Unit => {
                self.out.text(format!(r#""{}""#, variant.id));
            }
        }
    }

    fn print_variant_of_internally_tagged_enum(&mut self, tag: &str, variant: &ty::Variant) {
        match variant.fields {
            ty::Fields::Named { .. } | ty::Fields::Unnamed { .. } => {
                self.out.text(format!(r#"{{ "{}": ... }}"#, tag,));
            }

            ty::Fields::Unit => {
                self.out.text(format!(r#"{{ "{}" }}"#, tag,));
            }
        }
    }

    fn print_variant_of_untagged_enum(&mut self, variant: &ty::Variant) {
        match variant.fields {
            ty::Fields::Named { .. } | ty::Fields::Unnamed { .. } => {
                self.out.text("{ ... }");
            }

            ty::Fields::Unit => {
                self.out.text("null");
            }
        }
    }
}
