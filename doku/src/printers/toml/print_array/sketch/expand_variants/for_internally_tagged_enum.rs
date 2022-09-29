use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn try_expanding_internally_tagged_variants(
        &mut self,
        ty: &'ty Type,
    ) -> bool {
        let (tag, variants) = if let TypeKind::Enum {
            tag: Tag::Internal { tag },
            variants,
        } = &ty.kind
        {
            (tag, variants)
        } else {
            return false;
        };

        let variants = self.collect_variants_recursive(variants);
        for variant in &variants {
            self.print_current_name();
            let comment = variant.comment.or_else(|| {
                if variant.id == variant.title {
                    None
                } else {
                    Some(variant.title)
                }
            });
            if let Some(comment) = comment {
                self.out.writeln_comment(comment);
            }

            let indent = self.should_indent();
            if indent {
                self.out.inc_indent();
            }

            self.out.write_key_and_separator(tag);
            self.out.writeln(format!(r#""{}""#, variant.id));

            if let Fields::Named { .. } | Fields::Unnamed { .. } =
                variant.fields
            {
                if !variant.fields.is_table() {
                    panic!("Internally tagged scalar variants are unsupported in TOML")
                }
                self.print_fields(&variant.fields, None);
            }
            if indent {
                self.out.dec_indent();
            }
        }

        self.out.ln();
        true
    }
}
