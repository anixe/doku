use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn try_expanding_untagged_variants(
        &mut self,
        ty: &'ty Type,
    ) -> bool {
        let fields_obj = if let TypeKind::Struct { fields, .. } = &ty.kind {
            fields
        } else {
            return false;
        };

        let fields = if let Fields::Named { fields } = fields_obj {
            fields
        } else {
            return false;
        };

        let tags: Vec<_> = fields
            .iter()
            .filter(|(_, field)| field.ty.tag.is_some())
            .collect();

        if tags.is_empty() {
            return false;
        }

        // When a type contains more than one untagged field, getting all the
        // combinations right becomes a troublesome journey.
        //
        // An example of such type is:
        //
        // ```ignore
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
            unimplemented!(
                "Struct contains more than one tag for an untagged enum"
            );
        }

        let tagf = tags[0];

        // The "payload" field must be an enum - we can't enforce it through our
        // type system though, so we panic when someone messes this up.
        //
        // As always, this is not really expected to happen in practice, because
        // Serde issues its own error message for this case before us.
        let (tag, variants) =
            if let TypeKind::Enum { tag, variants } = &tagf.1.ty.kind {
                (*tag, variants)
            } else {
                panic!(
                    "Invalid type definition: Since field `{}` models a tag \
                     for an untagged enum, it must be an enum",
                    tagf.0
                );
            };

        // Similarly to the condition above, this - as well - is just a sanity
        // check
        if tag != Tag::None {
            panic!(
                "Invalid type definition: Since field `{}` models a tag for \
                 an untagged enum, it must be an untagged enum",
                tagf.0
            );
        }

        for variant in self.collect_variants(variants) {
            if fields_obj.should_write_table_name() {
                self.print_current_name();
            }

            self.print_fields(fields_obj, Some(&variant), false);
            self.out.ln();
        }

        self.out.ln();
        true
    }
}
