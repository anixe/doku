use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn try_expanding_externally_tagged_variants(
        &mut self,
        ty: &'ty Type,
    ) -> bool {
        let variants = if let TypeKind::Enum {
            tag: Tag::External,
            variants,
        } = &ty.kind
        {
            variants
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

            match variant.fields {
                Fields::Named { .. } | Fields::Unnamed { .. } => {
                    self.out.writeln("{");

                    if let Some(comment) = comment {
                        self.out.writeln_comment(comment);
                    }

                    self.out.inc_indent();
                    self.out.write(format!(r#""{}": "#, variant.id));
                    self.print_fields(&variant.fields, None);
                    self.out.ln();
                    self.out.dec_indent();
                    self.out.write("}");
                }

                Fields::Unit => {
                    if let Some(comment) = comment {
                        self.out.writeln_comment(comment);
                    }

                    self.out.write(format!(r#""{}""#, variant.id));
                }
            }
        }

        self.out.ln();

        true
    }
}
