use super::*;

pub(super) fn print<'ty>(
    ctxt: &mut Ctxt<'_, 'ty, '_>,
    tag: Tag,
    variants: &[&'ty Variant],
) {
    let indent = ctxt.parent.map_or(false, |parent| {
        matches!(parent.kind, TypeKind::Struct { .. })
    });

    if indent {
        ctxt.out.ln();
        ctxt.out.inc_indent();
    }

    for (variant_idx, variant) in variants.iter().enumerate() {
        if variant_idx > 0 {
            ctxt.out.write("\n// or\n");
        }

        if let Some(comment) = variant.comment {
            ctxt.out.writeln_comment(comment);
        }

        print_variant(ctxt, tag, variant);
    }

    if indent {
        ctxt.out.dec_indent();
    }
}

fn print_variant<'ty>(
    ctxt: &mut Ctxt<'_, 'ty, '_>,
    tag: Tag,
    variant: &'ty Variant,
) {
    match tag {
        Tag::Adjacent { tag, content } => {
            ctxt.out.writeln("{");
            ctxt.out.inc_indent();
            ctxt.out.write_key_and_separator(tag);
            ctxt.out.write(format!(r#""{}""#, variant.id));
            ctxt.out.write_property_separator_ln();

            if let Fields::Named { .. } | Fields::Unnamed { .. } =
                variant.fields
            {
                ctxt.out.write_key_and_separator(content);
                ctxt.print_fields(&variant.fields, None);
                ctxt.out.ln();
            }

            ctxt.out.dec_indent();
            ctxt.out.write("}");
        }

        Tag::Internal { tag } => {
            ctxt.out.writeln("{");
            ctxt.out.inc_indent();
            ctxt.out.write_key_and_separator(tag);
            ctxt.out.write(format!(r#""{}""#, variant.id));
            ctxt.out.write_property_separator_ln();

            if let Fields::Named { .. } | Fields::Unnamed { .. } =
                variant.fields
            {
                ctxt.nested()
                    .with_flat()
                    .print_fields(&variant.fields, None);

                ctxt.out.ln();
            }

            ctxt.out.dec_indent();
            ctxt.out.write("}");
        }

        Tag::External => match variant.fields {
            Fields::Named { .. } | Fields::Unnamed { .. } => {
                ctxt.out.writeln("{");
                ctxt.out.inc_indent();
                ctxt.out.write_key_and_separator(variant.id);
                ctxt.print_fields(&variant.fields, None);
                ctxt.out.ln();
                ctxt.out.dec_indent();
                ctxt.out.write("}");
            }

            Fields::Unit => {
                ctxt.out.write(format!(r#""{}""#, variant.id));
            }
        },

        Tag::None => match variant.fields {
            Fields::Named { .. } | Fields::Unnamed { .. } => {
                ctxt.print_fields(&variant.fields, None);
            }

            Fields::Unit => {
                ctxt.out.write("null");
            }
        },
    }
}
