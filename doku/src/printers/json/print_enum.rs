use super::*;

impl Ctxt<'_, '_> {
    pub fn print_enum(&mut self, tag: Tag, variants: &[Variant]) {
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
            Tag::Adjacent { tag, content } => self.print_variant_of_adjacently_tagged_enum(tag, content, variant),
            Tag::Internal { tag } => self.print_variant_of_internally_tagged_enum(tag, variant),
            Tag::External => self.print_variant_of_externally_tagged_enum(variant),
            Tag::None => self.print_variant_of_untagged_enum(variant),
        }
    }

    fn print_variant_of_adjacently_tagged_enum(&mut self, tag: &str, content: &str, variant: &Variant) {
        match variant.fields {
            Fields::Named { .. } | Fields::Unnamed { .. } => {
                self.out
                    .text(format!(r#"{{ "{}": "{}", "{}": ... }}"#, tag, variant.id, content));
            }

            Fields::Unit => {
                self.out.text(format!(r#"{{ "{}": "{}" }}"#, tag, variant.id));
            }
        }
    }

    fn print_variant_of_externally_tagged_enum(&mut self, variant: &Variant) {
        match variant.fields {
            Fields::Named { .. } | Fields::Unnamed { .. } => {
                self.out.text(format!(r#"{{ "{}": ... }}"#, variant.id));
            }

            Fields::Unit => {
                self.out.text(format!(r#""{}""#, variant.id));
            }
        }
    }

    fn print_variant_of_internally_tagged_enum(&mut self, tag: &str, variant: &Variant) {
        match variant.fields {
            Fields::Named { .. } | Fields::Unnamed { .. } => {
                self.out.text(format!(r#"{{ "{}": ... }}"#, tag,));
            }

            Fields::Unit => {
                self.out.text(format!(r#"{{ "{}" }}"#, tag,));
            }
        }
    }

    fn print_variant_of_untagged_enum(&mut self, variant: &Variant) {
        match variant.fields {
            Fields::Named { .. } | Fields::Unnamed { .. } => {
                self.out.text("{ ... }");
            }

            Fields::Unit => {
                self.out.text("null");
            }
        }
    }
}
