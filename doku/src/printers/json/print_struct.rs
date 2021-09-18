use super::*;

impl<'ty> Ctxt<'ty, '_> {
    pub fn print_struct(&mut self, fields: &'ty ty::Fields, transparent: bool, variant: Option<&'ty ty::Variant>) {
        if transparent {
            let fields: Vec<_> = match fields {
                ty::Fields::Named { fields } => fields.iter().map(|(_, field)| field).collect(),
                ty::Fields::Unnamed { fields } => fields.iter().collect(),
                ty::Fields::Unit => Default::default(),
            };

            // Serde already covers this case for us, so hopefully this will
            // never be triggered
            if fields.len() != 1 {
                panic!("Found a struct with invalid number of transparent fields");
            }

            self.with_ty(&fields[0].ty).print();
        } else {
            self.print_fields(fields, variant);
        }
    }
}
