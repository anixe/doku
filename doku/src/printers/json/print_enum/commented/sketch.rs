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
            ctxt.out.write("{ ");
            ctxt.out.write_key_and_separator(tag);
            ctxt.out.write(format!(r#""{}""#, variant.id));

            if let Fields::Named { .. } | Fields::Unnamed { .. } =
                variant.fields
            {
                ctxt.out.write(", ");
                ctxt.out.write_key_and_separator(content);
                ctxt.out.write("...");
            }

            ctxt.out.write(" }");
        }

        Tag::Internal { tag } => {
            ctxt.out.write("{ ");
            ctxt.out.write_key_and_separator(tag);
            ctxt.out.write(format!(r#""{}""#, variant.id));

            if let Fields::Named { .. } | Fields::Unnamed { .. } =
                variant.fields
            {
                ctxt.out.write(", ...");
            }

            ctxt.out.write(" }");
        }

        Tag::External => match variant.fields {
            Fields::Named { .. } | Fields::Unnamed { .. } => {
                ctxt.out.write("{ ");
                ctxt.out.write_key_and_separator(variant.id);
                ctxt.out.write("... }")
            }

            Fields::Unit => {
                ctxt.out.write(format!(r#""{}""#, variant.id));
            }
        },

        Tag::None => match variant.fields {
            Fields::Named { .. } | Fields::Unnamed { .. } => {
                ctxt.out.write("{ ... }");
            }

            Fields::Unit => {
                ctxt.out.write("null");
            }
        },
    }
}
