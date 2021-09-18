use super::*;

pub(super) fn comment(
    ctxt: &mut Ctxt<'_, '_, '_>,
    tag: Tag,
    variants: &[&Variant],
) {
    if ctxt.inline {
        return;
    }

    ctxt.out.append_comment(|comment| {
        if comment.is_empty() {
            swrite!(comment, "Possible variants:");
        } else {
            swrite!(comment, "; possible variants:");
        }
    });

    for variant in variants {
        comment_variant(ctxt, tag, variant);
    }
}

fn comment_variant(ctxt: &mut Ctxt<'_, '_, '_>, tag: Tag, variant: &Variant) {
    let mut str = format!("- {}", comment_variant_inner(ctxt, tag, variant));

    if let Some(comment) = variant.comment {
        for (comment_idx, comment) in comment.split('\n').enumerate() {
            if comment_idx == 0 {
                swrite!(str, " = {}", comment);
            } else {
                swrite!(str, "\n   {}", comment);
            }
        }
    } else if variant.title != variant.id {
        swrite!(str, " = {}", variant.title);
    }

    ctxt.out.writeln_comment(str);
}

fn comment_variant_inner(
    ctxt: &Ctxt<'_, '_, '_>,
    tag: Tag,
    variant: &Variant,
) -> String {
    match tag {
        Tag::Adjacent { tag, content } => match &variant.fields {
            Fields::Unit => format!(r#"{{ "{}": "{}" }}"#, tag, variant.id),

            fields => format!(
                r#"{{ "{}": "{}", "{}": {} }}"#,
                tag,
                variant.id,
                content,
                render_fields_inline(ctxt, fields, false)
            ),
        },

        Tag::Internal { tag } => {
            if let Fields::Named { fields } = &variant.fields {
                if fields.is_empty() {
                    format!(r#"{{ "{}": "{}" }}"#, tag, variant.id)
                } else {
                    let fields =
                        render_fields_inline(ctxt, &variant.fields, true);

                    format!(r#"{{ "{}": "{}", {} }}"#, tag, variant.id, fields)
                }
            } else {
                format!(r#"{{ "{}": "{}" }}"#, tag, variant.id)
            }
        }

        Tag::External => match &variant.fields {
            Fields::Unit => format!(r#""{}""#, variant.id),

            fields => format!(
                "{{ \"{}\": {} }}",
                variant.id,
                render_fields_inline(ctxt, fields, false)
            ),
        },

        Tag::None => render_fields_inline(ctxt, &variant.fields, false),
    }
}

/// Prints given fields in an inline-fashion, so that they fit in one line
/// (e.g. `{ "PostDeleted": { "id": 1, "user": 2 } }`).
fn render_fields_inline(
    ctxt: &Ctxt<'_, '_, '_>,
    fields: &Fields,
    flat: bool,
) -> String {
    let fmt = Formatting {
        auto_comments: AutoComments::none(),
        doc_comments: DocComments::Hidden,
        indent_style: IndentStyle { size: 0 },
        ..Default::default()
    };

    let mut out = Output::new(&fmt);

    let mut ctxt = Ctxt {
        // We want this ad-hoc context to be independent from our real one,
        // because we don't want to carry e.g. examples from parent into
        // it.
        //
        // Also, the type-kind here doesn't have to be necessarily `String`
        // - it can be anything, since we're not doing `variant_ctxt.print()`,
        // but rather we're calling `print_fields()` directly, so this
        // `ty` is not read anywhere.
        ty: &<String as Document>::ty(),

        val: None,
        vis: ctxt.vis,
        fmt: &fmt,
        out: &mut out,
        parents: Default::default(),
        flat,
        inline: true,
    };

    ctxt.print_fields(fields, None);
    out.render()
}
