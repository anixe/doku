use super::*;
use std::fmt::Write;

impl<'ty> Ctxt<'ty, '_> {
    pub fn print_comment_for_enum(&mut self, parent_comment: String, tag: Tag, variants: &'ty [Variant]) {
        let variants: Vec<_> = variants
            .iter()
            .filter(|variant| self.mode.allows(variant.serializable, variant.deserializable))
            .collect();

        if parent_comment.is_empty() {
            self.out.comment("// Possible variants:");
        } else {
            self.out.comment(format!("// {}; possible variants:", parent_comment));
        }

        for variant in variants {
            self.out.comment(comment_enum_variant(self, tag, variant));
        }
    }
}

fn comment_enum_variant(ctxt: &Ctxt, tag: Tag, variant: &Variant) -> String {
    let mut result = format!("// - {}", print_variant_layout(ctxt, tag, variant));

    // Let's consider following enums:
    //
    // ```rust
    // enum SimpleEnum {
    //     Foo,
    //     Bar,
    // }
    // ```
    //
    // ```rust
    // enum MotleyEnum {
    //     #[serde(rename = "a")]
    //     MinimumAge(usize),
    //
    //     /// Booking period
    //     #[serde(rename = "b")]
    //     BookingPeriod(String),
    // }
    // ```
    //
    // While the first enum is mostly self-explanatory, for the second enum we
    // want to generate a documentation that explains what `a` and `b` stand
    // for:
    //
    // ```ignore
    // { ... } // Possible variants:
    //         // - { "a": 123 } = MinimumAge
    //         // - { "b": "string" } = Booking period
    // ```
    //
    // So:
    //
    // - when user has provided a comment, we print the comment,
    //
    // - when user hasn't provided the comment, and the variant's been renamed, we
    //   print the original variant's identifier (this is rather suboptimal, as it
    //   leaks the original semi-private identifier into the documentation, but it's
    //   better than letting users figure out what e.g. `a` stands for.)
    if let Some(comment) = variant.comment {
        write!(result, " = {}", comment).unwrap();
    } else if variant.title != variant.id {
        write!(result, " = {}", variant.title).unwrap();
    }

    result
}

/// Returns a string that describes structure of given variant.
///
/// For instance, given this documentation:
///
/// ```ignore
/// { ... }  // Possible variants:
///          // - { "a": "string" } = Some comment
///               ^^^^^^^^^^^^^^^^^
///           this is variant's layout
/// ```
fn print_variant_layout(ctxt: &Ctxt, tag: Tag, variant: &Variant) -> String {
    match tag {
        Tag::Adjacent { tag, content } => {
            print_variant_layout::for_adjacently_tagged_enum(ctxt, tag, content, &variant)
        }

        Tag::Internal { tag } => print_variant_layout::for_internally_tagged_enum(ctxt, tag, &variant),
        Tag::External => print_variant_layout::for_externally_tagged_enum(ctxt, &variant),
        Tag::None => print_variant_layout::for_untagged_enum(ctxt, &variant),
    }
}

mod print_variant_layout {
    use super::*;

    pub fn for_adjacently_tagged_enum(ctxt: &Ctxt, tag: &str, content: &str, variant: &Variant) -> String {
        match &variant.fields {
            Fields::Unit => format!(r#"{{ "{}": "{}" }}"#, tag, variant.id),

            fields => format!(
                r#"{{ "{}": "{}", "{}": {} }}"#,
                tag,
                variant.id,
                content,
                inline_print_fields(ctxt, fields)
            ),
        }
    }

    pub fn for_externally_tagged_enum(ctxt: &Ctxt, variant: &Variant) -> String {
        match &variant.fields {
            Fields::Unit => format!(r#""{}""#, variant.id),

            fields => format!("{{ \"{}\": {} }}", variant.id, inline_print_fields(ctxt, fields)),
        }
    }

    pub fn for_internally_tagged_enum(ctxt: &Ctxt, tag: &str, variant: &Variant) -> String {
        if let Fields::Named { .. } = &variant.fields {
            let mut fields = inline_print_fields(ctxt, &variant.fields);

            // `inline_print_fields()` returns a string containing braces, e.g.:
            // `{ "foo": true }`.
            //
            // Since here we print braces manually anyway (see below), we've got
            // to remove them before using this string.
            fields.remove(0);
            fields.remove(fields.len() - 1);

            if !fields.is_empty() {
                return format!(r#"{{ "{}": "{}",{}}}"#, tag, variant.id, fields);
            }
        }

        format!(r#"{{ "{}": "{}" }}"#, tag, variant.id)
    }

    pub fn for_untagged_enum(ctxt: &Ctxt, variant: &Variant) -> String {
        inline_print_fields(ctxt, &variant.fields)
    }

    /// Prints given fields in an inline-fashion, so that they fit in one line
    /// (e.g. `{ "PostDeleted": { "id": 1, "user": 2 } }`).
    fn inline_print_fields(ctxt: &Ctxt, fields: &Fields) -> String {
        let mut variant_out = Paragraph::new(0, false);

        let mut variant_ctxt = Ctxt {
            mode:    ctxt.mode,
            out:     &mut variant_out,
            parents: Default::default(),
            flat:    false,
            inline:  true,

            // We want this ad-hoc context to be independent from our real one,
            // because we don't want to carry e.g. examples from parent into it.
            //
            // Also, the type-def here doesn't have to be necessarily `String` -
            // it can be virtually anything, since we're not doing
            // `variant_ctxt.print()`, but calling `print_fields()` directly (so
            // this `ty` is not read anywhere).
            ty: &Type::from_def(TypeDef::String),
        };

        variant_ctxt.print_fields(fields, None);
        variant_out.to_string()
    }
}
