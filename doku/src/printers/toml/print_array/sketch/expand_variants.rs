mod for_adjacently_tagged_enum;
mod for_externally_tagged_enum;
mod for_internally_tagged_enum;
mod for_untagged_enum;

use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn try_expanding_variants(&mut self, ty: &'ty Type) -> bool {
        self.try_expanding_adjacently_tagged_variants(ty)
            || self.try_expanding_externally_tagged_variants(ty)
            || self.try_expanding_internally_tagged_variants(ty)
            || self.try_expanding_untagged_variants(ty)
    }

    /// Starts by collecting the visible variants of an enum, and then
    /// recursively walks through them to find all nested enums
    /// and convert them to new variants with equivalent structs.
    fn collect_variants(&self, variants: &[Variant]) -> Vec<Variant> {
        let variants = variants
            .iter()
            .filter(|variant| {
                self.vis
                    .allows(variant.serializable, variant.deserializable)
            })
            .map(ToOwned::to_owned);

        collect_variants(variants)
    }
}

fn collect_variants(
    variants: impl IntoIterator<Item = Variant>,
) -> Vec<Variant> {
    let mut modified = false;
    let mut new_variants = Vec::new();

    'outer: for variant in variants {
        match &variant.fields {
            Fields::Named { fields } => {
                for (i, (name, field)) in fields.iter().enumerate() {
                    if let Some(structs) = field.ty.enum_to_structs() {
                        for s in structs {
                            let mut new_variant = variant.clone();
                            if let Fields::Named { fields } =
                                &mut new_variant.fields
                            {
                                let mut new_field = field.to_owned();
                                new_field.ty = s;
                                fields[i] = (name, new_field);
                            }
                            new_variants.push(new_variant);
                        }
                        modified = true;
                        continue 'outer;
                    }
                }
            }
            Fields::Unnamed { fields } => {
                for (i, field) in fields.iter().enumerate() {
                    if let Some(structs) = field.ty.enum_to_structs() {
                        for s in structs {
                            let mut new_variant = variant.clone();
                            if let Fields::Unnamed { fields } =
                                &mut new_variant.fields
                            {
                                let mut new_field = field.to_owned();
                                new_field.ty = s;
                                fields[i] = new_field;
                            }
                            new_variants.push(new_variant);
                        }
                        modified = true;
                        continue 'outer;
                    }
                }
            }
            Fields::Unit => {}
        };
        new_variants.push(variant);
    }

    if modified {
        collect_variants(new_variants)
    } else {
        new_variants
    }
}
