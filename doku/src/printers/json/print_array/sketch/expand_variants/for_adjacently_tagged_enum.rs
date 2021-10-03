use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn try_expanding_adjacently_tagged_variants(
        &mut self,
        ty: &'ty Type,
    ) -> bool {
        let (tag, content, variants) = if let TypeKind::Enum {
            tag: Tag::Adjacent { tag, content },
            variants,
        } = &ty.kind
        {
            (tag, content, variants)
        } else {
            return false;
        };

        let variants: Vec<_> = variants
            .iter()
            .filter(|variant| {
                self.vis
                    .allows(variant.serializable, variant.deserializable)
            })
            .enumerate()
            .collect();

        for (variant_idx, variant) in variants {
            if variant_idx > 0 {
                self.out.writeln(",");
            }

            let comment = variant.comment.or_else(|| {
                if variant.id == variant.title {
                    None
                } else {
                    Some(variant.title)
                }
            });

            self.out.writeln("{");

            if let Some(comment) = comment {
                self.out.writeln_comment(comment);
            }

            self.out.inc_indent();
            self.out.write(format!(r#""{}": "{}""#, tag, variant.id));

            if let Fields::Named { .. } | Fields::Unnamed { .. } =
                variant.fields
            {
                self.out.writeln(",");
                self.out.write(format!(r#""{}": "#, content));
                self.print_fields(&variant.fields, None);
            }

            self.out.ln();
            self.out.dec_indent();
            self.out.write("}");
        }

        self.out.ln();

        true
    }
}
