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

        let variants = self.collect_variants(variants);

        for (variant_idx, variant) in variants.iter().enumerate() {
            let comment = variant.comment.or_else(|| {
                if variant.id == variant.title {
                    None
                } else {
                    Some(variant.title)
                }
            });

            match variant.fields {
                Fields::Named { .. } | Fields::Unnamed { .. } => {
                    if !variant.fields.is_table() {
                        self.print_current_name();
                    }

                    if let Some(comment) = comment {
                        self.out.writeln_comment(comment);
                    }

                    let indent = self.should_indent();

                    if indent {
                        self.out.inc_indent();
                    }

                    let is_table = variant.fields.is_table();

                    self.out.space_between_fields(is_table, 0);
                    self.print_child_name(variant.id, is_table);
                    self.print_fields(&variant.fields, None, false);
                    self.out.ln();

                    if indent {
                        self.out.dec_indent();
                    }
                }

                Fields::Unit => {
                    if variant_idx > 0 {
                        self.out.writeln(",");
                    }

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
