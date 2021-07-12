use super::*;

impl<'ty> Ctxt<'ty, '_> {
    pub fn expand_variants_for_externally_tagged_enum(&mut self, ty: &'ty Type) -> bool {
        let variants = if let TypeDef::Enum {
            tag: Tag::External,
            variants,
        } = &ty.def
        {
            variants
        } else {
            return false;
        };

        let variants: Vec<_> = variants
            .iter()
            .filter(|variant| self.mode.allows(variant.serializable, variant.deserializable))
            .enumerate()
            .collect();

        for (variant_idx, variant) in variants {
            if variant_idx > 0 {
                self.out.line(",");
            }

            let comment = variant.comment.or_else(|| {
                if variant.id == variant.title {
                    None
                } else {
                    Some(variant.title)
                }
            });

            match variant.fields {
                Fields::Named { .. } | Fields::Unnamed { .. } => {
                    self.out.line("{");

                    if let Some(comment) = comment {
                        self.out.comment(format!("// {}", comment));
                    }

                    self.out.inc_indent();
                    self.out.text(format!(r#""{}": "#, variant.id));
                    self.print_fields(&variant.fields, None);
                    self.out.newline();
                    self.out.dec_indent();
                    self.out.text("}");
                }

                Fields::Unit => {
                    if let Some(comment) = comment {
                        self.out.comment(format!("// {}", comment));
                    }

                    self.out.text(format!(r#""{}""#, variant.id,));
                }
            }
        }

        self.out.newline();

        true
    }
}
