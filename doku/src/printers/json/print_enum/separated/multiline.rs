use super::*;

pub(super) fn print<'ty>(
    ctxt: &mut Ctxt<'_, 'ty, '_>,
    tag: Tag,
    variants: &[&'ty Variant],
) {
    // Whether to additionally indent the variants or not.
    //
    // We want to add some extra indentation when we're during a field expansion
    // so that we print:
    //
    // ```
    // {
    //   "enum":
    //     variant 1
    //     variant 2
    //     variant 3
    // }
    // ```
    //
    // ... instead of:
    //
    // ```
    // {
    //   "enum": variant 1
    //   variant 2
    //   variant 3
    // }
    // ```
    let indent = ctxt.parent.map_or(false, |parent| {
        matches!(parent.kind, TypeKind::Struct { .. }) && !ctxt.flat
    });

    if indent {
        ctxt.out.ln();
        ctxt.out.inc_indent();
    }

    // Whether to additionally separate the variants or not.
    //
    // We want to add some extra spacing when the variants are "somewhat large"
    // so that the `// or` comment clearly separates each particular variant.
    //
    // There is no single best metric here, but in general we prefer:
    //
    // ```
    // {
    //   "enum":
    //     // Specify site using the concrete URL
    //     {
    //       "url": "http://google.com"
    //     }
    //
    //     // or
    //
    //     // Specify site using just its name
    //     {
    //        "site": "google"
    //     }
    // }
    // ```
    //
    // ... over:
    //
    // ```
    // {
    //   "enum":
    //     // Specify site using the concrete URL
    //     {
    //       "url": "http://google.com"
    //     }
    //     // or
    //     // Specify site using just its name
    //     {
    //        "site": "google"
    //     }
    // }
    // ```
    let separate = match &ctxt.fmt.layout {
        Layout::OneColumn => {
            variants.iter().any(|variant| variant.comment.is_some())
        }
        Layout::TwoColumns { .. } => false,
    };

    for (variant_idx, variant) in variants.iter().enumerate() {
        if variant_idx > 0 {
            if separate {
                ctxt.out.ln();
            }

            ctxt.out
                .write(format!("\n{} or\n", ctxt.fmt.comments_style.separator));

            if separate {
                ctxt.out.ln();
            }
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
    if let Tag::Adjacent { .. } | Tag::External { .. } = tag {
        // Adjacent and external variants can't be flattened because we'd try to
        // print something nonsensical like:
        //
        // ```
        // {
        //   "tag": "...",
        //   "content": "foo": "bar"
        // }
        // ```
        //
        // Note that this property can't be specified from the frontend anyway
        // in here - what we're guarding against is us calling `.with_flat()`
        // somewhere before and carrying this flag too deep.
        ctxt.flat = false;
    }

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
