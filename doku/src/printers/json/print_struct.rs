use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn print_struct(
        &mut self,
        fields: &'ty Fields,
        transparent: bool,
        variant: Option<&'ty Variant>,
    ) {
        if transparent {
            self.print_transparent_struct(fields);
        } else {
            self.print_fields(fields, variant);
        }
    }

    fn print_transparent_struct(&mut self, fields: &'ty Fields) {
        let fields: Vec<_> = match fields {
            Fields::Named { fields } => {
                fields.iter().map(|(_, field)| field).collect()
            }
            Fields::Unnamed { fields } => fields.iter().collect(),
            Fields::Unit => Default::default(),
        };

        // Serde already covers this case for us, so hopefully this will
        // never be triggered
        if fields.len() != 1 {
            panic!("Found a struct with invalid number of transparent fields");
        }

        let example = self.example();

        self.nested()
            .with_ty(&fields[0].ty)
            .with_example(example)
            .print();
    }
}
