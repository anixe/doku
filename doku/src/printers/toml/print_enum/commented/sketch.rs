use super::*;

pub(super) fn sketch(
    ctxt: &mut Ctxt<'_, '_, '_>,
    tag: Tag,
    variants: &[&Variant],
) {
    if let Some(example) = ctxt.first_example() {
        ctxt.out.write(example);
        return;
    }

    let variant = if let Some(variant) = variants.first() {
        variant
    } else {
        ctxt.out.write("null");
        return;
    };

    sketch_variant(ctxt, tag, variant);
}

fn sketch_variant(ctxt: &mut Ctxt<'_, '_, '_>, tag: Tag, variant: &Variant) {
    match tag {
        Tag::Adjacent { tag, content } => {
            ctxt.out.write_key_and_separator(tag);
            ctxt.out.writeln(format!(r#""{}""#, variant.id));

            if !matches!(variant.fields, Fields::Unit) {
                ctxt.print_child_name(content, variant.fields.is_table());
                ctxt.out.write("...");
            }
        }

        Tag::Internal { tag } => {
            ctxt.out.write_key_and_separator(tag);
            ctxt.out.writeln(format!(r#""{}""#, variant.id));

            if !matches!(variant.fields, Fields::Unit) {
                ctxt.out.writeln("...");
            }
        }

        Tag::External => match &variant.fields {
            Fields::Unit => ctxt.out.write(format!(r#""{}""#, variant.id)),
            fields => {
                ctxt.print_child_name(variant.id, fields.is_table());
                ctxt.out.write("...");
            }
        },

        Tag::None => match variant.fields {
            Fields::Named { .. } | Fields::Unnamed { .. } => {
                ctxt.out.write("...");
            }

            Fields::Unit => {
                panic!("Untagged unit enums are unsupported in TOML")
            }
        },
    }
}
