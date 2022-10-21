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

        for variant in self.collect_variants(variants) {
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
                let is_table = variant.fields.is_table();

                self.out.space_between_fields(is_table, 0);
                self.print_child_name(content, is_table);
                self.print_fields(&variant.fields, None, false);
            }

            self.out.ln();

            if indent {
                self.out.dec_indent();
            }
        }

        self.out.ln();
        true
    }
}
