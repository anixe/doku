use super::*;

pub(super) fn comment(
    ctxt: &mut Ctxt<'_, '_, '_>,
    tag: Tag,
    variants: &[&Variant],
) {
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
    let rendered_variant = render_variant(ctxt, tag, variant);

    let rendered_variant = rendered_variant
        .lines()
        .enumerate()
        .map(|(line_idx, line)| {
            if line_idx > 0 {
                format!("  {}", line)
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut str = format!("- {}", rendered_variant);

    // ---

    let comment = if let Some(comment) = variant.comment {
        Some(comment)
    } else if variant.title != variant.id {
        Some(variant.title)
    } else {
        None
    };

    if let Some(comment) = comment {
        if str.lines().count() == 1 && comment.lines().count() == 1 {
            swrite!(str, " = {}", comment);
        } else {
            for (comment_line_idx, comment_line) in
                comment.split('\n').enumerate()
            {
                if comment_line_idx == 0 {
                    swrite!(str, "\n  = {}", comment_line);
                } else {
                    swrite!(str, "\n    {}", comment_line);
                }
            }
        }
    }

    // ---

    ctxt.out.writeln_comment(str);
}

fn render_variant(
    ctxt: &Ctxt<'_, '_, '_>,
    tag: Tag,
    variant: &Variant,
) -> String {
    let comma = if ctxt.fmt.objects_style.use_comma_as_separator {
        ","
    } else {
        ""
    };
    let quote = if ctxt.fmt.objects_style.surround_keys_with_quotes {
        "\""
    } else {
        ""
    };

    match tag {
        Tag::Adjacent { tag, content } => match &variant.fields {
            Fields::Unit => {
                format!(
                    "{{\n\t{q}{}{q}: \"{}\"\n}}",
                    tag,
                    variant.id,
                    q = quote
                )
            }

            fields => format!(
                "{{\n\t{q}{}{q}: \"{}\"{c}\n\t{q}{}{q}: {}\n}}",
                tag,
                variant.id,
                content,
                render_variant_fields(ctxt, fields, false, true),
                c = comma,
                q = quote
            ),
        },

        Tag::Internal { tag } => {
            if let Fields::Named { fields } = &variant.fields {
                if fields.is_empty() {
                    format!(
                        "{{\n\t{q}{}{q}: \"{}\"\n}}",
                        tag,
                        variant.id,
                        q = quote
                    )
                } else {
                    format!(
                        "{{\n\t{q}{}{q}: \"{}\"{c}\n\t{}\n}}",
                        tag,
                        variant.id,
                        render_variant_fields(
                            ctxt,
                            &variant.fields,
                            true,
                            true,
                        ),
                        c = comma,
                        q = quote
                    )
                }
            } else {
                format!(
                    "{{\n\t{q}{}{q}: \"{}\"\n}}",
                    tag,
                    variant.id,
                    q = quote
                )
            }
        }

        Tag::External => match &variant.fields {
            Fields::Unit => format!("\"{}\"", variant.id),

            fields => format!(
                "{{\n\t{q}{}{q}: {}\n}}",
                variant.id,
                render_variant_fields(ctxt, fields, false, true),
                q = quote
            ),
        },

        Tag::None => render_variant_fields(ctxt, &variant.fields, false, false),
    }
}

fn render_variant_fields(
    ctxt: &Ctxt<'_, '_, '_>,
    fields: &Fields,
    flat: bool,
    indent: bool,
) -> String {
    let fmt = Formatting {
        enums_style: EnumsStyle::Separated,
        layout: Layout::OneColumn,
        ..ctxt.fmt.clone()
    };

    let mut out = Output::new(&fmt);

    let mut ctxt = Ctxt {
        // We want this ad-hoc context to be independent from our real one,
        // because we don't want to carry e.g. examples from parent into it.
        //
        // Also, the type-kind here doesn't have to be necessarily `()` - it can
        // be anything, since we're not doing `variant_ctxt.print()`, but rather
        // we're calling `print_fields()` directly, so this `ty` is not read
        // anyway.
        ty: &<()>::ty(),

        val: Default::default(),
        vis: ctxt.vis,
        fmt: &fmt,
        out: &mut out,
        is_key: Default::default(),
        parent: Default::default(),
        example: Default::default(),
        flat,
        depth: Default::default(),
    };

    ctxt.print_fields(fields, None);

    let out = out.render();

    if indent {
        out.lines()
            .enumerate()
            .map(|(line_idx, line)| {
                if line_idx > 0 {
                    format!("  {}", line)
                } else {
                    line.to_owned()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        out
    }
}
