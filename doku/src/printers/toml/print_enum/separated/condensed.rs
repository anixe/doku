use super::*;

pub(super) fn print(
    ctxt: &mut Ctxt<'_, '_, '_>,
    tag: Tag,
    variants: &[&Variant],
) -> bool {
    if variants.iter().any(|variant| variant.comment.is_some()) {
        return false;
    }

    if variants
        .iter()
        .any(|variant| !matches!(variant.fields, Fields::Unit))
    {
        return false;
    }

    match tag {
        Tag::Adjacent { tag, .. } | Tag::Internal { tag } => {
            ctxt.out.write_key_and_separator(tag);
        }

        Tag::External => {
            //
        }

        Tag::None => {
            return false;
        }
    }

    for (variant_idx, variant) in variants.iter().enumerate() {
        if variant_idx > 0 {
            ctxt.out.write(" | ");
        }

        ctxt.out.write(format!(r#""{}""#, variant.id));
    }

    true
}
