use super::*;

impl<'ty> Ctxt<'ty, '_> {
    pub fn expand_variants_for_adjacently_tagged_enum(&mut self, ty: &'ty Type) -> bool {
        let (tag, content, variants) = if let TypeDef::Enum {
            tag: Tag::Adjacent { tag, content },
            variants,
        } = &ty.def
        {
            (tag, content, variants)
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

            self.out.line("{");

            if let Some(comment) = comment {
                self.out.comment(format!("// {}", comment));
            }

            self.out.inc_indent();
            self.out.text(format!(r#""{}": "{}""#, tag, variant.id));

            if let Fields::Named { .. } | Fields::Unnamed { .. } = variant.fields {
                self.out.line(",");
                self.out.text(format!(r#""{}": "#, content));
                self.print_fields(&variant.fields, None);
            }

            self.out.newline();
            self.out.dec_indent();
            self.out.text("}");
        }

        self.out.newline();

        true
    }
}
