use super::*;

pub(super) fn print<'ty>(
    ctxt: &mut Ctxt<'_, 'ty, '_>,
    tag: Tag,
    variants: &[&'ty Variant],
) {
    let indent = ctxt.should_indent()
        && ctxt.parent.map_or(false, |parent| {
            matches!(parent.kind, TypeKind::Struct { .. })
        });

    if indent {
        ctxt.out.ln();
        ctxt.out.inc_indent();
    }

    for (variant_idx, variant) in variants.iter().enumerate() {
        if ctxt.is_table() {
            print_table_variant(ctxt, tag, variant_idx, variant);
        } else {
            print_scalar_variant(ctxt, tag, variant_idx, variant);
        }
    }

    if indent {
        ctxt.out.dec_indent();
    }
}

fn print_table_variant<'ty>(
    ctxt: &mut Ctxt<'_, 'ty, '_>,
    tag: Tag,
    variant_idx: usize,
    variant: &'ty Variant,
) {
    if variant_idx > 0 {
        ctxt.out.write("\n# or\n\n");
    }

    if let Some(comment) = variant.comment {
        ctxt.out.writeln_comment(comment);
    }

    print_variant(ctxt, tag, variant);
}

fn print_scalar_variant<'ty>(
    ctxt: &mut Ctxt<'_, 'ty, '_>,
    tag: Tag,
    variant_idx: usize,
    variant: &'ty Variant,
) {
    if variant_idx > 0 {
        ctxt.out.write("\n# or\n");
    }

    ctxt.out.inc_indent();
    if let Some(comment) = variant.comment {
        ctxt.out.ln();
        ctxt.out.writeln_comment(comment);
    }

    print_variant(ctxt, tag, variant);
    ctxt.out.dec_indent();
}

fn print_variant<'ty>(
    ctxt: &mut Ctxt<'_, 'ty, '_>,
    tag: Tag,
    variant: &'ty Variant,
) {
    match tag {
        Tag::Adjacent { tag, content } => {
            ctxt.out.inc_indent();
            ctxt.out.write_key_and_separator(tag);
            ctxt.out.write(format!(r#""{}""#, variant.id));
            ctxt.out.ln();

            if !matches!(variant.fields, Fields::Unit) {
                ctxt.print_child_name(content, variant.fields.is_table());
                ctxt.print_fields(&variant.fields, None);
                ctxt.out.ln();
            }

            ctxt.out.dec_indent();
        }

        Tag::Internal { tag } => {
            ctxt.out.inc_indent();
            ctxt.out.write_key_and_separator(tag);
            ctxt.out.write(format!(r#""{}""#, variant.id));
            ctxt.out.ln();

            match &variant.fields {
                Fields::Named { .. } => {
                    ctxt.nested()
                        .with_flat()
                        .print_fields(&variant.fields, None);

                    ctxt.out.ln();
                }
                Fields::Unnamed { .. } => {
                    panic!("Internally tagged unnamed variants are unsupported in TOML")
                }
                Fields::Unit => {}
            }

            ctxt.out.dec_indent();
        }

        Tag::External => match variant.fields {
            Fields::Named { .. } | Fields::Unnamed { .. } => {
                ctxt.out.inc_indent();

                ctxt.print_child_name(variant.id, variant.fields.is_table());
                ctxt.print_fields(&variant.fields, None);
                ctxt.out.ln();

                ctxt.out.dec_indent();
            }

            Fields::Unit => {
                ctxt.out.write(format!(r#""{}""#, variant.id));
            }
        },

        Tag::None => match variant.fields {
            Fields::Named { .. } | Fields::Unnamed { .. } => {
                if !variant.fields.is_table() {
                    panic!("Untagged scalar variants are unsupported in TOML")
                }
                ctxt.out.inc_indent();
                ctxt.print_fields(&variant.fields, None);
                ctxt.out.ln();
                ctxt.out.dec_indent();
            }

            Fields::Unit => {
                panic!("Untagged unit variants are unsupported in TOML")
            }
        },
    }
}
