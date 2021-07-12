use super::*;

impl<'ty> Ctxt<'ty, '_> {
    pub fn expand_variants_for_untagged_enum(&mut self, ty: &'ty Type) -> bool {
        let fields = if let TypeDef::Struct {
            fields: Fields::Named { fields },
            ..
        } = &ty.def
        {
            fields
        } else {
            return false;
        };

        let tags: Vec<_> = fields.iter().filter(|(_, field)| field.ty.tag.is_some()).collect();

        if tags.is_empty() {
            return false;
        }

        // When a type contains more than one untagged field, getting all the
        // combinations right becomes a troublesome journey.
        //
        // An example of such type is:
        //
        // ```rust,ignore
        // struct EventAggregate {
        //   first_type: String,
        //   first_payload: EventPayload,
        //
        //   second_type: String,
        //   second_payload: EventPayload,
        // }
        // ```
        //
        // To print docs for this little guy, we'd have to perform a Cartesian
        // product on `EventPayload x EventPayload` (to list _all_ the possible
        // combinations) - which is doable, although pointless, because such
        // types are basically non-existent in practice.
        if tags.len() > 1 {
            unimplemented!("Struct contains more than one tag for an untagged enum");
        }

        let tagf = tags[0];

        // The "payload" field must be an enum - we can't enforce it through our
        // type system though, so we panic when someone messes this up.
        //
        // As always, this is not really expected to happen in practice, because
        // Serde issues its own error message for this case before us.
        let (tag, variants) = if let TypeDef::Enum { tag, variants } = &tagf.1.ty.def {
            (*tag, variants)
        } else {
            panic!(
                "Invalid type definition: Since field `{}` models a tag for an untagged enum, it must be an enum",
                tagf.0
            );
        };

        // Similarly to the condition above, this - as well - is just a sanity
        // check
        if tag != Tag::None {
            panic!(
                "Invalid type definition: Since field `{}` models a tag for an untagged enum, it must be an untagged \
                 enum",
                tagf.0
            );
        }

        for (variant_idx, variant) in variants.iter().enumerate() {
            if variant_idx > 0 {
                self.out.line(",");
            }

            self.print_named_fields(&fields, Some(variant));
        }

        self.out.newline();

        true
    }
}
